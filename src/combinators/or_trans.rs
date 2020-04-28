use crate::common::cell::*;
use crate::*;
use std::marker::PhantomData;
use std::ops::Range;

/// Pass if subparser pass, otherwise calls f with error point
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OrTrans<B: Parser<I>, I: TimeTravel, F> {
    base: B,
    f: ExtRefCell<F>,
    _i: PhantomData<I>,
}
impl<B: Parser<I>, I: TimeTravel, F> OrTrans<B, I, F>
where
    F: FnMut(I, Range<usize>) -> B::Output,
{
    pub fn new(base: B, f: F) -> Self {
        Self {
            base,
            f: ExtRefCell::new(f),
            _i: PhantomData,
        }
    }
}
impl<B: Parser<I>, I: TimeTravel, F> Parser<I> for OrTrans<B, I, F>
where
    F: FnMut(I, Range<usize>) -> B::Output,
{
    type Output = B::Output;

    fn parse(&self, mut input: I) -> Option<Self::Output> {
        let from = input.save();
        let base = self.base.parse(input.ref_clone());
        if let None = base {
            let f = unsafe { self.f.get_mut() };
            let now = input.save();
            let r = if input.is_complete() && from == now {
                from - 1
            } else {
                from
            }..now;
            input.back(from);
            Some(f(input.ref_clone(), r))
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
        let x = substr("1as");
        let t = x.or_trans(|_: CharSpan, ep| ep);

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(0..2))
    }
}
