use crate::*;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Map<B: Parser<I>, I: TimeTravel, F> {
    base: B,
    f: ExtRefCell<F>,
    _i: PhantomData<I>,
}
impl<B: Parser<I>, I: TimeTravel, U, F> Map<B, I, F>
where
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
impl<B: Parser<I>, I: TimeTravel, U, F> Parser<I> for Map<B, I, F>
where
    F: FnMut(B::Output) -> U,
{
    type Output = U;

    fn parse(&self, input: I) -> Option<Self::Output> {
        let base = self.base.parse(input)?;
        let f = unsafe { self.f.get_mut() };
        Some(f(base))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "asd";
        let span = code.span();
        let x = substr("asd");
        let m = x.map(|r| span.com_string(r));

        let r = m.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(Some("asd".to_string())));
    }
}
