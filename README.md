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

The URL is `/events/<slug>`, and `slug` defaults to the file name without its extension. Set it explicitly on anything you have shared a link to:

```toml
slug = "september-2026"
```

Leave `luma_url = ""` until the Luma page exists. Cards and the calendar then show "RSVP opens ~2 weeks before" in place of the link, and a spotlight page's button subscribes people to <https://luma.com/riscv-ottawa> instead, so they get told the moment RSVP opens. Filling `luma_url` in switches every one of them to "Save your spot" with no other change.

### A spotlight event

Now and then a monthly meetup turns into a bigger night. Add a `[spotlight]` block to its file and it gets a page of its own at `/events/<filename-without-.toml>`, a band above the hero on the home page carrying the countdown, and a pinned block at the top of `/events`. Events without the block are untouched: card, modal, calendar cell, exactly as before.

Only `kicker`, `headline`, and `tagline` are required. Everything else is optional and its section is left out when empty, so the page can go up as soon as there is something worth announcing.

```toml
[spotlight]
kicker = "our biggest night yet"          # eyebrow above the headline
headline = "RISC-V in 2026: ..."          # the marquee line, not `title`
tagline = "One sentence: who it is for and what they leave with."
draws = ["Three reasons to give up an evening, in the visitor's terms."]
call_to_build = ["Skills and contributors we want, one line each."]
og_image = "/og-september.png"            # optional; a 1200x630 file in public/

[[spotlight.speakers]]
name = "Megan Lehn"
affiliation = "RISC-V International"
affiliation_url = "https://riscv.org"     # optional; links the affiliation text
role = "Invited talk"
topic = "What they are speaking about."
status = "confirmed"                      # confirmed (default) | pending | tba
link = "https://example.com"              # optional; the person's own page

[spotlight.panel]
title = "Panel title"
subtitle = "One line on what the panel covers."
moderator = { name = "...", affiliation = "...", role = "Moderator" }

[[spotlight.panel.panelists]]              # same shape as a speaker
name = "Mike Thompson"
affiliation = "OpenHW Foundation"
role = "Hardware"

[[spotlight.schedule]]
minutes = 45                              # block length; times computed from `date`
title = "Panel: RISC-V in 2026"
presenter = "Moderated by ..."            # optional
detail = "Optional sentence."
highlight = true                          # accent treatment; use it sparingly

[spotlight.teaser]                        # something announced on the night only
label = "// project announcement"
headline = "..."
body = "..."
```

The run of show carries block lengths, not clock times. The page accumulates them
from the event's `date` and prints a range per block, so moving the start time
moves the whole evening with it. An event dated `YYYY-MM` has nothing to anchor
the first block to and prints lengths instead.

### Speaker headshots

Put the image under `public/`, which is served from the site root, and point at it with a site-root path:

```toml
[[spotlight.speakers]]
name = "Megan Lehn"
affiliation = "RISC-V International"
photo = "/people/megan-lehn.jpg"
```

Square images, ideally around 256x256; anything else is cropped to a square by the browser.
The same field works on panelists and on the panel moderator.

`photo` is optional and worth leaving out until the picture actually exists. A speaker without one gets a circle with their initials, and a `tba` slot gets an empty dashed circle, so a half-filled lineup still looks deliberate and nothing shifts as photos arrive. A path pointing at a missing file fails `cargo test`, so a typo shows up before the broken image does.

`status = "tba"` leaves the name out and renders a dashed "To be announced" card, which is the point: a lineup that visibly fills up gives people a reason to come back. `status = "pending"` shows the name with an "invited" mark. Only confirmed names appear on the home page band.

### Checking events against Luma

Luma is the source of truth for when and where an event happens and whether its page is live. The copy in `content/events/` is hand-crafted (with love), but to avoid potential mistakes, you can use the following to find where the two have drifted apart:

```bash
cargo test --features ssr --no-default-features --test luma_drift -- --ignored --nocapture
```

It reads the calendar's public ICS feed and reports events published on Luma while `luma_url` here is still empty, start times that moved, links that no longer resolve, and events on Luma with no file at all. Titles and venue strings are not compared.
The test is `#[ignore]`d because it needs the network, so a plain `cargo test` skips it.

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
- `cargo test --features ssr` (the network-bound Luma drift check is `#[ignore]`d and skipped here)
- `cargo leptos build --release`

`.github/workflows/luma-drift.yml` runs the drift check on its own, Mondays at 13:00 UTC and on demand from the Actions tab. It is kept off pull requests on purpose: it depends on Luma and fails for content reasons, so gating merges on it would turn unrelated changes red. Note that GitHub disables scheduled workflows after 60 days without repository activity.

## License

MIT. See [LICENSE](LICENSE).
