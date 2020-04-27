use crate::*;
use std::ops::Range;

/// Match an item
/// ## example
/// ```
/// # use parser_fuck::*;
/// let code = "asd".span();
/// let a = one('a');
/// let r = a.parse(code);
/// assert_eq!(r, Some(0..1))
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct One<T> {
    val: T,
}
impl<T> One<T> {
    #[inline]
    pub const fn new(val: T) -> Self {
        Self { val }
    }
}
impl<I: TimeTravel, T> Parser<I> for One<T>
where
    I::Item: PartialEq<T>,
{
    type Output = Range<usize>;

    fn parse(&self, mut input: I) -> Option<Self::Output> {
        let from = input.save();
        let a = input.next()?;
        input.do_ready();
        if PartialEq::<T>::eq(&a, &self.val) {
            Some(input.make_range(from))
        } else {
            None
        }
    }
}

/// Match an item
/// ## example
/// ```
/// # use parser_fuck::*;
/// let code = "asd".span();
/// let a = one('a');
/// let r = a.parse(code);
/// assert_eq!(r, Some(0..1))
/// ```
#[inline]
pub const fn one<T>(val: T) -> One<T> {
    One::new(val)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "asd";
        let span = code.span();
        let a = one('a');

        let r = a.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(0..1))
    }

    #[test]
    fn test_none() {
        let code = "asd";
        let span = code.span();
        let a = one('b');

        let r = a.parse(span);
        println!("{:?}", r);
        assert_eq!(r, None)
    }

    #[test]
    fn test_empty() {
        let code = "";
        let span = code.span();
        let a = one('a');

        let r = a.parse(span);
        println!("{:?}", r);
        assert_eq!(r, None)
    }

    #[test]
    fn test_multi() {
        let code = "asd";
        let span = code.span();
        let a = one('a');
        let b = one('s');

        let r = a.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(0..1));
        let r = b.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(1..2));
    }
}
