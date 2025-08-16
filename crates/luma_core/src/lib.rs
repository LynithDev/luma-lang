mod input;
pub use input::*;

pub mod ast;

pub use luma_macros::Display;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn merge(&self, other: &Span) -> Self {
        Self {
            start: self.start,
            end: other.end,
        }
    }

    pub fn merge_all(&self, others: &[Option<Span>]) -> Self {
        let mut start = self.start;
        let mut end = self.end;

        for span in others.iter().flatten() {
            if span.start < start {
                start = span.start;
            }

            if span.end > end {
                end = span.end;
            }
        }

        Self { start, end }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NumberRadix {
    Hexadecimal = 16, // 16 - x
    #[default]
    Decimal = 10, // 10 - d (or none)
    Octal = 8, // 8 - o
    Binary = 2, // 2 - b
}

impl NumberRadix {
    pub fn is_radix_char(c: char) -> bool {
        matches!(c.to_ascii_lowercase(), 'x' | 'd' | 'o' | 'b')
    }
}

impl TryFrom<char> for NumberRadix {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'x' => Ok(NumberRadix::Hexadecimal),
            'd' => Ok(NumberRadix::Decimal),
            'o' => Ok(NumberRadix::Octal),
            'b' => Ok(NumberRadix::Binary),
            _ => Err(()),
        }
    }
}
