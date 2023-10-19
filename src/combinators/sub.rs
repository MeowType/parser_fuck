use libsugar::*;

use crate::*;
use std::ops::Range;

/// Match subpart
/// ## example
/// ```
/// # use parser_fuck::*;
/// let code = "asd123".span();
/// let x = sub("asd".chars());
/// let r = x.parse(code);
/// assert_eq!(r, Some(0..3))
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Sub<T> {
    sub: Vec<T>,
}
impl<T> Sub<T> {
    #[inline]
    pub const fn new(sub: Vec<T>) -> Self {
        Self { sub }
    }
}
impl<T, I: IntoIterator<Item = T>> From<I> for Sub<T> {
    #[inline]
    fn from(c: I) -> Self {
        Self::new(c.into_iter().collect())
    }
}
impl<I: TimeTravel, T> Parser<I> for Sub<T>
where
    I::Item: PartialEq<T>,
{
    type Output = Range<usize>;

    fn parse(&self, mut input: I) -> Option<Self::Output> {
        let from = input.save();
        let pos: Option<usize> = {
            let mut siter = self.sub.iter();
            let mut i: usize = 0;
            loop {
                let io = input.next();
                let so = siter.next();

                bop!(!loop match && Some(a) = io, Some(b) = so => {
                    if PartialEq::<T>::ne(&a, b) {
                        break Some(i);
                    }
                } else {
                    break None;
                });

                i += 1;
            }
        };
        // let pos = input
        //     .ref_clone()
        //     .zip(self.sub.iter())
        //     .position(|(a, b)| PartialEq::<T>::ne(&a, b));
        if let None = pos {
            let pos = input.save() - from;
            if self.sub.len() + 1 == pos {
                return Some(from..input.save() - 1);
            }
            return if input.is_complete() {
                if pos == self.sub.len() {
                    Some(input.make_range(from))
                } else {
                    None
                }
            } else {
                None
            };
        }
        None
    }
}

/// Match subpart
/// ## example
/// ```
/// # use parser_fuck::*;
/// let code = "asd123".span();
/// let x = sub("asd".chars());
/// let r = x.parse(code);
/// assert_eq!(r, Some(0..3))
/// ```
#[inline]
pub fn sub<T, I: IntoIterator<Item = T>>(c: I) -> Sub<T> {
    Sub::from(c)
}

/// Match substring
/// ## example
/// ```
/// # use parser_fuck::*;
/// let code = "asd123".span();
/// let x = substr("asd");
/// let r = x.parse(code);
/// assert_eq!(r, Some(0..3))
/// ```
#[inline]
pub fn substr(c: &str) -> Sub<char> {
    Sub::from(c.chars())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "asd";
        let span = code.span();
        let x = substr("asd");

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(0..3));
    }

    #[test]
    fn test_empty_code() {
        let code = "";
        let span = code.span();
        let x = substr("asd");

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, None);
    }

    #[test]
    fn test_empty() {
        let code = "asd";
        let span = code.span();
        let x = substr("");

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(0..0));
    }

    #[test]
    fn test_one() {
        let code = "asd";
        let span = code.span();
        let x = substr("a");

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(0..1));
    }

    #[test]
    fn test_partial() {
        let code = "asdqwe";
        let span = code.span();
        let x = substr("asd");

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(0..3));
    }

    #[test]
    fn test_more() {
        let code = "asd";
        let span = code.span();
        let x = substr("asdqwe");

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, None);
    }

    #[test]
    fn test_middle() {
        let code = "asd";
        let mut span = code.span();
        let x = substr("s");

        span.next();
        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(1..2));
    }
}
