# Design: Room Database Consistency Fixes

## Overview

Fix data inconsistency between in-memory room state and SQLite database when rooms are deleted.

## Architecture Context

### Current Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    Room Lifecycle                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. CREATE                                                  │
│     INSERT INTO rooms (status='waiting')                   │
│     rooms.insert(id, Room::new(...))                       │
│                                                             │
│  2. JOIN (player2)                                         │
│     UPDATE rooms SET status='playing', player2_id=...      │
│     room.add_player() → status=Playing                     │
│                                                             │
│  3. DISCONNECT                                             │
│     Three paths:                                           │
│     a) Waiting state exit ❌ (fixes needed)                │
│     b) Playing state 2nd disconnect ✓                      │
│     c) Timeout resolution ❌ (fixes needed)                │
│                                                             │
│  4. DELETE                                                 │
│     rooms.remove(id)                                       │
│     UPDATE rooms SET status='ended' ← Only in some paths   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Inconsistency Root Causes

1. **Incomplete cleanup**: Only 2 of 5 deletion paths sync to DB
2. **No startup recovery**: Server doesn't clean orphaned DB records
3. **Dual source of truth**: Room list from DB, validation from memory

## Detailed Design

### Component: ws.rs - handle_disconnect()

**Location**: `server/src/ws.rs:354-499`

**Current Code Structure**:
```rust
async fn handle_disconnect(state: &AppState, room_id: String, user_id: String) {
    let mut rooms = state.rooms.write().await;
    let room = rooms.get_mut(&room_id)?;

    // Path A: Non-playing room (waiting, ended)
    if room.status != RoomStatus::Playing {
        room.remove_player(&user_id);
        if room.is_empty() {
            rooms.remove(&room_id);  // ❌ FIX NEEDED
        }
        return;
    }

    // Path B: Playing room, first disconnect
    room.remove_player(&user_id);
    if room.disconnected.is_some() {
        // Second disconnect
        room.status = RoomStatus::Ended;
        db::update_room_status(&state.db, &room_id, "ended").await?;  // ✓ OK
        rooms.remove(&room_id);
        return;
    }

    // Path C: Playing room, first disconnect → start timeout
    room.disconnected = Some((user_id, Instant::now(), opponent_id));
    start_timeout_task(...);  // ❌ FIXES NEEDED INSIDE
}
```

**Fix Locations**:

#### Fix 1: Path A - Waiting Room Cleanup (Line 363-373)

```rust
if room.status != RoomStatus::Playing {
    tracing::warn!(
        room_id = %room_id,
        user_id = %user_id,
        "player disconnected from non-playing room"
    );
    room.remove_player(&user_id);
    if room.is_empty() {
        // ADDED: Sync DB before removing from memory
        if let Err(e) = db::update_room_status(&state.db, &room_id, "ended").await {
            tracing::error!(error = %e, room_id = %room_id,
                "failed to update room status to ended");
        }
        rooms.remove(&room_id);
        tracing::info!(room_id = %room_id, "empty waiting room removed");
    }
    return;
}
```

**Reasoning**:
- Waiting rooms have 1 player (host)
- When host leaves, room becomes empty
- Must update DB before memory cleanup
- Follows pattern from Path B (line 401)

#### Fix 2 & 3: Timeout Task Edge Cases (Line 425-496)

Inside `tokio::spawn` timeout handler:

```rust
tokio::spawn(async move {
    tokio::time::sleep(Duration::from_secs(30)).await;

    let mut rooms = state_clone.rooms.write().await;
    let room = rooms.get_mut(&room_id_clone)?;

    // Edge Case 1: State became consistent (player reconnected)
    let is_reconnect_handled = room.disconnected.is_none();
    if is_reconnect_handled {
        return;  // ✓ OK - no cleanup needed
    }

    // Edge Case 2: Still disconnected but state is wrong
    let still_disconnected = room.disconnected
        .as_ref()
        .map(|(uid, _, _)| uid == &disconnected_user_id)
        .unwrap_or(false);

    if !still_disconnected {
        // FIX 2: Inconsistent state - clean up with DB sync
        tracing::warn!(room_id = %room_id_clone,
            "timeout task: inconsistent room state, cleaning up");

        if let Err(e) = db::update_room_status(&state_clone.db, &room_id_clone, "ended").await {
            tracing::error!(error = %e, room_id = %room_id_clone,
                "failed to update room status during inconsistent state cleanup");
        }

        rooms.remove(&room_id_clone);
        return;
    }

    // Edge Case 3: Can't determine winner
    let winner_id = room.disconnected
        .as_ref()
        .and_then(|(_, _, winner)| winner.clone())
        .or_else(|| room.get_opponent_id(&disconnected_user_id));

    if winner_id.is_none() {
        // FIX 3: No winner - clean up with DB sync
        tracing::error!(room_id = %room_id_clone,
            "timeout task: cannot determine winner, cleaning up");

        if let Err(e) = db::update_room_status(&state_clone.db, &room_id_clone, "ended").await {
            tracing::error!(error = %e, room_id = %room_id_clone,
                "failed to update room status during winner resolution failure");
        }

        rooms.remove(&room_id_clone);
        return;
    }

    // Normal timeout resolution (already has DB sync at line 478)
    room.status = RoomStatus::Ended;
    db::update_room_status(...)?;  // ✓ OK
    insert_game(...)?;
    rooms.remove(&room_id_clone);
});
```

**Reasoning**:
- Edge cases represent data corruption or race conditions
- Safe to clean up and mark as "ended"
- DB sync prevents orphan records
- Error logging for debugging

### Component: main.rs - Startup Cleanup

**Location**: `server/src/main.rs` (after line 79)

**New Function**:

```rust
/// Clean up orphaned room records from previous server runs.
///
/// Marks waiting rooms as ended if they're older than 1 hour.
/// This prevents issues where server restart left DB records but cleared memory.
async fn cleanup_orphaned_rooms(pool: &SqlitePool) -> anyhow::Result<()> {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Rooms older than 1 hour are considered stale
    let cutoff_seconds = 3600;
    let cutoff_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as i64 - cutoff_seconds;

    let result = sqlx::query(
        "UPDATE rooms
         SET status = 'ended',
             ended_at = ?
         WHERE status = 'waiting'
           AND created_at < ?"
    )
    .bind(cutoff_timestamp)
    .bind(cutoff_timestamp)
    .execute(pool)
    .await?;

    if result.rows_affected() > 0 {
        tracing::info!(
            cleaned_count = result.rows_affected(),
            cutoff_hours = cutoff_seconds / 3600,
            "cleaned up orphaned waiting rooms from previous server runs"
        );
    }

    Ok(())
}
```

**Call Site** (in `main()`):

```rust
let pool = db::create_pool(&db_path).await?;
db::init_database(&pool).await?;
println!("\x1b[32m\u{2713}\x1b[0m 数据库已连接: {}", db_path);

// ADDED: Clean up orphaned rooms
if let Err(e) = cleanup_orphaned_rooms(&pool).await {
    tracing::warn!("Failed to cleanup orphaned rooms: {}", e);
}

// ... rest of startup
```

**Design Decisions**:

1. **1-hour threshold**:
   - Short enough to clean up stale rooms
   - Long enough to avoid false positives during maintenance
   - Configurable via constant if needed

2. **Mark as ended instead of DELETE**:
   - Preserves history for debugging
   - Consistent with other room endings
   - Doesn't break foreign key constraints

3. **Best-effort approach**:
   - Logs warning if cleanup fails
   - Doesn't block server startup
   - Orphan rooms will be handled on next run

### Component: db.rs - No Changes Needed

Existing function `update_room_status()` already handles both status transitions:

```rust
pub async fn update_room_status(pool: &SqlitePool, room_id: &str, status: &str) -> Result<()> {
    let ended_at = if status == "ended" {
        Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64)
    } else {
        None
    };

    if let Some(ts) = ended_at {
        sqlx::query("UPDATE rooms SET status = ?, ended_at = ? WHERE id = ?")
            .bind(status)
            .bind(ts)
            .bind(room_id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE rooms SET status = ? WHERE id = ?")
            .bind(status)
            .bind(room_id)
            .execute(pool)
            .await?;
    }

    tracing::debug!(room_id = %room_id, new_status = %status, "room status updated");
    Ok(())
}
```

## Error Handling Strategy

### DB Update Failures

```rust
if let Err(e) = db::update_room_status(...).await {
    tracing::error!(error = %e, room_id = %room_id,
        "failed to update room status to ended");

    // Decision: Still remove from memory to prevent leaks
    rooms.remove(&room_id);
}
```

**Rationale**:
- Memory leak is worse than DB inconsistency
- Orphan DB record will be cleaned on next startup
- Room can't function without in-memory state anyway
- Error is logged for monitoring

### Logging Levels

- `ERROR`: DB update failures
- `WARN`: Inconsistent states, cleanup failures
- `INFO`: Normal room removal, startup cleanup results
- `DEBUG`: Detailed state transitions

## Testing Strategy

### Unit Tests

```rust
#[tokio::test]
async fn test_waiting_room_exit_updates_db() {
    let state = create_test_state().await;

    // Create waiting room
    let room_id = create_room(&state).await;
    assert_eq!(get_db_status(&state.db, &room_id).await, "waiting");

    // Simulate host disconnect
    disconnect_player(&state, &room_id, &host_id).await;

    // Verify DB updated
    assert_eq!(get_db_status(&state.db, &room_id).await, "ended");
    assert!(!room_exists_in_memory(&state, &room_id).await);
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_orphan_room_cleanup() {
    let pool = create_test_db().await;

    // Create stale waiting rooms
    let old_timestamp = SystemTime::now() - Duration::from_secs(7200); // 2 hours ago
    insert_stale_room(&pool, "room-old", old_timestamp).await;
    insert_recent_room(&pool, "room-new").await;

    // Run cleanup
    cleanup_orphaned_rooms(&pool).await.unwrap();

    // Verify
    assert_eq!(get_room_status(&pool, "room-old").await, Some("ended".to_string()));
    assert_eq!(get_room_status(&pool, "room-new").await, Some("waiting".to_string()));
}
```

### Manual Test Scenarios

See `proposal.md` Section: Testing Plan

## Performance Impact

- **DB operations**: +3 UPDATE statements per room deletion (negligible)
- **Startup cleanup**: Single scan of rooms table (fast with index on created_at)
- **Memory**: No changes
- **Latency**: <1ms per additional UPDATE

## Monitoring & Observability

### Metrics to Add (Future)

- `room_deletion_total{path="waiting|playing|timeout"}`
- `room_db_sync_errors_total`
- `orphan_rooms_cleaned_total`

### Log Queries

```bash
# Check for failed DB syncs
grep "failed to update room status" logs/server.log

# Check startup cleanup
grep "cleaned up orphaned" logs/server.log

# Monitor room lifecycle
grep "room status transition\|room removed" logs/server.log
```

## Rollback Plan

If issues arise:

1. **Revert code changes** (git revert)
2. **Clean orphan rooms manually**:
   ```sql
   UPDATE rooms
   SET status = 'ended', ended_at = strftime('%s', 'now')
   WHERE status = 'waiting'
     AND created_at < strftime('%s', 'now', '-1 hour');
   ```
3. **Investigate logs** for DB errors
4. **Re-deploy** after fix

## Future Improvements (Out of Scope)

1. **Room TTL**: Auto-expire waiting rooms after N minutes
2. **Health check**: Periodic DB/memory consistency check
3. **Monitoring**: Prometheus metrics export
4. **Architecture**: Consider DB-first or memory-first (see proposal.md)
