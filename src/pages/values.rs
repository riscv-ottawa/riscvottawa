use leptos::prelude::*;
use leptos_meta::Title;

struct Value {
    number: &'static str,
    title: &'static str,
    in_practice: &'static str,
    when_violated: &'static str,
}

const VALUES: [Value; 5] = [
    Value {
        number: "01",
        title: "Open ISA, open community",
        in_practice: "Default to public meetings, open-licensed materials, and transparent \
                      finances.",
        when_violated: "Private decisions about public matters; closed-door member selection.",
    },
    Value {
        number: "02",
        title: "Hands-on",
        in_practice: "Workshops produce things; talks come with code or RTL. Build, break, \
                      contribute.",
        when_violated: "All-theory presentations; vendor pitches with no demos.",
    },
    Value {
        number: "03",
        title: "Vendor-neutral",
        in_practice: "No exclusive sponsor content and no single-vendor cheerleading; \
                      sponsors get visibility, not editorial control.",
        when_violated: "Workshop titles that read like product launches; speakers who only \
                        know their own stuff and won't compare.",
    },
    Value {
        number: "04",
        title: "Be excellent to each other",
        in_practice: "Default to charitable interpretation; address harm directly and quickly.",
        when_violated: "Snarky culture; \"well actually\" gatekeeping; harassment unaddressed.",
    },
    Value {
        number: "05",
        title: "Respect everyone's time",
        in_practice: "Events start on time, end on time, and have a clear point; async-first \
                      comms; no pointless meetings.",
        when_violated: "Late-running meetups; meetings without agendas; \"quick question\" \
                        pings that consume hours.",
    },
];

#[component]
pub fn Values() -> impl IntoView {
    view! {
        <Title text="Values - RISC-V Ottawa"/>
        <section class="container-page py-16">
            <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">"/ values"</p>
            <h1 class="mt-4 font-mono text-4xl font-bold text-ink md:text-5xl">
                "Values"
            </h1>
            <p class="mt-4 max-w-2xl text-mute">
                "The principles that shape the RISC-V Ottawa community and the way we work "
                "together. They are meant to be testable: when a behavior violates a value, the "
                "value should help us adjudicate."
            </p>

            <div class="mt-12 grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                {VALUES
                    .iter()
                    .map(|v| view! { <ValueBox value=v/> })
                    .collect::<Vec<_>>()}
            </div>
        </section>
    }
}

#[component]
fn ValueBox(value: &'static Value) -> impl IntoView {
    view! {
        <div class="flex flex-col rounded-sm border border-line bg-surface p-6">
            <p class="font-mono text-sm text-accent">{value.number}</p>
            <h2 class="mt-3 font-mono text-xl font-semibold text-ink">{value.title}</h2>
            <p class="mt-3 text-sm text-mute">{value.in_practice}</p>
            <div class="mt-4 border-t border-line pt-4">
                <p class="font-mono text-xs uppercase tracking-[0.2em] text-accent">
                    "when violated"
                </p>
                <p class="mt-2 text-sm text-mute">{value.when_violated}</p>
            </div>
        </div>
    }
}
