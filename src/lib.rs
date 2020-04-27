pub mod combinators;
pub mod common;

pub use combinators::*;
pub use common::*;

/// Abstract Parser with chain call
pub trait Parser<I: TimeTravel> {
    type Output;

    /// do parse
    fn parse(&self, input: I) -> Option<Self::Output>;

    /// Map a `Parser<Output = T>` to `Parser<Output = U>` by applying a function to a contained value
    #[inline]
    fn map<U, F>(self, f: F) -> Map<Self, I, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> U,
    {
        Map::new(self, f)
    }

    /// Only pass if both subparsers pass
    #[inline]
    fn and<B>(self, b: B) -> And<Self, B, I>
    where
        Self: Sized,
        B: Parser<I>,
    {
        And::new(self, b)
    }

    /// Pass when any subparser passes
    #[inline]
    fn or<B>(self, b: B) -> Or<Self, B, I>
    where
        Self: Sized,
        B: Parser<I, Output = Self::Output>,
    {
        Or::new(self, b)
    }

    /// Pass if the subparser fail
    #[inline]
    fn not(self) -> Not<Self, I>
    where
        Self: Sized,
    {
        Not::new(self)
    }

    /// `*, >= 0`
    #[inline]
    fn many(self) -> Many<Self, I>
    where
        Self: Sized,
    {
        Many::new(self, 0, None)
    }

    /// `+, >= 1`
    #[inline]
    fn many1(self) -> Many<Self, I>
    where
        Self: Sized,
    {
        Many::new(self, 1, None)
    }

    /// `{n,}, >= n`
    #[inline]
    fn many_min(self, min: usize) -> Many<Self, I>
    where
        Self: Sized,
    {
        Many::new(self, min, None)
    }

    /// `{,m}, <= m`
    #[inline]
    fn many_max(self, max: usize) -> Many<Self, I>
    where
        Self: Sized,
    {
        Many::new(self, 0, Some(max))
    }

    /// `{1,m}, >= 1 && <= m`
    #[inline]
    fn many1_max(self, max: usize) -> Many<Self, I>
    where
        Self: Sized,
    {
        Many::new(self, 1, Some(max))
    }

    /// `{n,m}, >= n && <= m`
    #[inline]
    fn many_min_max(self, min: usize, max: usize) -> Many<Self, I>
    where
        Self: Sized,
    {
        Many::new(self, min, Some(max))
    }

    /// `{n}, == n`
    #[inline]
    fn some(self, count: usize) -> Many<Self, I>
    where
        Self: Sized,
    {
        Many::new(self, count, Some(count))
    }

    /// `?, 0 or 1`
    #[inline]
    fn may(self) -> May<Self, I>
    where
        Self: Sized,
    {
        May::new(self)
    }

    /// Continuously parse into iterators
    #[inline]
    fn iter(self) -> Iter<Self, I>
    where
        Self: Sized,
    {
        Iter::new(self)
    }

    /// Fail if the subparser fail, otherwise calls f with the new Parser and parse the Parser
    #[inline]
    fn and_then<U, F>(self, f: F) -> AndThen<Self, I, F>
    where
        Self: Sized,
        U: Parser<I>,
        F: FnMut(Self::Output) -> U,
    {
        AndThen::new(self, f)
    }

    /// Pass if subparser pass, otherwise calls f and parse the result Parser
    #[inline]
    fn or_else<U, F>(self, f: F) -> OrElse<Self, I, F>
    where
        Self: Sized,
        U: Parser<I, Output = Self::Output>,
        F: FnMut() -> U,
    {
        OrElse::new(self, f)
    }

    /// Wrap to dynamic
    #[inline]
    fn dyns(self) -> Dyn<I, Self::Output>
    where
        Self: Sized + 'static,
    {
        Dyn::new(self)
    }
}

impl<I, U, F> Parser<I> for F
where
    I: TimeTravel,
    F: Fn(I) -> Option<U>,
{
    type Output = U;

    #[inline]
    fn parse(&self, input: I) -> Option<Self::Output> {
        self(input)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! _def_parser_I_select {
 { } => { I };
 { $id:ident } => { $id };
}

/// ***Unfinished***  
/// Define a Parser with data like a function  
#[macro_export(local_inner_macros)]
macro_rules! def_parser {
    { } => { };
    { $lt:lifetime } => { std::marker::PhantomData };
    { $vis:vis $name:ident [$I:path] ($input:ident) -> $output:ty $(where $($tn:path: $wt:path),+)? $b:block } => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, PartialEq, Eq, Clone)]
        $vis struct $name;
        impl<$I: $crate::parser_fuck::TimeTravel> Parser<I> for $name
        $(where $($tn : $wt),+)?
        {
            type Output = $output;

            #[allow(unused_mut)]
            fn parse(&self, mut $input: I) -> Option<Self::Output> $b
        }
    };
    { $vis:vis $name:ident $(<$($lt:lifetime),*>)? ($input:ident: $it:ty ) -> $output:ty $b:block } => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, PartialEq, Eq, Clone)]
        $vis struct $name$(<$($lt),+>)? ( $($(std::marker::PhantomData<&$lt ()>),*)? );
        impl$(<$($lt),*>)? $name $(<$($lt),*>)? {
            #[inline]
            pub fn new() -> Self {
                Self ( $($(def_parser!($lt)),*)? )
            }
        }
        impl$(<$($lt),*>)? Parser<$it> for $name $(<$($lt),*>)?
        {
            type Output = $output;

            #[allow(unused_mut)]
            fn parse(&self, mut $input: $it) -> Option<Self::Output> $b
        }
    };
}