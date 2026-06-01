use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin {
    Learn,
    Build,
    Meet,
}

impl Pin {
    fn tag(self) -> &'static str {
        match self {
            Pin::Learn => "01",
            Pin::Build => "02",
            Pin::Meet => "03",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Pin::Learn => "Learn",
            Pin::Build => "Build",
            Pin::Meet => "Meet",
        }
    }

    fn body(self) -> &'static str {
        match self {
            Pin::Learn => "Hands-on projects covering the RISC-V ISA, embedded systems, FPGA development, and more.",
            Pin::Build => "Join us for project nights; bring a board or bring a question, and let's build together!",
            Pin::Meet => "Talks from practitioners in verification, embedded systems, computer security, and chip design.",
        }
    }

    fn aria_label(self) -> &'static str {
        match self {
            Pin::Learn => "Show Learn details",
            Pin::Build => "Show Build details",
            Pin::Meet => "Show Meet details",
        }
    }
}

#[component]
pub fn CpuWidget() -> impl IntoView {
    let active = RwSignal::new(Pin::Learn);

    view! {
        <section class="container-page hairline py-16">
            <div class="grid items-center gap-12 md:grid-cols-2">
                <div class="mx-auto w-full max-w-md">
                    <CpuSvg active=active/>
                </div>
                <DetailPanel active=active/>
            </div>
        </section>
    }
}

#[component]
fn DetailPanel(active: RwSignal<Pin>) -> impl IntoView {
    view! {
        <div aria-live="polite">
            <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">
                {move || format!("/ {}", active.get().tag())}
            </p>
            <h2 class="mt-3 font-mono text-3xl font-bold text-ink md:text-4xl">
                {move || active.get().title()}
            </h2>
            <p class="mt-4 max-w-md text-mute">
                {move || active.get().body()}
            </p>
            <p class="mt-8 font-mono text-xs uppercase tracking-[0.3em] text-mute">
                "click a solder point to switch"
            </p>
        </div>
    }
}

#[component]
fn CpuSvg(active: RwSignal<Pin>) -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 400 400"
            class="h-auto w-full"
            xmlns="http://www.w3.org/2000/svg"
            role="img"
            aria-label="Interactive RISC-V chip with three clickable solder points"
        >
            // decorative pin stubs (16 of them, four per side)
            <g aria-hidden="true" stroke="var(--color-line)" stroke-width="2">
                // top side (skip the live one at x=200 by drawing it as live trace below)
                <line x1="140" y1="100" x2="140" y2="86"/>
                <line x1="170" y1="100" x2="170" y2="86"/>
                <line x1="230" y1="100" x2="230" y2="86"/>
                <line x1="260" y1="100" x2="260" y2="86"/>
                // bottom side
                <line x1="140" y1="300" x2="140" y2="314"/>
                <line x1="170" y1="300" x2="170" y2="314"/>
                <line x1="230" y1="300" x2="230" y2="314"/>
                <line x1="260" y1="300" x2="260" y2="314"/>
                // left side
                <line x1="100" y1="140" x2="86" y2="140"/>
                <line x1="100" y1="170" x2="86" y2="170"/>
                <line x1="100" y1="230" x2="86" y2="230"/>
                <line x1="100" y1="260" x2="86" y2="260"/>
                // right side
                <line x1="300" y1="140" x2="314" y2="140"/>
                <line x1="300" y1="170" x2="314" y2="170"/>
                <line x1="300" y1="230" x2="314" y2="230"/>
                <line x1="300" y1="260" x2="314" y2="260"/>
            </g>

            // chip body
            <rect
                x="100"
                y="100"
                width="200"
                height="200"
                rx="6"
                fill="var(--color-surface)"
                stroke="var(--color-line)"
                stroke-width="2"
                aria-hidden="true"
            />
            // notch indicator (orientation dot)
            <circle cx="120" cy="120" r="3" fill="var(--color-mute)" aria-hidden="true"/>

            // silkscreen
            <text
                x="200"
                y="195"
                text-anchor="middle"
                class="font-mono"
                font-family="var(--font-mono)"
                font-size="22"
                font-weight="700"
                fill="var(--color-ink)"
                aria-hidden="true"
            >
                "RV32I"
            </text>
            <text
                x="200"
                y="220"
                text-anchor="middle"
                class="font-mono"
                font-family="var(--font-mono)"
                font-size="11"
                letter-spacing="3"
                fill="var(--color-mute)"
                aria-hidden="true"
            >
                "riscvottawa"
            </text>

            // live pin traces (chip edge to solder point)
            <g aria-hidden="true" stroke="var(--color-line)" stroke-width="2">
                <line x1="200" y1="100" x2="200" y2="40"/>
                <line x1="300" y1="200" x2="360" y2="200"/>
                <line x1="200" y1="300" x2="200" y2="360"/>
            </g>

            // solder points (interactive)
            <SolderPoint pin=Pin::Learn cx=200.0 cy=40.0 active=active/>
            <SolderPoint pin=Pin::Build cx=360.0 cy=200.0 active=active/>
            <SolderPoint pin=Pin::Meet cx=200.0 cy=360.0 active=active/>
        </svg>
    }
}

#[component]
fn SolderPoint(pin: Pin, cx: f64, cy: f64, active: RwSignal<Pin>) -> impl IntoView {
    let on_click = move |_| active.set(pin);
    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        let key = ev.key();
        if key == "Enter" || key == " " || key == "Spacebar" {
            ev.prevent_default();
            active.set(pin);
        }
    };

    let cx_str = cx.to_string();
    let cy_str = cy.to_string();

    view! {
        <g
            role="button"
            tabindex="0"
            aria-label=pin.aria_label()
            class="cursor-pointer outline-none focus-visible:[outline:2px_solid_var(--color-accent)]"
            on:click=on_click
            on:keydown=on_keydown
        >
            // glow ring (visible only when active)
            <circle
                cx=cx_str.clone()
                cy=cy_str.clone()
                r="14"
                fill="none"
                stroke="var(--color-accent)"
                stroke-width="1.5"
                opacity=move || if active.get() == pin { "0.9" } else { "0" }
                style="transition: opacity 200ms ease;"
            />
            // solder dot itself
            <circle
                cx=cx_str
                cy=cy_str
                r="6"
                fill=move || if active.get() == pin { "var(--color-accent)" } else { "var(--color-warm)" }
                class="animate-solder-pulse"
            />
        </g>
    }
}
