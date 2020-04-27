use crate::common::cell::*;
use crate::*;
use std::marker::PhantomData;

/// Fail if the subparser fail, otherwise calls f with the new Parser and parse the Parser
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AndThen<B: Parser<I>, I: TimeTravel, F> {
    base: B,
    f: ExtRefCell<F>,
    _i: PhantomData<I>,
}
impl<B: Parser<I>, I: TimeTravel, U, F> AndThen<B, I, F>
where
    U: Parser<I>,
    F: FnMut(B::Output) -> U,
{
    pub fn new(base: B, f: F) -> Self {
        Self {
            base,
            f: ExtRefCell::new(f),
            _i: PhantomData,
        }
    }
}
impl<B: Parser<I>, I: TimeTravel, U, F> Parser<I> for AndThen<B, I, F>
where
    U: Parser<I>,
    F: FnMut(B::Output) -> U,
{
    type Output = U::Output;

    fn parse(&self, mut input: I) -> Option<Self::Output> {
        let base = self.base.parse(input.ref_clone())?;
        let f = unsafe { self.f.get_mut() };
        let then: U = f(base);
        input.re_ready();
        then.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "asd123";
        let span = code.span();
        let x = substr("asd");
        let t = x.and_then(|_| substr("123"));

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(3..6))
    }
}
