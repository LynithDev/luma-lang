use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CodeSource {
    pub content: String,
    pub file_path: Option<String>,
}

impl CodeSource {
    pub fn new(content: String, file_path: Option<String>) -> Self {
        Self { content, file_path }
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