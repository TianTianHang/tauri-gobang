# Fix Room Database Inconsistency

## Problem Summary

**Critical Issue**: Players create a room in "waiting" state, exit, but the room remains in the database. Other players can see it in the room list but get "room not found" when trying to join.

**Root Cause**: Room deletion in memory does not sync to database in multiple code paths.

## Impact Analysis

### User-Facing Symptoms

1. **Waiting room exit bug** (CRITICAL - Reported)
   - User creates room → DB: waiting, Memory: Waiting
   - Host exits → Memory deleted, DB still shows "waiting"
   - Room list shows this orphaned room
   - Others try to join → "Room not found" error

2. **Server restart orphan rooms** (HIGH - Discovered during exploration)
   - Server has 3 waiting rooms in DB and memory
   - Server crashes/restarts → Memory cleared
   - DB still has 3 waiting rooms
   - Users see these in room list but can't join

3. **Timeout edge cases** (MEDIUM - Rare)
   - State inconsistency during reconnection
   - Unable to determine winner
   - Memory deleted, DB record remains

### Affected Code Paths

| Location | Scenario | Has DB Sync? |
|----------|----------|--------------|
| `ws.rs:371` | Player exits waiting room | ❌ **MISSING** |
| `ws.rs:404` | Second disconnect in playing room | ✓ Present |
| `ws.rs:448` | Timeout: inconsistent state | ❌ **MISSING** |
| `ws.rs:460` | Timeout: no winner determinable | ❌ **MISSING** |
| `ws.rs:495` | Timeout: normal end | ✓ Present |

## Current Architecture

```
Room List Query → Database (WHERE status = 'waiting')
Room Validation → Memory HashMap
Room Creation    → Both DB and Memory
Room Deletion    → Mostly Memory only ❌
```

**Design Flaw**: Hybrid approach without consistency guarantees.

## Solution Strategy (Option C)

**Fix in-place without architecture changes.**

### Phase 1: Critical Fixes (Blocker)

1. **ws.rs:371** - Add DB sync when deleting empty waiting rooms
2. **ws.rs:448** - Add DB sync for timeout state cleanup
3. **ws.rs:460** - Add DB sync for timeout winner resolution failure

### Phase 2: Robustness Improvements (Important)

4. **main.rs** - Clean orphaned waiting rooms on startup
5. **Optional** - Periodic cleanup job for stale rooms

### Phase 3: Monitoring (Nice to have)

6. Add metrics for room lifecycle
7. Alert on DB/Memory room count mismatch

## Detailed Fixes

### Fix 1: Waiting Room Exit (ws.rs:363-373)

```rust
// BEFORE
if room.status != RoomStatus::Playing {
    room.remove_player(&user_id);
    if room.is_empty() {
        rooms.remove(&room_id);  // ❌ Only memory
    }
    return;
}

// AFTER
if room.status != RoomStatus::Playing {
    tracing::warn!(
        room_id = %room_id,
        user_id = %user_id,
        "player disconnected from non-playing room"
    );
    room.remove_player(&user_id);
    if room.is_empty() {
        // ✓ Sync DB before memory deletion
        if let Err(e) = db::update_room_status(&state.db, &room_id, "ended").await {
            tracing::error!(error = %e, "failed to update room status");
        }
        rooms.remove(&room_id);
    }
    return;
}
```

### Fix 2 & 3: Timeout Edge Cases (ws.rs:446-462)

```rust
// Location 1: Inconsistent state
if !still_disconnected {
    tracing::warn!(room_id = %room_id_clone, "timeout task: inconsistent state");
    // ✓ Add DB sync
    if let Err(e) = db::update_room_status(&state_clone.db, &room_id_clone, "ended").await {
        tracing::error!(error = %e, "failed to update room status");
    }
    rooms.remove(&room_id_clone);
    return;
}

// Location 2: No winner determinable
if winner_id.is_none() {
    tracing::error!(room_id = %room_id_clone, "timeout: no winner determinable");
    // ✓ Add DB sync
    if let Err(e) = db::update_room_status(&state_clone.db, &room_id_clone, "ended").await {
        tracing::error!(error = %e, "failed to update room status");
    }
    rooms.remove(&room_id_clone);
    return;
}
```

### Fix 4: Startup Cleanup (main.rs)

```rust
// After db::init_database(&pool).await?
if let Err(e) = cleanup_orphaned_rooms(&pool).await {
    tracing::warn!("Failed to cleanup orphaned rooms: {}", e);
}

// New function:
async fn cleanup_orphaned_rooms(pool: &SqlitePool) -> Result<()> {
    // Mark waiting rooms older than 1 hour as ended
    let cutoff = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as i64 - 3600;

    let updated = sqlx::query(
        "UPDATE rooms SET status = 'ended', ended_at = ? WHERE status = 'waiting' AND created_at < ?"
    )
    .bind(cutoff)
    .bind(cutoff)
    .execute(pool)
    .await?
    .rows_affected();

    if updated > 0 {
        tracing::info!(cleaned = %updated, "cleaned up orphaned waiting rooms");
    }

    Ok(())
}
```

## Testing Plan

### Manual Test Cases

1. **Test waiting room exit**
   - Create room as host
   - Click cancel immediately
   - Refresh room list → Should not show the room

2. **Test server restart**
   - Create 2-3 waiting rooms
   - Restart server
   - Check room list → Should not show orphan rooms

3. **Test timeout scenarios**
   - Start game as Player 1
   - Disconnect during game
   - Wait 30s timeout
   - Check room list → Should be cleaned

### Integration Test

```rust
#[tokio::test]
async fn test_waiting_room_exit_syncs_db() {
    // Create room in waiting state
    // Simulate host disconnect
    // Verify DB status is 'ended'
    // Verify room list doesn't show it
}
```

## Risks & Considerations

### Low Risk
- Changes are localized to 3 code paths
- Follows existing pattern (ws.rs:401, ws.rs:478)
- DB operation failure is logged but doesn't crash

### Considerations
- **Startup cleanup**: 1 hour threshold prevents accidental cleanup of recently created rooms during short restarts
- **DB error handling**: Already logs errors; room is still removed from memory to prevent leaks
- **Performance**: Minimal impact (one extra UPDATE per room deletion)

## Rollout Plan

1. Deploy fixes to dev environment
2. Run manual test cases
3. Add integration tests
4. Deploy to production
5. Monitor logs for "failed to update room status" errors
6. Run one-time DB cleanup script for existing orphan rooms

## Future Considerations

This is a stability fix. Long-term improvements could include:
- Migrate to DB-first architecture (Option A)
- Or memory-first with lightweight room list (Option B)
- Add room TTL for automatic cleanup
- Implement distributed locking for multi-server deployments
