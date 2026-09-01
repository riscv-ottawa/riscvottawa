use super::{Event, Project, ResourceSection};
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ContentStore {
    pub events: Arc<[Event]>,
    pub projects: Arc<[Project]>,
    pub resources: Arc<[ResourceSection]>,
}

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Parse {
        path: PathBuf,
        error: toml::de::Error,
    },
    Invalid {
        path: PathBuf,
        message: String,
    },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "content I/O error: {e}"),
            LoadError::Parse { path, error } => {
                write!(f, "failed to parse {}: {error}", path.display())
            }
            LoadError::Invalid { path, message } => {
                write!(f, "{}: {message}", path.display())
            }
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadError::Io(e) => Some(e),
            LoadError::Parse { error, .. } => Some(error),
            LoadError::Invalid { .. } => None,
        }
    }
}

impl From<io::Error> for LoadError {
    fn from(e: io::Error) -> Self {
        LoadError::Io(e)
    }
}

impl ContentStore {
    pub fn load_from_dir(root: impl AsRef<Path>) -> Result<Self, LoadError> {
        let root = root.as_ref();
        let mut events = load_events(&root.join("events"))?;
        let mut projects = load_projects(&root.join("projects"))?;
        let resources = load_resources(&root.join("resources"))?;
        events.sort_by_key(|e| e.date);
        projects.sort_by(|a, b| a.slug.cmp(&b.slug));
        Ok(Self {
            events: events.into(),
            projects: projects.into(),
            resources: resources.into(),
        })
    }
}

fn load_events(dir: &Path) -> Result<Vec<Event>, LoadError> {
    let mut out: Vec<Event> = Vec::new();
    let mut seen: Vec<(String, PathBuf)> = Vec::new();

    for (stem, contents, path) in read_toml_dir(dir)? {
        let mut event: Event = toml::from_str(&contents).map_err(|error| LoadError::Parse {
            path: path.clone(),
            error,
        })?;

        // The file name is only a default. Anything with a link out in the world
        // should pin its own slug, so renaming the file doesn't move the URL.
        if event.slug.trim().is_empty() {
            event.slug = stem;
        }
        let slug = event.slug.trim().to_string();

        if !is_url_safe(&slug) {
            return Err(LoadError::Invalid {
                path,
                message: format!(
                    "slug `{slug}` must be non-empty and use only letters, digits, `-` and `_`"
                ),
            });
        }
        if let Some((_, first)) = seen.iter().find(|(s, _)| *s == slug) {
            return Err(LoadError::Invalid {
                path,
                message: format!("slug `{slug}` is already used by {}", first.display()),
            });
        }

        seen.push((slug.clone(), path));
        event.slug = slug;
        out.push(event);
    }
    Ok(out)
}

fn is_url_safe(slug: &str) -> bool {
    !slug.is_empty()
        && slug
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn load_resources(dir: &Path) -> Result<Vec<ResourceSection>, LoadError> {
    let mut out = Vec::new();
    for (raw_slug, contents, path) in read_toml_dir(dir)? {
        let mut section: ResourceSection =
            toml::from_str(&contents).map_err(|error| LoadError::Parse {
                path: path.clone(),
                error,
            })?;
        section.slug = strip_order_prefix(&raw_slug);
        out.push(section);
    }
    Ok(out)
}

fn strip_order_prefix(slug: &str) -> String {
    if let Some((head, tail)) = slug.split_once('-') {
        if !head.is_empty() && head.chars().all(|c| c.is_ascii_digit()) {
            return tail.to_string();
        }
    }
    slug.to_string()
}

fn load_projects(dir: &Path) -> Result<Vec<Project>, LoadError> {
    let mut out = Vec::new();
    for (slug, contents, path) in read_toml_dir(dir)? {
        let mut project: Project = toml::from_str(&contents).map_err(|error| LoadError::Parse {
            path: path.clone(),
            error,
        })?;
        project.slug = slug;
        out.push(project);
    }
    Ok(out)
}

fn read_toml_dir(dir: &Path) -> io::Result<Vec<(String, String, PathBuf)>> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }
        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        let contents = fs::read_to_string(&path)?;
        out.push((slug, contents, path));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shipped content parses. Without this the first sign of a broken file
    /// is the server refusing to start, which is a slow way to find a typo.
    #[test]
    fn the_real_content_directory_loads() {
        let store = ContentStore::load_from_dir("content").expect("content/ parses");
        assert!(!store.events.is_empty());
        assert!(!store.projects.is_empty());
        assert!(!store.resources.is_empty());

        // Spotlight blocks nest several tables deep, so check one all the way
        // down rather than just that the file parsed.
        if let Some(ev) = store.events.iter().find(|e| e.spotlight.is_some()) {
            let s = ev.spotlight.as_ref().expect("checked above");
            assert!(!s.headline.is_empty());
            assert!(!s.draws.is_empty());
            assert!(!s.schedule.is_empty());
            assert!(!s.call_to_build.is_empty());
            assert!(s.panel.as_ref().is_some_and(|p| !p.panelists.is_empty()));
        }
    }

    /// A `photo` pointing at a file that isn't there renders as a broken image
    /// on a public page and nothing else complains, so check the paths resolve.
    #[test]
    fn every_speaker_photo_exists() {
        let store = ContentStore::load_from_dir("content").expect("content/ parses");
        let mut checked = 0;

        for event in store.events.iter() {
            let Some(spotlight) = &event.spotlight else {
                continue;
            };
            let panel_people = spotlight
                .panel
                .iter()
                .flat_map(|p| std::iter::once(&p.moderator).chain(p.panelists.iter()));

            for speaker in spotlight.speakers.iter().chain(panel_people) {
                let Some(photo) = &speaker.photo else {
                    continue;
                };
                assert!(
                    photo.starts_with('/'),
                    "{}: photo `{photo}` must be a site-root path like `/speakers/name.jpg`",
                    speaker.display_name()
                );
                let path = Path::new("public").join(photo.trim_start_matches('/'));
                assert!(
                    path.is_file(),
                    "{}: photo `{photo}` has no file at {}",
                    speaker.display_name(),
                    path.display()
                );
                checked += 1;
            }
        }
        println!("checked {checked} speaker photos");
    }
}
