pub mod manager;

use std::{ops::Deref, path::PathBuf};

#[derive(Debug, Clone)]
pub struct CodeSource {
    pub content: String,
    pub file_path: Option<String>,
}

/// A unique identifier for a code source. Managed by the [`SourceManager`](manager::SourceManager)
/// This supports a maximum of (2^32)-1 sources, which should be more than enough for any project.
/// 
/// Note: A CodeSourceId of `u32::MAX` (4294967295) is reserved. Used to represent the absence of a source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeSourceId(u32);

impl CodeSource {
    pub fn new(content: String, file_path: Option<String>) -> Self {
        Self { content, file_path }
    }

    pub fn void() -> Self {
        Self {
            content: String::new(),
            file_path: None,
        }
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.content.len()
    }

    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.file_path.as_deref().and_then(|path| {
            std::path::Path::new(path)
                .file_name()
                .and_then(|name| name.to_str())
        })
    }

    #[must_use]
    pub fn source_file(&self) -> &str {
        self.file_path.as_deref().unwrap_or("<virtual>")
    }

    #[must_use]
    pub const fn is_file(&self) -> bool {
        self.file_path.is_some()
    }
}

impl From<String> for CodeSource {
    fn from(s: String) -> Self {
        CodeSource::new(s, None)
    }
}

impl From<&str> for CodeSource {
    fn from(s: &str) -> Self {
        CodeSource::new(s.to_string(), None)
    }
}

impl TryFrom<PathBuf> for CodeSource {
    type Error = std::io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let file_path = path.to_str().map(|s| s.to_string());
        Ok(CodeSource::new(content, file_path))
    }
}

impl CodeSourceId {
    pub const ZERO: CodeSourceId = CodeSourceId::new(0);

    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn value(&self) -> u32 {
        self.0
    }
}

impl From<u32> for CodeSourceId {
    fn from(id: u32) -> Self {
        CodeSourceId::new(id)
    }
}

impl From<CodeSourceId> for u32 {
    fn from(id: CodeSourceId) -> Self {
        *id
    }
}

impl Deref for CodeSourceId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}