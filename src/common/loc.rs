use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Loc {
    pub offset: usize,
    pub line: usize,
    pub char: usize,
}
impl Loc {
    #[inline]
    pub const fn new() -> Self {
        Self {
            offset: 0,
            line: 0,
            char: 0,
        }
    }
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

pub const fn loc_of(offset: usize, line: usize, char: usize) -> Loc {
    Loc::new_at(offset, line, char)
}

//\/////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct LocRange {
    pub from: Loc,
    pub to: Loc,
}
impl LocRange {
    #[inline]
    pub const fn new(from: Loc, to: Loc) -> Self {
        Self { from, to }
    }
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

#[inline]
pub const fn loc_range_of(from: Loc, to: Loc) -> LocRange {
    LocRange::new(from, to)
}

//\/////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ComLoc {
    type ComLocData;

    fn loc(&self, data: Self::ComLocData) -> Option<Loc>;
}
pub trait ComLocRange {
    type ComLocRangeData;

    fn loc_range(&self, data: Self::ComLocRangeData) -> Option<LocRange>;
}

pub trait GetLoc {
    fn loc(&self) -> Loc;
}
pub trait GetLocRange {
    fn loc_range(&self) -> LocRange;
}
