use crate::{CodeSource, CodeSourceId};

#[derive(Default, Debug)]
pub struct SourceManager {
    sources: Vec<CodeSource>,
}

impl SourceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_source(&mut self, source: CodeSource) -> CodeSourceId {
        let id = CodeSourceId::new(self.sources.len() as u32);
        self.sources.push(source);
        id
    }

    pub fn get_source(&self, id: CodeSourceId) -> Option<&CodeSource> {
        self.sources.get(*id as usize)
    }

    pub fn get_sources(&self) -> &[CodeSource] {
        &self.sources
    }
}