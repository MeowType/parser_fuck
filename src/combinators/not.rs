use crate::*;
use std::marker::PhantomData;

/// Pass if the subparser fail
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Not<A, I = ()> {
    a: A,
    _i: PhantomData<I>,
}
impl<I: TimeTravel, A> Not<A, I>
where
    A: Parser<I>,
{
    #[inline]
    pub fn new(a: A) -> Self {
        Self { a, _i: PhantomData }
    }
}
impl<I: TimeTravel, A> Parser<I> for Not<A, I>
where
    A: Parser<I>,
{
    type Output = ();

    fn parse(&self, input: I) -> Option<Self::Output> {
        let a = self.a.parse(input.ref_clone());
        if let Some(_) = a {
            None
        } else {
            Some(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "asd";
        let span = code.span();
        let a = substr("acd");
        let x = a.not();

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(()))
    }
}
