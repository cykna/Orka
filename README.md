# Orka

Orka is a lightweight container runtime written in Rust, built on top of Linux namespaces (via `clone` syscall). It serves as a low-level containerd-like foundation to run isolated processes — and as a base for higher-level tooling like a Docker-compatible interface.

## How it works

Orka uses `CLONE_NEWPID` to spawn processes in isolated PID namespaces, similar to how container runtimes isolate workloads from the host system.

## Usage

```sh
orka --exec <binary> [--arg <arg>...] [--env KEY=VALUE...]
```

**Example:**

```sh
orka --exec /bin/sh --env PATH=/usr/bin:/bin
```

## Building

```sh
cargo build --release
```

## Requirements

- Linux (namespace support required)
- Rust 2024 edition

## Roadmap

- [ ] Network namespace isolation (`CLONE_NEWNET`)
- [ ] Mount namespace + rootfs pivot
- [ ] cgroups resource limits
- [ ] Image layer support
- [ ] Docker-compatible CLI layer

## License

MIT
