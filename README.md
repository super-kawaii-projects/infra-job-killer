# infra-job-killer

A Leptos web application for managing infrastructure deployments and jobs.

Built with [Leptos](https://leptos.dev) (Rust full-stack web framework), served as a single self-contained binary.

## Architecture

```
infra-job-killer/
├── frontend/       # Leptos WASM frontend + server
├── shared/         # Shared types between client and server
├── Dockerfile
└── docker-compose.yml
```

## Prerequisites

- Rust toolchain (see `rust-toolchain.toml`)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos): `cargo install cargo-leptos`

## Development

```bash
cargo leptos watch
```

App runs at http://localhost:3000

## Production build

```bash
cargo leptos build --release
```

## Docker

```bash
docker compose up --build
```

Runs on port 3000. Data and deployments persist to `./data` and `./deployments`.

## License

Copyright (c) 2026 Stillwater Strategic Solutions LLC. All rights reserved.

Source-available software, not open source. Free for personal, non-commercial
evaluation. **Commercial and production use requires a paid license.** See
[LICENSE](LICENSE) or contact michaelisaacs121092@gmail.com for commercial
licensing.
