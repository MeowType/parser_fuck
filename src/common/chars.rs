use super::*;
use batch_oper::effect;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::string::ToString;

/// Characters and newlines with location
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Char {
    /// char and Loc
    Char(char, Loc),
    /// wrap `'\n'` | `'\r'` | `"\r\n"` and Loc
    Wrap(Loc),
}
impl Char {
    /// map char  
    /// None if it‘s not `Char`
    #[inline]
    pub fn char<R>(&self, mut f: impl FnMut(char, Loc) -> R) -> Option<R> {
        match self {
            Char::Char(c, l) => Some(f(*c, *l)),
            _ => None,
        }
    }
    /// map wrap  
    /// None if it’s not `Wrap`
    #[inline]
    pub fn wrap<R>(&self, mut f: impl FnMut(Loc) -> R) -> Option<R> {
        match self {
            Char::Wrap(l) => Some(f(*l)),
            _ => None,
        }
    }
    /// is `Char`
    #[inline]
    pub fn is_char(&self) -> bool {
        matches!(self, Char::Char(_, _))
    }
    /// is `Wrap`
    #[inline]
    pub fn is_wrap(&self) -> bool {
        matches!(self, Char::Wrap(_))
    }

    /// just equivalent to eq, but ignore Loc
    #[inline]
    pub fn char_eq(&self, other: &Self) -> bool {
        match self {
            Char::Char(c, _) => {
                if let Char::Char(oc, _) = other {
                    c == oc
                } else {
                    false
                }
            }
            Char::Wrap(_) => matches!(other, Char::Wrap(_)),
        }
    }
    /// just equivalent to ne, but ignore Loc
    #[inline]
    pub fn char_ne(&self, other: &Self) -> bool {
        !self.char_eq(other)
    }
    /// Get the char  
    /// when it is Wrap return `'\n'`
    #[inline]
    pub fn c(&self) -> char {
        match self {
            Char::Char(c, _) => *c,
            Char::Wrap(_) => '\n',
        }
    }
}
impl GetLoc for Char {
    #[inline]
    fn loc(&self) -> Loc {
        match self {
            Char::Char(_, l) => *l,
            Char::Wrap(l) => *l,
        }
    }
}
impl Display for Char {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Char::Char(c, _) => write!(f, "Char({})", c),
            Char::Wrap(_) => write!(f, "Wrap"),
        }
    }
}
impl PartialEq<char> for Char {
    /// Checks if a char is equal to it  
    /// when it is Wrap check eq to `'\n'`
    fn eq(&self, other: &char) -> bool {
        match self {
            Char::Char(c, _) => *c == *other,
            Char::Wrap(_) => '\n' == *other,
        }
    }
}
impl Into<char> for Char {
    #[inline]
    fn into(self) -> char {
        self.c()
    }
}
impl GetString for Char {
    #[inline]
    fn get_string(&self) -> String {
        self.c().to_string()
    }
}
impl GetChar for Char {
    #[inline]
    fn get_char(&self) -> char {
        self.c()
    }
}

//\/////////////////////////////////////////////////////////////////////////////////////////////////

/// An iterator that produces [`Char`](enum.Char.html)
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct CharChars<I: Iterator<Item = char>> {
    iter: I,
    loc: Loc,
    isr: Option<Loc>,
    end: bool,
    nextret: Option<Char>,
}
impl<I: Iterator<Item = char>> CharChars<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            loc: Loc::new(),
            isr: None,
            end: false,
            nextret: None,
        }
    }
}
impl<I: Iterator<Item = char>> From<I> for CharChars<I> {
    #[inline]
    fn from(iter: I) -> Self {
        Self::new(iter)
    }
}
impl<I: Iterator<Item = char>> Iterator for CharChars<I> {
    type Item = Char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(r) = self.nextret {
            self.nextret = None;
            return Some(r);
        }
        if self.end {
            return None;
        }
        loop {
            let n = self.iter.next();
            if let Some(c) = n {
                if let Some(l) = self.isr {
                    if c == '\n' {
                        self.isr = None;
                        return effect(Some(Char::Wrap(l)), |_| {
                            self.loc.offset += 1;
                        });
                    } else {
                        if c == '\r' {
                            self.isr = Some(self.loc);
                            self.loc.offset += 1;
                            self.loc.char = 0;
                            self.loc.line += 1;
                            return Some(Char::Wrap(l));
                        } else {
                            self.isr = None;
                            self.nextret = Some(Char::Char(c, self.loc));
                            self.loc.offset += 1;
                            self.loc.char += 1;
                            return Some(Char::Wrap(l));
                        }
                    }
                } else {
                    if c == '\r' {
                        self.isr = Some(self.loc);
                        self.loc.offset += 1;
                        self.loc.char = 0;
                        self.loc.line += 1;
                    } else if c == '\n' {
                        return effect(Some(Char::Wrap(self.loc)), |_| {
                            self.loc.offset += 1;
                            self.loc.char = 0;
                            self.loc.line += 1;
                        });
                    } else {
                        return effect(Some(Char::Char(c, self.loc)), |_| {
                            self.loc.offset += 1;
                            self.loc.char += 1;
                        });
                    }
                }
            } else {
                if let Some(l) = self.isr {
                    self.end = true;
                    return Some(Char::Wrap(l));
                } else {
                    self.end = true;
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Char, CharChars, Loc};

    #[test]
    fn test_char_chars() {
        let code = "a\nb\rc\r\nd";
        let mut cc = CharChars::new(code.chars());

        assert_eq!(cc.next(), Some(Char::Char('a', Loc::new_at(0, 0, 0))));
        assert_eq!(cc.next(), Some(Char::Wrap(Loc::new_at(1, 0, 1))));
        assert_eq!(cc.next(), Some(Char::Char('b', Loc::new_at(2, 1, 0))));
        assert_eq!(cc.next(), Some(Char::Wrap(Loc::new_at(3, 1, 1))));
        assert_eq!(cc.next(), Some(Char::Char('c', Loc::new_at(4, 2, 0))));
        assert_eq!(cc.next(), Some(Char::Wrap(Loc::new_at(5, 2, 1))));
        assert_eq!(cc.next(), Some(Char::Char('d', Loc::new_at(7, 3, 0))));
        assert_eq!(cc.next(), None);
    }
}
