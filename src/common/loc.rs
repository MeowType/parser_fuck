use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::ops::{Range, RangeTo};

/// Location in source code
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Loc {
    /// nth of characters
    pub offset: usize,
    /// nth of line
    pub line: usize,
    /// nth of characters in current line
    pub char: usize,
}
impl Loc {
    /// New empty
    #[inline]
    pub const fn new() -> Self {
        Self {
            offset: 0,
            line: 0,
            char: 0,
        }
    }
    /// New at
    #[inline]
    pub const fn new_at(offset: usize, line: usize, char: usize) -> Self {
        Self { offset, line, char }
    }
}
impl Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Loc {{ {}:{}({}) }}", self.line, self.char, self.offset)
    }
}
impl Default for Loc {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl From<(usize, usize, usize)> for Loc {
    #[inline]
    fn from((offset, line, char): (usize, usize, usize)) -> Self {
        Self::new_at(offset, line, char)
    }
}
impl From<()> for Loc {
    #[inline]
    fn from(_: ()) -> Self {
        Self::new()
    }
}

/// Shorthand for Loc::new_at
pub const fn loc_of(offset: usize, line: usize, char: usize) -> Loc {
    Loc::new_at(offset, line, char)
}

//\/////////////////////////////////////////////////////////////////////////////////////////////////

/// Range of Location in source code
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LocRange {
    /// from
    pub from: Loc,
    /// to
    pub to: Loc,
}
impl LocRange {
    /// New at
    #[inline]
    pub const fn new(from: Loc, to: Loc) -> Self {
        Self { from, to }
    }
    /// New empty
    #[inline]
    pub const fn new_empty() -> Self {
        Self {
            from: Loc::new(),
            to: Loc::new(),
        }
    }
}
impl Display for LocRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LocRange {{ {} .. {} }}", self.from, self.to)
    }
}
impl Default for LocRange {
    #[inline]
    fn default() -> Self {
        Self::new_empty()
    }
}
impl From<(usize, usize, usize, usize, usize, usize)> for LocRange {
    #[inline]
    fn from((a1, b1, c1, a2, b2, c2): (usize, usize, usize, usize, usize, usize)) -> Self {
        Self::new(Loc::new_at(a1, b1, c1), Loc::new_at(a2, b2, c2))
    }
}
impl From<(usize, usize, usize)> for LocRange {
    #[inline]
    fn from((offset, line, char): (usize, usize, usize)) -> Self {
        Self::new(
            Loc::new_at(offset, line, char),
            Loc::new_at(offset, line, char),
        )
    }
}
impl From<()> for LocRange {
    #[inline]
    fn from(_: ()) -> Self {
        Self::new_empty()
    }
}
impl From<Range<Loc>> for LocRange {
    #[inline]
    fn from(r: Range<Loc>) -> Self {
        Self::new(r.start, r.end)
    }
}
impl From<RangeTo<Loc>> for LocRange {
    #[inline]
    fn from(r: RangeTo<Loc>) -> Self {
        Self::new(Loc::new(), r.end)
    }
}
impl From<(Loc, Loc)> for LocRange {
    #[inline]
    fn from((from, to): (Loc, Loc)) -> Self {
        Self::new(from, to)
    }
}

/// Shorthand for LocRange::new
#[inline]
pub const fn loc_range_of(from: Loc, to: Loc) -> LocRange {
    LocRange::new(from, to)
}

//\/////////////////////////////////////////////////////////////////////////////////////////////////

/// Calculate Loc
pub trait ComLoc {
    type ComLocData;

    /// Calculate Loc
    fn loc(&self, data: Self::ComLocData) -> Option<Loc>;
}
/// Calculate LocRange
pub trait ComLocRange {
    type ComLocRangeData;

    /// Calculate ComLocRange
    fn loc_range(&self, data: Self::ComLocRangeData) -> Option<LocRange>;
}

/// Get Loc
pub trait GetLoc {
    /// Get Loc
    fn loc(&self) -> Loc;
}
/// Get LocRange
pub trait GetLocRange {
    /// Get LocRange
    fn loc_range(&self) -> LocRange;
}
