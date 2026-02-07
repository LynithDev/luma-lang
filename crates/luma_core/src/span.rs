use crate::CodeSourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub source_id: CodeSourceId,
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(source_id: CodeSourceId, start: u32, end: u32) -> Self {
        Self {
            source_id,
            start,
            end,
        }
    }

    pub fn void() -> Self {
        Self {
            source_id: CodeSourceId::void(),
            start: 0,
            end: 0,
        }
    }

    pub fn merge(&mut self, other: &Span) {
        self.start = self.start.min(other.start);
        self.end = self.end.max(other.end);
    }

    pub fn merged(&self, other: &Span) -> Span {
        Span {
            source_id: self.source_id,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn maybe_merge(&mut self, other: Option<&Span>) {
        if let Some(other) = other {
            self.merge(other);
        }
    }

    pub fn maybe_merged(&self, other: Option<&Span>) -> Span {
        if let Some(other) = other {
            self.merged(other)
        } else {
            *self
        }
    }

    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> u32 {
        self.end - self.start
    }

    pub fn as_range(&self) -> std::ops::Range<usize> {
        self.start as usize..self.end as usize
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}