use crate::*;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Or<A, B, I = ()> {
    a: A,
    b: B,
    _i: PhantomData<I>,
}
impl<I: TimeTravel, A, B> Or<A, B, I>
where
    A: Parser<I>,
    B: Parser<I, Output = A::Output>,
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
impl<I: TimeTravel, A, B> Parser<I> for Or<A, B, I>
where
    A: Parser<I>,
    B: Parser<I, Output = A::Output>,
{
    type Output = A::Output;

    fn parse(&self, mut input: I) -> Option<Self::Output> {
        let from = input.save();
        let a = self.a.parse(input.ref_clone());
        a.or_else(|| {
            input.back(from);
            let b = self.b.parse(input);
            b
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "asd";
        let span = code.span();
        let a = substr("b");
        let b = substr("a");
        let x = a.or(b);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(0..1));
    }

    #[test]
    fn test_a() {
        let code = "asd";
        let span = code.span();
        let a = substr("a");
        let b = substr("b");
        let x = a.or(b);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(0..1));
    }

    #[test]
    fn test_none() {
        let code = "asd";
        let span = code.span();
        let a = substr("b");
        let b = substr("c");
        let x = a.or(b);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, None);
    }
}
