use std::{fmt::{Debug, Display}, ops::{Add, Deref, DerefMut}};

// MARK: Span
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    
    #[must_use]
    pub const fn len(&self) -> usize {
        self.end - self.start
    }
    
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    pub fn merge(&mut self, other: &Span) -> &mut Self {
        *self = Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        };
        self
    }

    pub fn maybe_merge(&mut self, other: &Option<Span>) -> &mut Self {
        if let Some(other_span) = other {
            self.merge(other_span);
        }
        self
    }

    #[must_use]
    pub fn merged(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    #[must_use]
    pub fn maybe_merged(&self, other: &Option<Span>) -> Span {
        if let Some(other_span) = other {
            self.merged(other_span)
        } else {
            *self
        }
    }
    
    #[must_use]
    pub fn contains(&self, pos: usize) -> bool {
        self.start <= pos && pos < self.end
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Span({}..={})", self.start, self.end)
    }
}

impl Add for Span {
    type Output = Span;
    
    fn add(self, other: Span) -> Span {
        Span {
            start: self.start,
            end: other.end,
        }
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(range: std::ops::Range<usize>) -> Self {
        Span {
            start: range.start,
            end: range.end,
        }
    }
}


// MARK: Spanned
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spanned<T> {
    pub item: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    #[must_use]
    #[allow(clippy::self_named_constructors)]
    pub fn spanned<S: Into<Span>>(span: S, item: T) -> Self {
        Self { span: span.into(), item }
    }

    pub fn try_map_inner<I>(self) -> Result<Spanned<I>, I::Error>
    where
        I: TryFrom<T>,
    {
        Ok(Spanned {
            item: I::try_from(self.item)?,
            span: self.span,
        })
    }

    pub fn map_inner<I>(self) -> Spanned<I>
    where
        I: From<T>,
    {
        Spanned {
            item: I::from(self.item),
            span: self.span,
        }
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

// MARK: MaybeSpanned
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaybeSpanned<T> {
    pub item: T,
    pub span: Option<Span>,
}

impl<T> Deref for MaybeSpanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T> DerefMut for MaybeSpanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl<T: Default> Default for MaybeSpanned<T> {
    fn default() -> Self {
        Self {
            item: T::default(),
            span: None,
        }
    }
}

impl<T> MaybeSpanned<T> {
    #[must_use]
    pub fn new<S: Into<Option<Span>>>(span: S, item: T) -> Self {
        Self { span: span.into(), item }
    }

    #[must_use]
    pub fn spanned<S: Into<Span>>(span: S, item: T) -> Self {
        Self::new(Some(span.into()), item)
    }
    
    #[must_use]
    pub fn unspanned(item: T) -> Self {
        Self::new(None, item)
    }

    pub fn try_map_inner<I>(self) -> Result<MaybeSpanned<I>, I::Error>
    where
        I: TryFrom<T>,
    {
        Ok(MaybeSpanned {
            item: I::try_from(self.item)?,
            span: self.span,
        })
    }

    pub fn map_inner<I>(self) -> MaybeSpanned<I>
    where
        I: From<T>,
    {
        MaybeSpanned {
            item: I::from(self.item),
            span: self.span,
        }
    }
}