# scryer-plugin-pdk

Guest runtime bindings for **Scryer** WebAssembly plugins.

This crate is the guest half of Scryer's command-model plugin invocation
protocols. The host runs the plugin as a `wasm32-wasip1` **command** (a `_start`
entry), writes one request document to the plugin's stdin, and reads exactly one
response document from its stdout.

It serves **Scryer's plugin contract** — it is deliberately *not* a
general-purpose plugin framework. The API promise is "what Scryer's host
provides". Wire types are *not* owned here: the protocol/descriptor/schema types
remain the single source of truth in [`scryer-plugin-sdk`], which this crate
depends on and re-exports.

## What it provides

- Descriptor-aware archive and subtitle-sync runners and `main` macros. With a
  `describe` argument they lazily build and write one `PluginDescriptor`; with
  no argument they dispatch the normal request/response protocol.
- Compatibility runners and macro forms for command plugins that still own
  descriptor dispatch themselves.
- A panic hook that reports to stderr (guests build `panic = "abort"`, so the
  process then aborts / the host observes a trap).
- Re-exports of command wire-protocol types from `scryer-plugin-sdk`, plus the
  complete SDK under `scryer_plugin_pdk::sdk` for descriptor construction.

Protocol-level faults (malformed request, unwritable stdout) go to stderr and
exit non-zero. Operational outcomes are reported **in-band** through the
protocol response, never by exiting non-zero.

The stdin/stdout transport is isolated in one module (`framing`). If a host
spike shows stdin/stdout capture misbehaves under `wasmtime-wasi`, the
documented fallback (request/response files in a dedicated rw control preopen,
same JSON) is a contained change to that module only.

## Usage

```rust
use scryer_plugin_pdk::{ArchivePluginProcessRequest, ArchivePluginProcessResponse};

fn descriptor() -> scryer_plugin_pdk::sdk::PluginDescriptor {
    // ... construct the plugin descriptor ...
    unimplemented!()
}

fn handle(request: ArchivePluginProcessRequest) -> ArchivePluginProcessResponse {
    // ... inspect / extract / verify-repair / repair-then-extract ...
    unimplemented!()
}

scryer_plugin_pdk::scryer_archive_plugin_main!(
    descriptor = descriptor,
    handler = handle,
);
```

## Building the guest artifact

The plugin is a **command** binary: it needs a `main` (the macro provides one)
and is built for a `wasm32-wasip1` target with `panic = "abort"`. The resulting
module exports `_start` and `memory`. For the archive plugin it imports exactly
the two host crypto functions (`host_aes_cbc_decrypt`, `host_crc32`; RFC 123
§5). `weaver-unrar` defaults those imports to the neutral `host` namespace;
Scryer builds it with `host-abi-extism` to route them through
`extism:host/user`. The namespace is compatibility routing only: the guest has
no Extism dependency.

Scryer's host supports baseline Wasm plus SIMD and relaxed SIMD. The catalog
`feature_sets` metadata in each plugin's `[package.metadata.scryer]` selects a
matching flavor per host. Build each flavor with the target/`RUSTFLAGS` below;
the slugs mirror `required_features`:

| Flavor | `required_features` | Build |
|---|---|---|
| baseline | `[]` | `cargo build --profile plugin-release --target wasm32-wasip1` |
| simd | `["simd128"]` | baseline + `RUSTFLAGS="-C target-feature=+simd128"` |
| relaxed-simd | `["simd128","relaxed-simd"]` | baseline + `RUSTFLAGS="-C target-feature=+simd128,+relaxed-simd"` |

Threads and exception handling are not supported by Scryer's plugin host. Do
not publish plugin flavors that require either feature.

Release artifacts follow the repo convention: `wasm-opt -Oz` (with the matching
`--enable-*` flags per flavor), `zstd -19`, BLAKE3 digest, cosign bundle.
The release tool invokes the PDK-owned `describe` command after optimization,
validates descriptor parity across feature variants, and embeds the descriptor
as `scryer.plugin-descriptor.v1` before compression and digest generation. The
PDK does not mutate or post-process its own Wasm binary.

## Versioning

`0.x` during the RFC 123 program. Version 0.4 adds the standardized
descriptor-aware entry points. Releases remain semver-honest, but the API
promise stays "what Scryer's host provides", nothing broader. Publication to
crates.io is owner-triggered.

[`scryer-plugin-sdk`]: https://crates.io/crates/scryer-plugin-sdk
