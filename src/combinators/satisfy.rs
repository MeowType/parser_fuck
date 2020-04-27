use crate::common::cell::*;
use crate::*;
use std::marker::PhantomData;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Satisfy<F, I = ()> {
    f: ExtRefCell<F>,
    _i: PhantomData<I>,
}
impl<I, F> Satisfy<F, I>
where
    I: TimeTravel,
    F: FnMut(I::Item) -> bool,
{
    #[inline]
    pub fn new(f: F) -> Self {
        Self {
            f: ExtRefCell::new(f),
            _i: PhantomData,
        }
    }
}
impl<I, F> Parser<I> for Satisfy<F, I>
where
    I: TimeTravel,
    F: FnMut(I::Item) -> bool,
{
    type Output = Range<usize>;

    fn parse(&self, mut input: I) -> Option<Self::Output> {
        let from = input.save();
        let n: I::Item = input.next()?;
        input.do_ready();
        let f = unsafe { self.f.get_mut() };
        let r: bool = f(n);
        if r {
            Some(input.make_range(from))
        } else {
            None
        }
    }
}

#[inline]
pub fn satisfy<I, F>(f: F) -> Satisfy<F, I>
where
    I: TimeTravel,
    F: FnMut(I::Item) -> bool,
{
    Satisfy::new(f)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "asd";
        let span = code.span();
        let x = satisfy(|c: Char| c == 'a');

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(0..1))
    }

    #[test]
    fn test_one() {
        let code = "a";
        let span = code.span();
        let x = satisfy(|c: Char| c == 'a');

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(0..1))
    }

    #[test]
    fn test_none() {
        let code = "b";
        let span = code.span();
        let x = satisfy(|c: Char| c == 'a');

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, None)
    }

    #[test]
    fn test_empty() {
        let code = "";
        let span = code.span();
        let x = satisfy(|c: Char| c == 'a');

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, None)
    }
}
