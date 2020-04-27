use crate::common::cell::*;
use crate::*;
use std::marker::PhantomData;

/// Pass if subparser pass, otherwise calls f and parse the result Parser
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OrElse<B: Parser<I>, I: TimeTravel, F> {
    base: B,
    f: ExtRefCell<F>,
    _i: PhantomData<I>,
}
impl<B: Parser<I>, I: TimeTravel, U, F> OrElse<B, I, F>
where
    U: Parser<I, Output = B::Output>,
    F: FnMut() -> U,
{
    pub fn new(base: B, f: F) -> Self {
        Self {
            base,
            f: ExtRefCell::new(f),
            _i: PhantomData,
        }
    }
}
impl<B: Parser<I>, I: TimeTravel, U, F> Parser<I> for OrElse<B, I, F>
where
    U: Parser<I, Output = B::Output>,
    F: FnMut() -> U,
{
    type Output = B::Output;

    fn parse(&self, mut input: I) -> Option<Self::Output> {
        let from = input.save();
        let base = self.base.parse(input.ref_clone());
        if let None = base {
            let f = unsafe { self.f.get_mut() };
            let then: U = f();
            input.back(from);
            then.parse(input)
        } else {
            base
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "123qwe";
        let span = code.span();
        let x = substr("asd");
        let t = x.or_else(|| substr("123"));

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(0..3))
    }

    #[test]
    fn test_2() {
        let code = "123qwe";
        let span = code.span();
        let x = substr("123");
        let t = x.or_else(|| substr("asd"));

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(0..3));
    }

    #[test]
    fn test_none() {
        let code = "123qwe";
        let span = code.span();
        let x = substr("asd");
        let t = x.or_else(|| substr("qwe"));

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, None);
    }

    #[test]
    fn test_multi() {
        let code = "123qwe";
        let mut span = code.span();
        let x = substr("asd");
        let t = x.or_else(|| substr("123"));

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(0..3));

        span.re_ready();

        let x = substr("qwe");
        let t = x.or_else(|| substr("456"));

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(3..6));
    }
}
