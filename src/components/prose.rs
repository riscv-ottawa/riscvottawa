use leptos::prelude::*;

/// Splits a content-file string into paragraphs on blank lines.
///
/// A blank line starts a new paragraph. A single newline is only how the TOML
/// happens to be wrapped, so it collapses to a space, which is what HTML would
/// have done with it anyway. That means a long line can be hard-wrapped in the
/// content file for readability without changing what the page shows, while a
/// deliberate break still survives.
pub fn paragraphs(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Vec<&str> = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !current.is_empty() {
                out.push(current.join(" "));
                current.clear();
            }
        } else {
            current.push(line);
        }
    }
    if !current.is_empty() {
        out.push(current.join(" "));
    }
    out
}

/// The same prose flattened onto one line, for the places that can hold text
/// but not markup: `og:description`, an `alt`, a `title` attribute.
pub fn single_line(text: &str) -> String {
    paragraphs(text).join(" ")
}

/// One `<p>` per paragraph, so prose in a content file can run to more than one
/// without the blank line between them disappearing into HTML whitespace.
///
/// Every paragraph carries the same class, so whatever top margin separates the
/// block from what precedes it also separates the paragraphs from each other.
/// Where a block has no margin of its own, pass `mt-* first:mt-0`.
///
/// `class` is `&'static str` on purpose: Tailwind only emits a utility it can
/// find as a literal in the source, so the classes have to be written out at
/// each call site rather than assembled here.
#[component]
pub fn Prose(#[prop(into)] text: String, class: &'static str) -> impl IntoView {
    paragraphs(&text)
        .into_iter()
        .map(|p| view! { <p class=class>{p}</p> })
        .collect_view()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_lines_split_and_soft_wraps_join() {
        let text = "First line\nwrapped in the file.\n\nA second paragraph.";
        assert_eq!(
            paragraphs(text),
            vec!["First line wrapped in the file.", "A second paragraph."]
        );
    }

    #[test]
    fn runs_of_blank_lines_and_stray_whitespace_make_one_break() {
        // What you get from a TOML `"""` block that wraps its delimiters onto
        // their own lines, or from writing `\n\n` by hand inside one.
        assert_eq!(paragraphs("\nOne.\n \n\n\nTwo.\n"), vec!["One.", "Two."]);
    }

    #[test]
    fn ordinary_single_line_text_is_unchanged() {
        assert_eq!(paragraphs("Just a sentence."), vec!["Just a sentence."]);
        assert!(paragraphs("   ").is_empty());
    }

    #[test]
    fn single_line_rejoins_for_attributes() {
        assert_eq!(single_line("One.\n\nTwo."), "One. Two.");
    }
}
