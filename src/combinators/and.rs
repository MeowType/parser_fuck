use crate::*;
use std::marker::PhantomData;

/// Only pass if both subparsers pass
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

#[macro_export]
#[doc(hidden)]
macro_rules! _and {
    { $(,)? $($r:ident),* } => { ($($r),*) };
    { $input:ident ; $a: expr, $b: expr ; $(,)? $($r:ident),* } => {
        $a.parse($input.ref_clone()).and_then(|a| {
            $input.re_ready();
            $b.parse($input).map(|b| _and!($($r),*, a, b))
        })
    };
    { $input:ident ; $a: expr, $($b: expr),+ ; $(,)? $($r:ident),* } => {
        $a.parse($input.ref_clone()).and_then(|a| {
            $input.re_ready();
            _and!($input ; $($b),+ ; $($r),* , a )
        })
    };
}
/// Only pass if all subparsers pass
/// # Example
/// ```rust
/// # use parser_fuck::*;
/// let code = "asd".span();
/// let r = and!(code; one('a'), one('s'), one('d'));
/// assert_eq!(r, Some((0..1, 1..2, 2..3)));
/// ```
#[macro_export(local_inner_macros)]
macro_rules! and {
    { } => {{ }};
    { $input:expr } => {{ }};
    { $input:expr ; } => {{ }};
    { $input:expr ; $a:expr $(,)? } => {{
        $a.parse($input)
    }};
    { $input:expr ; $a: expr, $($b: expr),+ $(,)? } => {{
        let mut input = $input;
        _and!(input ; $a, $($b),+ ;)
    }};
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_macro() {
        let code = "asd";
        let span = code.span();

        let r = and!(span; one('a'), one('s'), one('d'));
        println!("{:?}", r);
        assert_eq!(r, Some((0..1, 1..2, 2..3)))
    }

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
