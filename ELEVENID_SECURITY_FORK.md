# ElevenID temporary security backport

This is a temporary downstream fork of [`gtk-rs/gtk-rs-core`](https://github.com/gtk-rs/gtk-rs-core).
ElevenID consumes only the `glib` package from the
`elevenid/security-backport-2024-0429` commit; it does not consume a forked GTK
stack.

## Scope

- Base: upstream `0.18.5`.
- Advisory: `RUSTSEC-2024-0429` / `GHSA-wrw7-89jp-8q8g`.
- Change: the two-line upstream `VariantStrIter` fix from
  [gtk-rs/gtk-rs-core#1343](https://github.com/gtk-rs/gtk-rs-core/pull/1343),
  which changes the GLib out-parameter from an immutable reference to a mutable
  one.
- Packaging: `glib` uses released `*-sys` and `glib-macros` crates instead of
  workspace paths. This prevents the patch from replacing unrelated GTK crates.

There are no API or ABI changes. Upstream licensing and history are preserved.

## Removal trigger

`ElevenID/marty-verifier` must remove this patch when a stable Tauri release
uses the GTK4/WebKitGTK6 dependency path and resolves `glib >= 0.20`. Track the
Tauri migration in the verifier's public dependency-health record and issue
`#31`; review it on every Tauri release and no later than the recorded review
date. This fork is not a permanent dependency.
