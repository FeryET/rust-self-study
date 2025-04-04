# Rust Projects for Learning Linux Kernel Mechanisms (Generated by DeepSeek)

## 1. Process Monitor (`ps` Clone)

**Basic Scope**:

- List running processes (PID, name, status).
- Parse `/proc/[pid]/stat` and `/proc/[pid]/cmdline`.
- Display user/group IDs (UID/GUID).

**Learn**:

- `/proc` filesystem structure
- Process lifecycle (fork, exec, exit)
- User vs. kernel threads

**Rust Crates**:

- `nix` (for UID/GID handling)
- `procfs` (parse `/proc`)

**Extensions**:

- Live refresh (like `top`)
- CPU/memory usage graphs
- Process tree hierarchy

______________________________________________________________________

## 2. `strace` Lite

**Basic Scope**:

- Trace system calls made by a process.
- Print syscall names (e.g., `read`, `write`).
- Use `ptrace` to attach to a process.

**Learn**:

- `ptrace` system call
- Syscall table/numbers
- Signal handling (SIGTRAP)

**Rust Crates**:

- `nix` (ptrace wrapper)
- `syscalls` (syscall definitions)

**Extensions**:

- Show syscall arguments/return values
- Filter specific syscalls (e.g., only file I/O)
- Measure syscall latency

______________________________________________________________________

## 3. Minimal Load Balancer

**Basic Scope**:

- TCP proxy that forwards connections to 2 backend servers.
- Round-robin selection.
- Basic error handling (skip unresponsive backends).

**Learn**:

- `epoll` for non-blocking I/O
- Socket programming
- Connection state tracking

**Rust Crates**:

- `tokio` (async runtime)
- `socket2` (raw socket control)

**Extensions**:

- Health checks (HTTP/ICMP)
- Least-connections algorithm
- TLS termination

______________________________________________________________________

## 4. FUSE "Hello World"

**Basic Scope**:

- Mount a virtual filesystem that shows:
  - A static file (`echo "Hello World" > /mnt/test`)
  - Directory listing with 1 fake file.

**Learn**:

- FUSE API (`getattr`, `readdir`, `open`)
- VFS (Virtual Filesystem Switch)
- Inode/dentry concepts

**Rust Crate**:

- `fuser`

**Extensions**:

- In-memory key-value store as files
- Write support
- File permissions (UID-based)

______________________________________________________________________

## 5. Nano Container Runtime

**Basic Scope**:

- Run a process in isolated PID/mount namespaces.
- Use `unshare` to create namespaces.
- Basic chroot-like filesystem isolation.

**Learn**:

- Linux namespaces (PID, mount)
- `clone()` syscall
- `pivot_root` for filesystem isolation

**Rust Crates**:

- `nix` (namespace handling)
- `libc` (raw syscalls)

**Extensions**:

- cgroups for memory/CPU limits
- OverlayFS for layered images
- Network namespace + virtual Ethernet

______________________________________________________________________

## 6. Packet Sniffer (Lite)

**Basic Scope**:

- Capture ICMP packets (ping).
- Print source/destination IP addresses.
- Use raw sockets.

**Learn**:

- Ethernet/IP packet headers
- `AF_PACKET` sockets
- Promiscuous mode

**Rust Crates**:

- `pcap`
- `etherparse` (packet parsing)

**Extensions**:

- TCP/UDP payload inspection
- DNS query logging
- BPF filter support

______________________________________________________________________

## 7. `inotify` Backup Watcher

**Basic Scope**:

- Watch a directory for file changes (create/modify).
- Copy changed files to `./backup/`.

**Learn**:

- `inotify` API (IN_CREATE, IN_MODIFY)
- File event coalescing
- `sendfile` syscall for efficient copies

**Rust Crates**:

- `notify` (inotify wrapper)
- `tokio` (async file IO)

**Extensions**:

- Deduplication
- S3/remote backups
- File versioning

______________________________________________________________________

## 8. Mini Shell

**Basic Scope**:

- Run commands (e.g., `ls`, `echo`).
- Support `&&` and `||` operators.
- Basic environment variable expansion.

**Learn**:

- `fork()`/`execve()`
- Signal handling (Ctrl+C = SIGINT)
- Pipe/file redirection (stdout > file)

**Rust Crates**:

- `nix` (process control)
- `libc` (signals)

**Extensions**:

- Pipes (`|`)
- Background jobs (`&`)
- Tab-completion

______________________________________________________________________

## 9. Systemd Lite (Service Manager)

**Basic Scope**:

- Start/stop services from config files.
- Track PID of running services.
- Restart crashed services.

**Learn**:

- Daemonization (`fork` + setsid)
- Process supervision
- Logging to syslog/journald

**Rust Crates**:

- `syslog`
- `serde` (config parsing)

**Extensions**:

- Dependency ordering
- Socket activation
- Resource limits (cgroups)

______________________________________________________________________

## 10. Kernel Module (Rust)

**Basic Scope**:

- "Hello World" module that logs to `dmesg`.
- Create a `/sys/hello` sysfs entry.

**Learn**:

- Kernel module lifecycle (`init`, `exit`)
- Sysfs interface
- Safe Rust in kernel space

**Tools**:

- `rust-for-linux` (experimental)
- `kbuild` system

**Extensions**:

- Character device driver
- Interrupt handling
- Kernel thread creation

______________________________________________________________________

## Starter Recommendations

1. **Begin with**:
   - Process Monitor → Learn `/proc` and process lifecycle
   - Mini Shell → Master `fork`/`exec`
1. **Then try**:
   - `strace` Lite → Deep dive into syscalls
   - Nano Container → Understand isolation primitives
1. **Advanced**:
   - FUSE Filesystem → Filesystem internals
   - Kernel Module → Bare-metal Linux interaction
