# riscvottawa

Homepage for the RISC-V Ottawa community group.
Built with [Leptos](https://www.leptos.dev) (Rust full-stack), [Axum](https://github.com/tokio-rs/axum), and [Tailwind CSS](https://tailwindcss.com/).

## Quick start

Prerequisites: Rust stable (1.89 or later), the `wasm32-unknown-unknown` target, and `cargo-leptos`.

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked cargo-leptos --version 0.3.6
```

Run the dev server:
```bash
cargo leptos serve/watch
```

Open <http://localhost:3000>. The watcher rebuilds on changes to `src/`, `style/`, and `content/`.

## Adding content

Add or remove files under `content/events/`, `content/projects/`, and `content/resources/`.
The server loads everything at startup and aborts with a clear error if any file fails to parse.

### A new event

Create `content/events/<YYYY-MM-DD-slug>.toml`:

```toml
title = "Intro to RISC-V"
date = "2026-05-08T18:00:00-04:00"   # RFC3339, or "2026-05" if only the month is settled
location = "uOttawa, Ottawa"
summary = "One-paragraph teaser shown in cards and the calendar."
description = """
Longer body shown in the day-detail panel.
"""
luma_url = "https://lu.ma/your-event-id"
tags = ["beginner", "architecture"]
```

### A new project

Create `content/projects/<NN-slug>.toml`:

```toml
title = "RISC-V Fundamentals"
level = "beginner"             # beginner | intermediate | advanced
duration = "4 hours"  #optional
summary = "..."
description = """
Long form description.
"""
prerequisites = ["basic C", "shell"]
contact_url = "mailto:projects@riscvottawa.org"   # optional
```

### A new resource section or link

Resources are grouped into sections under `content/resources/`. Each file is one section, named `<NN>-<slug>.toml`. The numeric prefix controls display order (lower numbers appear first).

To add a link to an existing section, open the corresponding file and append a `[[links]]` entry:

```toml
[[links]]
name = "Display name"
href = "https://example.com"
```

To add a new section, create `content/resources/<NN>-<slug>.toml`:

```toml
title = "Section heading"

[[links]]
name = "First link"
href = "https://example.com"
```

## Production build

```bash
cargo leptos build --release
```

Outputs:

- `target/release/riscvottawa` — the SSR binary.
- `target/site/` — public asset directory (CSS, WASM, JS, fonts).

The binary loads `./content` from its current working directory at startup and listens on `LEPTOS_SITE_ADDR` (defaults to `127.0.0.1:3000`).

Relevant runtime environment variables:

| Variable                 | Default              | Purpose                                   |
| ------------------------ | -------------------- | ----------------------------------------- |
| `LEPTOS_OUTPUT_NAME`     | `riscvottawa`        | Bundle name (`pkg/<name>.{js,wasm,css}`)  |
| `LEPTOS_SITE_ROOT`       | `target/site`        | Where to serve static assets from         |
| `LEPTOS_SITE_PKG_DIR`    | `pkg`                | Subdir of `SITE_ROOT` that holds bundles  |
| `LEPTOS_SITE_ADDR`       | `127.0.0.1:3000`     | Listen address                            |

## Container (Podman/Docker)

Build and run:

```bash
podman build -t riscvottawa .
podman run --rm -p 3000:3000 riscvottawa
```

The image is multi-stage: a `rust:bookworm` builder runs `cargo leptos build --release`, then the result is copied into a `debian:bookworm-slim` runtime that runs as a non-root `app` user. The final image is roughly 80–150 MB.

## CI

`.github/workflows/ci.yml` runs on every push to `main` and every pull request:

- `cargo fmt --all -- --check`
- `cargo clippy --features ssr -- -D warnings`
- `cargo clippy --features hydrate --target wasm32-unknown-unknown --lib -- -D warnings`
- `cargo leptos build --release`

## License

MIT. See [LICENSE](LICENSE).
