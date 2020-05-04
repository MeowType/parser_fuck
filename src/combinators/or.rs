use crate::*;
use std::marker::PhantomData;

/// Pass when any subparser passes
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

#[macro_export]
#[doc(hidden)]
macro_rules! _or {
    { $input:ident, $from:ident ; $a:expr, $b: expr } => {
        $a.parse($input.ref_clone()).or_else(|| {
            $input.back($from);
            $b.parse($input)
        })
    };
    { $input:ident, $from:ident ; $a:expr, $($b: expr),+ } => {
        $a.parse($input.ref_clone()).or_else(|| {
            $input.back($from);
            _or!($input, $from ; $($b),+)
        })
    }
}
/// Pass when any subparser passes
/// # Example
/// ```rust
/// # use parser_fuck::*;
/// let code = "b".span();
/// let r = or!(code ; one('a'), one('b'), one('c'));
/// assert_eq!(r, Some(0..1));
/// ```
#[macro_export(local_inner_macros)]
macro_rules! or {
    { } => {{ }};
    { $input:expr } => {{ }};
    { $input:expr ; } => {{ }};
    { $input:expr ; $a:expr $(,)? } => {{
        $a.parse($input)
    }};
    { $input:expr ; $a:expr, $($b: expr),+ $(,)? } => {{
        let mut input = $input;
        let from = input.save();
        _or!(input, from ; $a, $($b),+)
    }};
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_macro() {
        let code = "b";
        let span = code.span();

        let r = or!(span ; one('a'), one('b'), one('c'));
        println!("{:?}", r);
        assert_eq!(r, Some(0..1));
    }

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
