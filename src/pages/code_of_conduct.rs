use leptos::prelude::*;
use leptos_meta::Title;

const DISCORD_URL: &str = "https://discord.gg/EfryE4wfk4";
const GOVERNANCE_URL: &str = "https://github.com/riscv-ottawa/governance";

#[component]
pub fn CodeOfConduct() -> impl IntoView {
    view! {
        <Title text="Code of Conduct - RISC-V Ottawa"/>
        <section class="container-page py-16">
            <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">
                "/ code of conduct"
            </p>
            <h1 class="mt-4 font-mono text-4xl font-bold text-ink md:text-5xl">
                "Code of Conduct"
            </h1>

            <div class="mt-8 max-w-3xl space-y-12">
                <div class="space-y-4 text-mute">
                    <p>
                        "RISC-V Ottawa has adopted the Contributor Covenant, version 3.0, as its "
                        "Code of Conduct for all online community spaces. It puts our value of "
                        "\"be excellent to each other\" into concrete terms. In-person events "
                        "additionally follow the Berlin Code of Conduct."
                    </p>
                </div>

                <Section title="Our Pledge">
                    <p>"We pledge to make our community welcoming, safe, and equitable for all."</p>
                    <p>
                        "We are committed to fostering an environment that respects and promotes "
                        "the dignity, rights, and contributions of all individuals, regardless of "
                        "characteristics including race, ethnicity, caste, color, age, physical "
                        "characteristics, neurodiversity, disability, sex or gender, gender "
                        "identity or expression, sexual orientation, language, philosophy or "
                        "religion, national or social origin, socio-economic position, level of "
                        "education, or other status. The same privileges of participation are "
                        "extended to everyone who participates in good faith and in accordance "
                        "with this Covenant."
                    </p>
                </Section>

                <Section title="Encouraged Behaviors">
                    <p>
                        "While acknowledging differences in social norms, we all strive to meet "
                        "our community's expectations for positive behavior. We also understand "
                        "that our words and actions may be interpreted differently than we intend "
                        "based on culture, background, or native language."
                    </p>
                    <p>
                        "With these considerations in mind, we agree to behave mindfully toward "
                        "each other and act in ways that center our shared values, including:"
                    </p>
                    <ol class="list-decimal space-y-2 pl-6">
                        <EmphItem pre="Respecting the " strong="purpose of our community" post=", our activities, and our ways of gathering."/>
                        <EmphItem pre="Engaging " strong="kindly and honestly" post=" with others."/>
                        <EmphItem pre="Respecting " strong="different viewpoints" post=" and experiences."/>
                        <EmphItem pre="" strong="Taking responsibility" post=" for our actions and contributions."/>
                        <EmphItem pre="Gracefully giving and accepting " strong="constructive feedback" post="."/>
                        <EmphItem pre="Committing to " strong="repairing harm" post=" when it occurs."/>
                        <EmphItem pre="Behaving in other ways that promote and sustain the " strong="well-being of our community" post="."/>
                    </ol>
                </Section>

                <Section title="Restricted Behaviors">
                    <p>
                        "We agree to restrict the following behaviors in our community. Instances, "
                        "threats, and promotion of these behaviors are violations of this Code of "
                        "Conduct."
                    </p>
                    <ol class="list-decimal space-y-2 pl-6">
                        <Term term="Harassment." desc="Violating explicitly expressed boundaries or engaging in unnecessary personal attention after any clear request to stop."/>
                        <Term term="Character attacks." desc="Making insulting, demeaning, or pejorative comments directed at a community member or group of people."/>
                        <Term term="Stereotyping or discrimination." desc="Characterizing anyone's personality or behavior on the basis of immutable identities or traits."/>
                        <Term term="Sexualization." desc="Behaving in a way that would generally be considered inappropriately intimate in the context or purpose of the community."/>
                        <Term term="Violating confidentiality." desc="Sharing or acting on someone's personal or private information without their permission."/>
                        <Term term="Endangerment." desc="Causing, encouraging, or threatening violence or other harm toward any person or group."/>
                        <EmphItem pre="Behaving in other ways that " strong="threaten the well-being" post=" of our community."/>
                    </ol>
                    <h3 class="mt-6 font-mono text-lg font-semibold text-ink">"Other Restrictions"</h3>
                    <ol class="list-decimal space-y-2 pl-6">
                        <Term term="Misleading identity." desc="Impersonating someone else for any reason, or pretending to be someone else to evade enforcement actions."/>
                        <Term term="Failing to credit sources." desc="Not properly crediting the sources of content you contribute."/>
                        <Term term="Promotional materials." desc="Sharing marketing or other commercial content in a way that is outside the norms of the community."/>
                        <Term term="Irresponsible communication." desc="Failing to responsibly present content which includes, links or describes any other restricted behaviors."/>
                    </ol>
                </Section>

                <Section title="Reporting an Issue">
                    <p>
                        "Tensions can occur between community members even when they are trying "
                        "their best to collaborate. Not every conflict represents a code of "
                        "conduct violation, and this Code of Conduct reinforces encouraged "
                        "behaviors and norms that can help avoid conflicts and minimize harm."
                    </p>
                    <p>
                        "When an incident does occur, it is important to report it promptly. To "
                        "report a possible violation, message an organizer or moderator directly "
                        "on our "
                        <a href=DISCORD_URL target="_blank" rel="noopener noreferrer" class="text-accent hover:text-accent-soft">
                            "Discord"
                        </a>
                        ". RISC-V Ottawa does not have a dedicated contact email yet, so a Discord "
                        "direct message is the primary confidential channel; for non-sensitive "
                        "matters you may instead open an issue in our "
                        <a href=GOVERNANCE_URL target="_blank" rel="noopener noreferrer" class="text-accent hover:text-accent-soft">
                            "governance repository"
                        </a>
                        "."
                    </p>
                    <p>
                        "The organizing team takes reports of violations seriously and will make "
                        "every effort to respond in a timely manner. They will investigate all "
                        "reports of code of conduct violations, reviewing messages, logs, and "
                        "recordings, or interviewing witnesses and other participants. The "
                        "organizing team will keep investigation and enforcement actions as "
                        "transparent as possible while prioritizing safety and confidentiality. In "
                        "order to honor these values, enforcement actions are carried out in "
                        "private with the involved parties, but communicating to the whole "
                        "community may be part of a mutually agreed upon resolution."
                    </p>
                </Section>

                <Section title="Addressing and Repairing Harm">
                    <p>
                        "Enforcement decisions are made by the organizing team, and their outcome "
                        "is recorded in an anonymized enforcement log."
                    </p>
                    <p>
                        "If an investigation by the organizing team finds that this Code of "
                        "Conduct has been violated, the following enforcement ladder may be used "
                        "to determine how best to repair harm, based on the incident's impact on "
                        "the individuals involved and the community as a whole. Depending on the "
                        "severity of a violation, lower rungs on the ladder may be skipped."
                    </p>
                    <Rung
                        name="1. Warning"
                        event="A violation involving a single incident or series of incidents."
                        consequence="A private, written warning from the organizing team."
                        repair="Examples of repair include a private written apology, acknowledgement of responsibility, and seeking clarification on expectations."
                    />
                    <Rung
                        name="2. Temporarily Limited Activities"
                        event="A repeated incidence of a violation that previously resulted in a warning, or the first incidence of a more serious violation."
                        consequence="A private, written warning with a time-limited cooldown period designed to underscore the seriousness of the situation and give the community members involved time to process the incident. The cooldown period may be limited to particular communication channels or interactions with particular community members."
                        repair="Examples of repair may include making an apology, using the cooldown period to reflect on actions and impact, and being thoughtful about re-entering community spaces after the period is over."
                    />
                    <Rung
                        name="3. Temporary Suspension"
                        event="A pattern of repeated violation which the organizing team has tried to address with warnings, or a single serious violation."
                        consequence="A private written warning with conditions for return from suspension. In general, temporary suspensions give the person being suspended time to reflect upon their behavior and possible corrective actions."
                        repair="Examples of repair include respecting the spirit of the suspension, meeting the specified conditions for return, and being thoughtful about how to reintegrate with the community when the suspension is lifted."
                    />
                    <Rung
                        name="4. Permanent Ban"
                        event="A pattern of repeated code of conduct violations that other steps on the ladder have failed to resolve, or a violation so serious that the organizing team determines there is no way to keep the community safe with this person as a member."
                        consequence="Access to all community spaces, tools, and communication channels is removed. In general, permanent bans should be rarely used, should have strong reasoning behind them, and should only be resorted to if working through other remedies has failed to change the behavior."
                        repair="There is no possible repair in cases of this severity."
                    />
                    <p>
                        "This enforcement ladder is intended as a guideline. It does not limit the "
                        "ability of the organizing team to use their discretion and judgment, in "
                        "keeping with the best interests of our community."
                    </p>
                </Section>

                <Section title="Scope">
                    <p>
                        "This Code of Conduct applies within all community spaces, and also "
                        "applies when an individual is officially representing the community in "
                        "public or other spaces. Examples of representing our community include "
                        "using an official email address, posting via an official social media "
                        "account, or acting as an appointed representative at an online or offline "
                        "event."
                    </p>
                </Section>

                <Section title="In-person events">
                    <p>
                        "At in-person events, everyone is additionally expected to follow the "
                        <a href="https://berlincodeofconduct.org" target="_blank" rel="noopener noreferrer" class="text-accent hover:text-accent-soft">
                            "Berlin Code of Conduct"
                        </a>
                        ", which covers conduct specific to physical gatherings such as talks, "
                        "workshops, and meetups."
                    </p>
                </Section>

                <Section title="Attribution">
                    <p>
                        "This Code of Conduct is adapted from the Contributor Covenant, version "
                        "3.0, permanently available at "
                        <a href="https://www.contributor-covenant.org/version/3/0/" target="_blank" rel="noopener noreferrer" class="text-accent hover:text-accent-soft">
                            "https://www.contributor-covenant.org/version/3/0/"
                        </a>
                        "."
                    </p>
                    <p>
                        "Contributor Covenant is stewarded by the Organization for Ethical Source "
                        "and licensed under CC BY-SA 4.0. To view a copy of this license, visit "
                        <a href="https://creativecommons.org/licenses/by-sa/4.0/" target="_blank" rel="noopener noreferrer" class="text-accent hover:text-accent-soft">
                            "https://creativecommons.org/licenses/by-sa/4.0/"
                        </a>
                    </p>
                    <p>
                        "For answers to common questions about Contributor Covenant, see the FAQ "
                        "at "
                        <a href="https://www.contributor-covenant.org/faq" target="_blank" rel="noopener noreferrer" class="text-accent hover:text-accent-soft">
                            "https://www.contributor-covenant.org/faq"
                        </a>
                        ". Translations are provided at "
                        <a href="https://www.contributor-covenant.org/translations" target="_blank" rel="noopener noreferrer" class="text-accent hover:text-accent-soft">
                            "https://www.contributor-covenant.org/translations"
                        </a>
                        ". Additional enforcement and community guideline resources can be found "
                        "at "
                        <a href="https://www.contributor-covenant.org/resources" target="_blank" rel="noopener noreferrer" class="text-accent hover:text-accent-soft">
                            "https://www.contributor-covenant.org/resources"
                        </a>
                        ". The enforcement ladder was inspired by the work of "
                        <a href="https://github.com/mozilla/inclusion" target="_blank" rel="noopener noreferrer" class="text-accent hover:text-accent-soft">
                            "Mozilla's code of conduct team"
                        </a>
                        "."
                    </p>
                </Section>
            </div>
        </section>
    }
}

#[component]
fn Section(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div>
            <h2 class="font-mono text-2xl font-semibold text-ink md:text-3xl">{title}</h2>
            <div class="mt-4 space-y-4 text-mute">{children()}</div>
        </div>
    }
}

#[component]
fn Term(term: &'static str, desc: &'static str) -> impl IntoView {
    view! {
        <li>
            <strong class="font-semibold text-ink">{term}</strong>
            " "
            {desc}
        </li>
    }
}

#[component]
fn EmphItem(pre: &'static str, strong: &'static str, post: &'static str) -> impl IntoView {
    view! {
        <li>
            {pre}
            <strong class="font-semibold text-ink">{strong}</strong>
            {post}
        </li>
    }
}

#[component]
fn Rung(
    name: &'static str,
    event: &'static str,
    consequence: &'static str,
    repair: &'static str,
) -> impl IntoView {
    view! {
        <div class="rounded-sm border border-line bg-surface p-5">
            <h3 class="font-mono text-lg font-semibold text-ink">{name}</h3>
            <dl class="mt-3 space-y-2 text-sm">
                <div>
                    <dt class="inline text-accent">"Event: "</dt>
                    <dd class="inline text-mute">{event}</dd>
                </div>
                <div>
                    <dt class="inline text-accent">"Consequence: "</dt>
                    <dd class="inline text-mute">{consequence}</dd>
                </div>
                <div>
                    <dt class="inline text-accent">"Repair: "</dt>
                    <dd class="inline text-mute">{repair}</dd>
                </div>
            </dl>
        </div>
    }
}
