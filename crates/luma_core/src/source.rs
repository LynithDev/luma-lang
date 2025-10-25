use std::{hash::Hash, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeSourceKind {
    File(PathBuf),
    Virtual,
}

impl CodeSourceKind {
    pub fn source_name(&self) -> String {
        if let CodeSourceKind::File(path) = &self {
            path.to_string_lossy().to_string()
        } else {
            "<virtual>".to_string()
        }
    }
}

impl std::fmt::Display for CodeSourceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeSourceKind::File(path) => write!(f, "{}", path.display()),
            CodeSourceKind::Virtual => write!(f, "<virtual>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeSource {
    pub(crate) kind: CodeSourceKind,
    pub(crate) source: String,
}

impl Hash for CodeSource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.kind {
            CodeSourceKind::File(ref path) => path.hash(state),
            CodeSourceKind::Virtual => self.source.hash(state),
        }
    }
}

impl CodeSource {
    pub fn source_name(&self) -> String {
        self.kind.source_name()
    }

    pub fn kind(&self) -> &CodeSourceKind {
        &self.kind
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

impl TryFrom<PathBuf> for CodeSource {
    type Error = std::io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let source = std::fs::read_to_string(&path)?;
        Ok(Self {
            source,
            kind: CodeSourceKind::File(path),
        })
    }
}

impl From<&str> for CodeSource {
    fn from(s: &str) -> Self {
        Self {
            kind: CodeSourceKind::Virtual,
            source: s.trim().to_string(),
        }
    }
}

impl From<String> for CodeSource {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}