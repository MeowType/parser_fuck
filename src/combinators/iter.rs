use crate::*;
use std::marker::PhantomData;

/// Continuously parse into iterators
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Iter<A, I = ()> {
    a: A,
    _i: PhantomData<I>,
}
impl<I: TimeTravel, A> Iter<A, I>
where
    A: Parser<I>,
{
    #[inline]
    pub fn new(a: A) -> Self {
        Self { a, _i: PhantomData }
    }
}
impl<I: TimeTravel, A: Clone> Parser<I> for Iter<A, I>
where
    A: Parser<I>,
{
    type Output = ParserIter<A, I>;

    #[inline]
    fn parse(&self, input: I) -> Option<Self::Output> {
        Some(ParserIter {
            a: self.a.clone(),
            input,
        })
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParserIter<A, I> {
    a: A,
    input: I,
}
impl<A, I> ParserIter<A, I> {
    pub fn input(&self) -> &I {
        &self.input
    }
}
impl<A, I: TimeTravel> Iterator for ParserIter<A, I>
where
    A: Parser<I>,
{
    type Item = A::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let from = self.input.save();
        let a = self.a.parse(self.input.ref_clone());
        if let None = a {
            self.input.back(from);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::ops::Range;

    #[test]
    fn test() {
        let code = "aaa";
        let span = code.span();
        let a = one('a');
        let x = a.iter();

        let r = x.parse(span).unwrap();
        let v: Vec<Range<usize>> = r.collect();
        println!("{:?}", v);
        assert_eq!(v, vec![0..1, 1..2, 2..3]);
    }

    #[test]
    fn test_partial() {
        let code = "aab";
        let span = code.span();
        let a = one('a');
        let x = a.iter();

        let r = x.parse(span).unwrap();
        let v: Vec<Range<usize>> = r.collect();
        println!("{:?}", v);
        assert_eq!(v, vec![0..1, 1..2]);
    }

    #[test]
    fn test_none() {
        let code = "bbb";
        let span = code.span();
        let a = one('a');
        let x = a.iter();

        let r = x.parse(span).unwrap();
        let v: Vec<Range<usize>> = r.collect();
        println!("{:?}", v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_multi() {
        let code = "aab";
        let span = code.span();
        let a = one('a');
        let b = one('b');
        let x = a.iter();

        let r = x.parse(span.ref_clone()).unwrap();
        let v: Vec<Range<usize>> = r.collect();
        println!("{:?}", v);
        assert_eq!(v, vec![0..1, 1..2]);

        let y = b.iter();

        let r = y.parse(span).unwrap();
        let v: Vec<Range<usize>> = r.collect();
        println!("{:?}", v);
        assert_eq!(v, vec![2..3]);
    }
}
