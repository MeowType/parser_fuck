use crate::*;
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct Dyn<I: TimeTravel, O = ()> {
    base: Arc<dyn Parser<I, Output = O>>,
}
impl<I: TimeTravel, O> Dyn<I, O> {
    #[inline]
    pub fn new<T: Parser<I, Output = O> + 'static>(base: T) -> Self {
        Self {
            base: Arc::new(base),
        }
    }
}
impl<I: TimeTravel, O> From<Arc<dyn Parser<I, Output = O>>> for Dyn<I, O> {
    fn from(base: Arc<dyn Parser<I, Output = O>>) -> Self {
        Self { base }
    }
}
impl<I: TimeTravel, O> Parser<I> for Dyn<I, O> {
    type Output = O;

    #[inline]
    fn parse(&self, input: I) -> Option<Self::Output> {
        self.base.parse(input)
    }
}
impl<I: TimeTravel, O> Debug for Dyn<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dyn").field("base", &"...").finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let code = "asd";
        let span = code.span();

        let a = substr("asd");
        let d = a.dyns();

        let r = d.parse(span);
        println!("{:?}", r);
        assert_eq!(r, Some(0..3));
    }
}
