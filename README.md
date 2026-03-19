# shahanshahi

Rust library for the **Shahanshahi (Imperial Iranian)** civil calendar — developed in the spirit of [Melliran](https://github.com/melliran): open Persian technology and shared digital infrastructure so teams and organizations can adopt **spec-backed** Persian calendar tooling without fragile one-offs.

**Why spec-first:** see [`docs/VISION.md`](./docs/VISION.md).

**Status: pre-implementation.** The crate publishes a workspace skeleton only. Arithmetic and conversions ship after [`SPEC.md`](./SPEC.md) and golden dates in [`data/reference-dates.json`](./data/reference-dates.json) are filled from primary sources.

## Repository layout

| Path | Purpose |
|------|--------|
| `SPEC.md` | Authoritative rules this code implements |
| `data/reference-dates.json` | Vetted Y/M/D pairs + citations for tests |
| `crates/shahanshahi` | Library crate (API to grow with the spec) |

## Building

From the repository root:

```bash
cargo build -p shahanshahi
cargo test -p shahanshahi
```

## Repository

[github.com/melliran/shahanshahi](https://github.com/melliran/shahanshahi)

## Contributing

See [`CONTRIBUTING.md`](./CONTRIBUTING.md).

## License

Licensed under either of [Apache-2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT) at your option.
