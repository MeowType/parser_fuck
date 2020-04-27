use crate::*;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct May<A, I = ()> {
    a: A,
    _i: PhantomData<I>,
}
impl<I: TimeTravel, A> May<A, I>
where
    A: Parser<I>,
{
    #[inline]
    pub fn new(a: A) -> Self {
        Self { a, _i: PhantomData }
    }
}
impl<I: TimeTravel, A> Parser<I> for May<A, I>
where
    A: Parser<I>,
{
    type Output = Option<A::Output>;

    fn parse(&self, mut input: I) -> Option<Self::Output> {
        let from = input.save();
        let a = self.a.parse(input.ref_clone());
        if a.is_none() {
            input.back(from);
        } else {
            input.re_ready();
        }
        Some(a)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "asd123";
        let span = code.span();
        let a = substr("asd");
        let x = a.may();

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(Some(0..3)));
    }

    #[test]
    fn test_none() {
        let code = "123123";
        let span = code.span();
        let a = substr("asd");
        let x = a.may();

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(None));
    }

    #[test]
    fn test_2() {
        let code = "asdasd";
        let span = code.span();
        let a = substr("asd");
        let x = a.may();

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(Some(0..3)));
    }

    #[test]
    fn test_multi() {
        let code = "asd123";
        let span = code.span();
        let a = substr("asd");
        let b = substr("123");
        let x = a.may();
        let y = b.may();

        let r = x.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(Some(0..3)));
        let r = y.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(Some(3..6)));
    }
}
