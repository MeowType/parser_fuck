use crate::*;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Many<A, I = ()> {
    a: A,
    min: usize,
    max: Option<usize>,
    _i: PhantomData<I>,
}
impl<I: TimeTravel, A> Many<A, I>
where
    A: Parser<I>,
{
    #[inline]
    pub fn new(a: A, min: usize, max: Option<usize>) -> Self {
        if let Some(max) = max {
            if max < min {
                panic!("max must be >= min")
            }
        }
        Self {
            a,
            min,
            max,
            _i: PhantomData,
        }
    }
}
impl<I: TimeTravel, A> Parser<I> for Many<A, I>
where
    A: Parser<I>,
{
    type Output = Vec<A::Output>;

    fn parse(&self, mut input: I) -> Option<Self::Output> {
        let mut vec: Vec<A::Output> = vec![];
        if let Some(max) = self.max {
            loop {
                let from = input.save();
                let a = self.a.parse(input.ref_clone());
                if let Some(a) = a {
                    vec.push(a);
                    if vec.len() > max {
                        return None;
                    }
                } else {
                    input.back(from);
                    if vec.len() >= self.min {
                        return Some(vec);
                    } else {
                        return None;
                    }
                }
                input.re_ready()
            }
        } else {
            loop {
                let from = input.save();
                let a = self.a.parse(input.ref_clone());
                if let Some(a) = a {
                    vec.push(a);
                } else {
                    input.back(from);
                    if vec.len() >= self.min {
                        return Some(vec);
                    } else {
                        return None;
                    }
                }
                input.re_ready()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "asdasdasd";
        let span = code.span();
        let a = substr("asd");
        let x = a.many();

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![0..3, 3..6, 6..9]));
    }

    #[test]
    fn test_multi1() {
        let code = "asd123";
        let span = code.span();
        let a = substr("asd");
        let b = substr("123");
        let x = a.many();
        let y = b.many();

        let r = x.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(vec![0..3]));
        let r = y.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![3..6]));
    }

    #[test]
    fn test_multi() {
        let code = "asdasd123123";
        let span = code.span();
        let a = substr("asd");
        let b = substr("123");
        let x = a.many();
        let y = b.many();

        let r = x.parse(span.ref_clone());
        println!("{:?}", r);
        assert_eq!(r, Some(vec![0..3, 3..6]));
        let r = y.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![6..9, 9..12]));
    }

    #[test]
    fn test_partial() {
        let code = "asdasdqwe";
        let span = code.span();
        let a = substr("asd");
        let x = a.many();

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![0..3, 3..6]));
    }

    #[test]
    fn test_0() {
        let code = "qwe123";
        let span = code.span();
        let a = substr("asd");
        let x = a.many();

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![]));
    }

    #[test]
    fn test_1() {
        let code = "qwe123";
        let span = code.span();
        let a = substr("asd");
        let x = a.many1();

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, None);
    }
    #[test]
    fn test_1_has() {
        let code = "asdqwe123";
        let span = code.span();
        let a = substr("asd");
        let x = a.many1();

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![0..3]));
    }

    #[test]
    fn test_1_more() {
        let code = "asdasd123";
        let span = code.span();
        let a = substr("asd");
        let x = a.many1();

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![0..3, 3..6]));
    }

    #[test]
    fn test_1_all() {
        let code = "asdasdasd";
        let span = code.span();
        let a = substr("asd");
        let x = a.many1();

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![0..3, 3..6, 6..9]));
    }

    #[test]
    fn test_max() {
        let code = "asdasdasd";
        let span = code.span();
        let a = substr("asd");
        let x = a.many_max(1);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, None);
    }

    #[test]
    fn test_max_some() {
        let code = "asd123asd";
        let span = code.span();
        let a = substr("asd");
        let x = a.many_max(1);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![0..3]));
    }

    #[test]
    fn test_max_empty() {
        let code = "qwe123asd";
        let span = code.span();
        let a = substr("asd");
        let x = a.many_max(1);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![]));
    }

    #[test]
    fn test_range_1() {
        let code = "asd123asd";
        let span = code.span();
        let a = substr("asd");
        let x = a.many_min_max(1, 1);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![0..3]));
    }

    #[test]
    fn test_range_2() {
        let code = "asdasdasd";
        let span = code.span();
        let a = substr("asd");
        let x = a.many_min_max(1, 1);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, None);
    }

    #[test]
    fn test_range_3() {
        let code = "123qwe";
        let span = code.span();
        let a = substr("asd");
        let x = a.many_min_max(1, 1);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, None);
    }

    #[test]
    fn test_range_4() {
        let code = "asdasd123";
        let span = code.span();
        let a = substr("asd");
        let x = a.many_min_max(1, 2);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![0..3, 3..6]));
    }

    #[test]
    fn test_range_5() {
        let code = "asdasdasd123";
        let span = code.span();
        let a = substr("asd");
        let x = a.some(3);

        let r = x.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(vec![0..3, 3..6, 6..9]));
    }

    #[test]
    #[should_panic]
    fn test_max_lt_min() {
        let code = "asdasdasd123";
        let span = code.span();
        let a = substr("asd");
        let x = a.many_min_max(3, 1);
        let r = x.parse(span);
        println!("{:?}", r);
    }
}
