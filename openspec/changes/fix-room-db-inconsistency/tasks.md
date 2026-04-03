# Tasks: Room Database Consistency Fixes

## Phase 1: Critical Fixes (Blocker)

### Task 1.1: Fix Waiting Room Exit
**Location**: `server/src/ws.rs:363-373`
**Priority**: CRITICAL
**Estimated**: 15 minutes

- [x] Add `db::update_room_status()` call before `rooms.remove()`
- [x] Add error logging with room_id context
- [x] Add info log for successful cleanup
- [ ] Test: Create room, exit immediately, check room list

**Acceptance**:
```rust
if room.is_empty() {
    if let Err(e) = db::update_room_status(&state.db, &room_id, "ended").await {
        tracing::error!(error = %e, room_id = %room_id,
            "failed to update room status to ended");
    }
    rooms.remove(&room_id);
}
```

### Task 1.2: Fix Timeout Inconsistent State
**Location**: `server/src/ws.rs:446-450`
**Priority**: HIGH
**Estimated**: 10 minutes

- [x] Add `db::update_room_status()` before `rooms.remove()`
- [x] Add warn-level logging for inconsistent state
- [x] Add error logging for DB failures
- [ ] Test: Simulate race condition in timeout task

**Acceptance**:
```rust
if !still_disconnected {
    tracing::warn!(room_id = %room_id_clone,
        "timeout task: inconsistent room state, cleaning up");
    if let Err(e) = db::update_room_status(&state_clone.db, &room_id_clone, "ended").await {
        tracing::error!(error = %e, room_id = %room_id_clone,
            "failed to update room status during inconsistent state cleanup");
    }
    rooms.remove(&room_id_clone);
    return;
}
```

### Task 1.3: Fix Timeout No Winner
**Location**: `server/src/ws.rs:458-462`
**Priority**: HIGH
**Estimated**: 10 minutes

- [x] Add `db::update_room_status()` before `rooms.remove()`
- [x] Add error-level logging for no-winner case
- [x] Add error logging for DB failures
- [ ] Test: Simulate timeout with corrupted room state

**Acceptance**:
```rust
if winner_id.is_none() {
    tracing::error!(room_id = %room_id_clone,
        "timeout task: cannot determine winner, cleaning up");
    if let Err(e) = db::update_room_status(&state_clone.db, &room_id_clone, "ended").await {
        tracing::error!(error = %e, room_id = %room_id_clone,
            "failed to update room status during winner resolution failure");
    }
    rooms.remove(&room_id_clone);
    return;
}
```

## Phase 2: Robustness Improvements

### Task 2.1: Implement Startup Cleanup Function
**Location**: `server/src/main.rs` (new function)
**Priority**: HIGH
**Estimated**: 20 minutes

- [x] Create `cleanup_orphaned_rooms()` function
- [x] Add SQL UPDATE query with 1-hour cutoff
- [x] Add info log for cleaned rooms count
- [x] Handle errors gracefully (log warning, don't crash)
- [x] Add function to module exports

**Acceptance**:
```rust
async fn cleanup_orphaned_rooms(pool: &SqlitePool) -> anyhow::Result<()> {
    let cutoff = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as i64 - 3600;

    let result = sqlx::query(
        "UPDATE rooms SET status = 'ended', ended_at = ?
         WHERE status = 'waiting' AND created_at < ?"
    )
    .bind(cutoff)
    .bind(cutoff)
    .execute(pool)
    .await?;

    if result.rows_affected() > 0 {
        tracing::info!(cleaned_count = result.rows_affected(),
            "cleaned up orphaned waiting rooms");
    }

    Ok(())
}
```

### Task 2.2: Call Startup Cleanup in main()
**Location**: `server/src/main.rs:79` (after `init_database`)
**Priority**: HIGH
**Estimated**: 5 minutes

- [x] Add cleanup call after DB initialization
- [x] Wrap in try/except with warn log
- [ ] Test: Create stale room, restart server, verify cleanup

**Acceptance**:
```rust
db::init_database(&pool).await?;
println!("\x1b[32m\u{2713}\x1b[0m 数据库已连接: {}", db_path);

if let Err(e) = cleanup_orphaned_rooms(&pool).await {
    tracing::warn!("Failed to cleanup orphaned rooms: {}", e);
}
```

## Phase 3: Testing

### Task 3.1: Manual Test - Waiting Room Exit
**Priority**: MEDIUM
**Estimated**: 10 minutes

Steps:
- [ ] Start server
- [ ] Create room as user A
- [ ] Click "取消并返回大厅" immediately
- [ ] Refresh room list as user B
- [ ] Verify: Room should NOT appear

**Expected Result**: Room list doesn't show orphaned room

### Task 3.2: Manual Test - Server Restart
**Priority**: MEDIUM
**Estimated**: 15 minutes

Steps:
- [ ] Start server
- [ ] Create 2-3 waiting rooms
- [ ] Stop server (Ctrl+C)
- [ ] Start server again
- [ ] Check room list
- [ ] Verify: Old waiting rooms should not appear
- [ ] Check logs: Should see "cleaned up orphaned waiting rooms"

**Expected Result**: Startup cleanup removes orphaned DB records

### Task 3.3: Manual Test - Timeout Scenarios
**Priority**: LOW
**Estimated**: 20 minutes

Steps:
- [ ] Create game with 2 players
- [ ] Player 1 disconnects
- [ ] Wait for timeout message on Player 2
- [ ] Wait 30 seconds for timeout
- [ ] Check room list
- [ ] Verify: Room should not appear

**Expected Result**: Timeout cleanup removes room from DB

### Task 3.4: Integration Tests
**Priority**: LOW
**Estimated**: 1 hour

- [x] Add `test_waiting_room_exit_updates_db()`
- [x] Add `test_orphan_room_cleanup()`
- [ ] Add `test_timeout_inconsistent_state_cleanup()`
- [x] Run `cargo test` in server directory
- [x] All tests pass

See `design.md` for test implementation details.

## Phase 4: Verification

### Task 4.1: Code Review Checklist
**Priority**: MEDIUM
**Estimated**: 15 minutes

- [x] All 3 DB sync fixes follow existing pattern
- [x] Error logging is consistent
- [x] No new dependencies added
- [x] Code compiles without warnings (`cargo clippy`)
- [x] No breaking changes to public API
- [x] Migration SQL not needed (uses existing function)

### Task 4.2: Log Output Verification
**Priority**: LOW
**Estimated**: 10 minutes

Check server logs for correct messages:

**Expected logs on waiting room exit**:
```
WARN player disconnected from non-playing room
INFO empty waiting room removed
DEBUG room status updated (from db.rs)
```

**Expected logs on startup with orphans**:
```
INFO cleaned up orphaned waiting rooms
  cleaned_count=3
```

**Expected logs on DB sync failure**:
```
ERROR failed to update room status to ended
  room_id=xxx
  error=...
```

### Task 4.3: One-Time DB Cleanup
**Priority**: LOW
**Estimated**: 5 minutes

For production environments with existing orphan rooms:

```sql
-- Run once to clean existing orphaned waiting rooms
UPDATE rooms
SET status = 'ended',
    ended_at = strftime('%s', 'now')
WHERE status = 'waiting'
  AND created_at < strftime('%s', 'now', '-1 hour');
```

- [ ] Backup database before running
- [ ] Execute cleanup SQL
- [ ] Verify row count affected
- [ ] Check room list no longer shows orphans

## Phase 5: Deployment (Optional)

### Task 5.1: Staging Deployment
**Priority**: OPTIONAL
**Estimated**: 30 minutes

- [ ] Deploy to staging environment
- [ ] Run manual test suite
- [ ] Monitor logs for errors
- [ ] Check room list functionality
- [ ] Load test with multiple rooms

### Task 5.2: Production Deployment
**Priority**: OPTIONAL
**Estimated**: 1 hour

- [ ] Schedule maintenance window
- [ ] Backup production database
- [ ] Deploy code changes
- [ ] Run one-time cleanup SQL
- [ ] Monitor logs for 24 hours
- [ ] Check for "failed to update room status" errors
- [ ] Verify room list works correctly

### Task 5.3: Post-Deployment Monitoring
**Priority**: OPTIONAL
**Duration**: 1 week

Monitor for:
- [ ] No increase in DB errors
- [ ] Room list works correctly
- [ ] No orphan rooms accumulating
- [ ] Users can join games normally
- [ ] Timeout handling works

## Summary

**Total Estimated Time**:
- Phase 1: 35 minutes (critical fixes)
- Phase 2: 25 minutes (robustness)
- Phase 3: 45 minutes (testing)
- Phase 4: 30 minutes (verification)
- **Total**: ~2.5 hours (without deployment)
- **With deployment**: ~4 hours

**Risk Level**: LOW
- Localized changes
- Follows existing patterns
- Error handling is defensive
- Rollback is straightforward

**Dependencies**:
- None (all independent tasks)

**Definition of Done**:
- ✅ All 3 critical fixes implemented
- ✅ Startup cleanup implemented
- ✅ Manual tests pass
- ✅ No compilation warnings
- ✅ Logs verified
