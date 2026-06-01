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
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "content I/O error: {e}"),
            LoadError::Parse { path, error } => {
                write!(f, "failed to parse {}: {error}", path.display())
            }
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadError::Io(e) => Some(e),
            LoadError::Parse { error, .. } => Some(error),
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
    let mut out = Vec::new();
    for (slug, contents, path) in read_toml_dir(dir)? {
        let mut event: Event = toml::from_str(&contents).map_err(|error| LoadError::Parse {
            path: path.clone(),
            error,
        })?;
        event.slug = slug;
        out.push(event);
    }
    Ok(out)
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
