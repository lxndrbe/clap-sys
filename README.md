# clap-sys (fork)

Rust bindings for the [CLAP](https://cleveraudio.org/) audio plugin API.

This is a **maintained fork** of the original [`clap-sys`](https://github.com/micahrj/clap-sys) by Micah Johnston. It tracks a newer CLAP version and a newer Rust toolchain than the package currently published on crates.io.

| | This fork | crates.io `clap-sys` 0.5.0 |
|---|---|---|
| CLAP | 1.2.10 | 1.2.2 |
| Rust edition / MSRV | 2024 / 1.85 | 2021 / 1.64 |
| Publish | Git only | [crates.io](https://crates.io/crates/clap-sys) |

**Not published on crates.io.** The crate name `clap-sys` there remains owned by the original project. Use this fork via Git:

```toml
clap-sys = { git = "https://github.com/lxndrbe/clap-sys", tag = "v0.6.0" }
```

## Compatibility

Because of the CLAP bump, MSRV raise, and a few binding renames/signature fixes, this fork is **not drop-in compatible** with every crate that depends on crates.io `clap-sys` 0.5.x (for example older [clack](https://github.com/prokopyl/clack) checkouts). Treat it as a distinct line until those projects opt in.

## Related projects

- Upstream bindings: [micahrj/clap-sys](https://github.com/micahrj/clap-sys)
- [clack](https://github.com/prokopyl/clack): safe Rust wrappers over CLAP (may pin crates.io `clap-sys`)
- [clap-sys-stubs](https://github.com/spwilson2/clap-sys-stubs): empty stubs for every CLAP interface
- [clap-validator](https://github.com/robbert-vdh/clap-validator): validation / automated testing of CLAP plugins
- [NIH-plug](https://github.com/robbert-vdh/nih-plug): high-level Rust plugin framework with CLAP support

## Credits

- **Micah Johnston** — original `clap-sys` author and design
- **Robbert van der Helm** — long-time co-maintainer and API updates
- **Quant1um** — CLAP 1.2.x binding updates through 1.2.10 that this fork builds on
- Further contributions from the upstream and fork histories (see `git shortlog`)

## License

Same as upstream: dual-licensed under [MIT](LICENSE-MIT) and [Apache-2.0](LICENSE-APACHE). Contributions are accepted under the same terms.
