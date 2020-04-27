use crate::*;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct And<A, B, I = ()> {
    a: A,
    b: B,
    _i: PhantomData<I>,
}
impl<I: TimeTravel, A, B> And<A, B, I>
where
    A: Parser<I>,
    B: Parser<I>,
{
    #[inline]
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            _i: PhantomData,
        }
    }
}
impl<I: TimeTravel, A, B> Parser<I> for And<A, B, I>
where
    A: Parser<I>,
    B: Parser<I>,
{
    type Output = (A::Output, B::Output);

    fn parse(&self, mut input: I) -> Option<Self::Output> {
        let a = self.a.parse(input.ref_clone())?;
        input.re_ready();
        let b = self.b.parse(input)?;
        Some((a, b))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "asd";
        let span = code.span();
        let a = substr("a");
        let b = substr("s");
        let x = a.and(b);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some((0..1, 1..2)));
    }
}
