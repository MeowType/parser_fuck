use crate::common::cell::*;
use crate::*;
use std::marker::PhantomData;
use std::ops::Range;

/// Pass if subparser pass, otherwise calls f with error point
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OrTrans<B: Parser<I>, I: TimeTravel, F> {
    base: B,
    no_eof: bool,
    no_retry: bool,
    f: ExtRefCell<F>,
    _i: PhantomData<I>,
}
impl<B: Parser<I>, I: TimeTravel, F> OrTrans<B, I, F>
where
    F: FnMut(I, Range<usize>) -> B::Output,
{
    pub fn new(base: B, no_eof: bool, no_retry: bool, f: F) -> Self {
        Self {
            base,
            no_eof,
            no_retry,
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
            if self.no_retry {
                if input.is_complete() && self.no_eof {
                    return None;
                }
                let f = unsafe { self.f.get_mut() };
                let now = input.save();
                let r = if input.is_complete() && from == now && now != 0 {
                    from - 1
                } else {
                    from
                }..now;
                return Some(f(input.ref_clone(), r));
            }
            loop {
                if input.is_complete() {
                    if self.no_eof {
                        return None;
                    }
                    let now = input.save();
                    let f = unsafe { self.f.get_mut() };

                    let r = if input.is_complete() && from == now && now != 0 {
                        from - 1
                    } else {
                        from
                    }..now;

                    let r = Some(f(input.clone(), r));
                    return r;
                }
                let save = input.save();
                let base = self.base.parse(input.ref_clone());
                if base.is_some() {
                    let f = unsafe { self.f.get_mut() };

                    let r = from..save;

                    let r = Some(f(input.ref_clone(), r));
                    input.back(save);
                    return r;
                }
            }
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
        let code = "asd123";
        let span = code.span();
        let x = substr("123");
        let t = x.or_trans(false, |_: CharSpan, ep| ep);

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(0..3))
    }

    #[test]
    fn test_no_retry() {
        let code = "asd123";
        let span = code.span();
        let x = substr("123");
        let t = x.or_trans(true, |_: CharSpan, ep| ep);

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(0..1))
    }

    #[test]
    fn test_empty() {
        let code = "";
        let span = code.span();
        let x = substr("asd");
        let t = x.or_trans(false, |_: CharSpan, ep| ep);

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(0..0))
    }

    #[test]
    fn test_no_match() {
        let code = "as";
        let span = code.span();
        let x = substr("asd");
        let t = x.or_trans(false, |_: CharSpan, ep| ep);

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(0..2));
        let l = span.loc_range(r.unwrap());
        println!("{:?}", l);
        assert_eq!(l, Some((0, 0, 0, 1, 0, 1).into()));
    }

    #[test]
    fn test_noend() {
        let code = "as";
        let span = code.span();
        let x = substr("asd");
        let t = x.or_trans_noend(false, |_: CharSpan, ep| ep);

        let r = t.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, None)
    }
}
