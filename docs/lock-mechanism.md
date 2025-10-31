# Repository Lock Mechanism

Codex implements a cross-platform lock mechanism to prevent multiple CLI instances from conflicting when working on the same repository.

## Overview

When a Codex CLI instance starts working on a repository, it acquires a lock by creating `.codex/lock.json`. This prevents other instances from making concurrent changes that could cause conflicts.

## Lock File Format

The lock file (`.codex/lock.json`) contains:

```json
{
  "version": "0.52.0",
  "pid": 12345,
  "ppid": 12340,
  "uid": 1000,
  "hostname": "my-computer",
  "repo_path": "/path/to/repo",
  "started_at": 1698765432,
  "expires_at": 1698769032
}
```

Fields:
- `version`: Codex version that created the lock
- `pid`: Process ID of the lock holder
- `ppid`: Parent process ID (Unix only)
- `uid`: User ID (Unix only)
- `hostname`: Hostname of the machine holding the lock
- `repo_path`: Absolute path to the repository
- `started_at`: Unix timestamp when lock was acquired
- `expires_at`: Optional TTL expiration timestamp

## Commands

### Check Lock Status

```bash
codex lock status
```

Shows current lock information, including:
- Whether a lock exists
- Who holds the lock
- When it was acquired
- Whether it's stale

### Remove Stale Lock

```bash
codex lock remove
```

Removes a stale lock. A lock is considered stale if:
- The TTL has expired
- The process no longer exists

To force removal without checks:
```bash
codex lock remove --force
```

## Stale Lock Detection

Codex automatically detects stale locks using:

### Process Liveness Check
- **Unix**: Sends signal 0 to check if process exists
- **Windows**: Attempts to open process handle
- If process doesn't exist, lock is stale

### TTL (Time-To-Live)
If a TTL is set, the lock expires after the specified duration.

## Troubleshooting

### "Repository is locked" Error

If you see:
```
Repository is locked by process 12345 on my-computer (started at 1698765432).
Use 'codex unlock --force' to remove stale locks.
```

1. Check if the lock is stale:
   ```bash
   codex lock status
   ```

2. If stale, remove it:
   ```bash
   codex lock remove
   ```

3. If not stale, wait for the other process to complete, or investigate why it's still running.

### Manual Lock Removal

If the CLI fails to remove a stale lock, you can manually delete:
```bash
rm .codex/lock.json
```

## Lock Acquisition Flow

1. Check if `.codex/lock.json` exists
2. If exists, read lock info and check if stale
3. If stale, remove old lock
4. Create new lock file atomically using `O_EXCL` flag
5. Write lock information
6. Continue with operation

## Platform-Specific Notes

### Unix/Linux/Mac
- Uses `O_EXCL` flag for atomic file creation
- Uses signal 0 for process liveness check
- Includes `ppid` and `uid` in lock info

### Windows
- Uses `CREATE_NEW` flag for atomic file creation
- Uses process handle for liveness check
- Does not include `ppid` or `uid`

## Configuration

Lock behavior can be configured via environment variables:

```bash
# Disable lock mechanism (not recommended)
CODEX_DISABLE_LOCK=true

# Set TTL in seconds (default: no TTL)
CODEX_LOCK_TTL=3600

# Enable lock wait with timeout
CODEX_LOCK_WAIT=true
CODEX_LOCK_WAIT_TIMEOUT=300  # 5 minutes
```
