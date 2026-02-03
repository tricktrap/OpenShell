# Sandbox Architecture

Navigator's sandboxing isolates a user command in a child process while policy parsing and
platform-specific enforcement live behind clear interfaces. The `navigator-sandbox` binary is the
entry point and spawns a child process, applying restrictions before `exec`.

## Components

- `crates/navigator-sandbox`: CLI + library that loads policy, spawns the child process, and applies sandbox rules.
- `crates/navigator-core`: shared types and utilities (policy schema, errors, config).

## Policy Model

Sandboxing is driven by a required YAML policy file. Provide it via `--policy` or
`NAVIGATOR_SANDBOX_POLICY`. The policy schema includes:

- `filesystem`: read-only and read-write allow lists, plus optional inclusion of the workdir.
- `network`: mode (`allow`, `block`, `proxy`) and optional proxy configuration.
- `landlock`: compatibility behavior (`best_effort` or `hard_requirement`).
- `process`: optional `run_as_user`/`run_as_group` to drop privileges for the child process.

See `docs/sandbox-policy.yaml` for an example policy.

## Linux Enforcement (Landlock + Seccomp)

Linux enforcement lives in `crates/navigator-sandbox/src/sandbox/linux`.

- Landlock restricts filesystem access to the allow lists from the policy. If no paths are listed,
  Landlock is skipped. When enabled, a ruleset is created and enforced before the child `exec`.
- Seccomp blocks socket creation for common network domains (IPv4/IPv6 and others), preventing the
  child process from opening outbound sockets directly.

## Proxy Routing

When `network.mode: proxy` is set, `NAVIGATOR_PROXY_SOCKET` is exported to the child. Seccomp still
blocks direct socket creation so traffic must flow through the proxy channel.

## Process Privileges

The sandbox supervisor can run as a more privileged user while the child process drops to a less
privileged account before `exec`. Configure this via `process.run_as_user` and
`process.run_as_group` in the policy. If unset, the child inherits the supervisor's user/group.

## Platform Extensibility

Platform-specific implementations are wired through `crates/navigator-sandbox/src/sandbox/mod.rs`.
Non-Linux platforms currently log a warning and skip enforcement, leaving room for a macOS backend
later without changing the public policy or CLI surface.
