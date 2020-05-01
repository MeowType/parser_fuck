//! A simple parser combinator library
//! 
//! ## example
//! ```rust
//! use parser_fuck::*;
//! use std::collections::HashMap;
//! use std::f64;
//! use std::ops::Range;
//! 
//! static CODE: &'static str =
//!     "{ \"a\": 1, \"b\": true, \"c\": [null, 1.5, false], \"d\": { \"v\": \"asd\" } }";
//! 
//! fn main() {
//!     let r = json(CODE);
//!     println!("{:?}", r);
//!     assert_eq!(
//!         r,
//!         Ok(JsonVal::Object({
//!             let mut map = HashMap::new();
//!             map.insert("a".to_string(), JsonVal::Number(1.0));
//!             map.insert("b".to_string(), JsonVal::Bool(true));
//!             map.insert(
//!                 "c".to_string(),
//!                 JsonVal::Array(vec![
//!                     JsonVal::Null,
//!                     JsonVal::Number(1.5),
//!                     JsonVal::Bool(false),
//!                 ]),
//!             );
//!             map.insert(
//!                 "d".to_string(),
//!                 JsonVal::Object({
//!                     let mut map = HashMap::new();
//!                     map.insert("v".to_string(), JsonVal::String("asd".to_string()));
//!                     map
//!                 }),
//!             );
//!             map
//!         }))
//!     )
//! }
//! 
//! pub fn json(code: &str) -> JsonResult {
//!     let code = code.span();
//!     value.parse(code).unwrap()
//! }
//! 
//! fn boolval(input: CharSpan) -> Option<JsonVal> {
//!     substr("true")
//!         .map(|_| JsonVal::Bool(true))
//!         .or(substr("false").map(|_| JsonVal::Bool(false)))
//!         .parse(input)
//! }
//! 
//! fn nullval(input: CharSpan) -> Option<JsonVal> {
//!     substr("null").map(|_| JsonVal::Null).parse(input)
//! }
//! 
//! fn numberval(input: CharSpan) -> Option<JsonVal> {
//!     fn num_start(input: CharSpan) -> Option<Range<usize>> {
//!         satisfy(|c: Char| {
//!             c.char(|c: char, _| c != '0' && c.is_ascii_digit())
//!                 .unwrap_or(false)
//!         })
//!         .parse(input)
//!     }
//!     fn num_body(input: CharSpan) -> Option<Range<usize>> {
//!         satisfy(|c: Char| c.char(|c: char, _| c.is_ascii_digit()).unwrap_or(false)).parse(input)
//!     }
//!     one('-')
//!         .may()
//!         .and(
//!             one('0').or(num_start
//!                 .many1()
//!                 .and(num_body.many())
//!                 .map(range_of_many1_many)),
//!         )
//!         .map(range_of_optrange_range)
//!         .and(
//!             one('.')
//!                 .and(num_body.many1())
//!                 .map(range_of_range_many1)
//!                 .may(),
//!         )
//!         .map(range_of_range_optrange)
//!         .and(
//!             one('e')
//!                 .or(one('E'))
//!                 .and(one('-').or(one('+')).may())
//!                 .map(range_of_range_optrange)
//!                 .and(num_body.many1())
//!                 .map(range_of_range_many1)
//!                 .may(),
//!         )
//!         .map(range_of_range_optrange)
//!         .map(|v: Range<usize>| {
//!             let s: String = input.com_string(v).unwrap();
//!             let v: f64 = s.parse::<f64>().unwrap();
//!             JsonVal::Number(v)
//!         })
//!         .parse(input.ref_clone())
//! }
//! 
//! fn stringval(input: CharSpan) -> Option<JsonResult> {
//!     fn str_esc(input: CharSpan) -> Option<Result<char, JsonParserError>> {
//!         one('\\')
//!             .and({
//!                 one('"')
//!                     .map(|_| '"')
//!                     .or(one('\\').map(|_| '\\'))
//!                     .or(one('/').map(|_| '/'))
//!                     .or(one('b').map(|_| ''))
//!                     .or(one('f').map(|_| ''))
//!                     .or(one('n').map(|_| '\n'))
//!                     .or(one('r').map(|_| '\r'))
//!                     .or(one('t').map(|_| '\t'))
//!                     .or(one('u')
//!                         .and(
//!                             satisfy(|c: Char| c.char(|c: char, _| c.is_digit(16)).unwrap_or(false))
//!                                 .some(4),
//!                         )
//!                         .map(|(_, u)| {
//!                             let s = input.com_string(range_of_many1(u)).unwrap();
//!                             let hex: u32 = u32::from_str_radix(&*s, 16).unwrap();
//!                             let c = std::char::from_u32(hex).unwrap();
//!                             c
//!                         }))
//!                     .map(|v| Result::<char, JsonParserError>::Ok(v))
//!                     .or_trans(|i: CharSpan, ep| {
//!                         let loc = i.loc_range(ep).unwrap();
//!                         Err(JsonParserError {
//!                             loc,
//!                             msg: "Illegal escape character".to_string(),
//!                         })
//!                     })
//!             })
//!             .map(|(_, v)| v)
//!             .parse(input.ref_clone())
//!     }
//!     fn str_body(input: CharSpan) -> Option<Result<char, JsonParserError>> {
//!         satisfy(|c: Char| c == '' || c == '' || c == '\n' || c == '\r' || c == '\t')
//!             .map(|i| {
//!                 let loc = input.loc_range(i).unwrap();
//!                 Err(JsonParserError {
//!                     loc,
//!                     msg: "Control characters are not allowed in the string".to_string(),
//!                 })
//!             })
//!             .or(str_esc)
//!             .or(satisfy(|c: Char| c != '"').map(|i| {
//!                 let s = input.com_string(i).unwrap();
//!                 let c = s.chars().next().unwrap();
//!                 Ok(c)
//!             }))
//!             .parse(input.ref_clone())
//!     }
//!     one('"')
//!         .and(str_body.many())
//!         .and(one('"'))
//!         .map(
//!             |((_, v), _): (
//!                 (Range<usize>, Vec<Result<char, JsonParserError>>),
//!                 Range<usize>,
//!             )| {
//!                 let mut val = String::new();
//!                 for c in v {
//!                     let c = c?;
//!                     val.push(c);
//!                 }
//!                 let jv = JsonVal::String(val);
//!                 Ok(jv)
//!             },
//!         )
//!         .parse(input.ref_clone())
//! }
//! 
//! fn whitespace(input: CharSpan) -> Option<()> {
//!     satisfy(|c: Char| c.is_wrap() || c == ' ' || c == '\t')
//!         .many()
//!         .map(|_| {})
//!         .parse(input)
//! }
//! 
//! fn value(input: CharSpan) -> Option<JsonResult> {
//!     whitespace
//!         .and(
//!             stringval
//!                 .or(object)
//!                 .or(array)
//!                 .or(numberval.or(boolval).or(nullval).map(|v| Ok(v))),
//!         )
//!         .and(whitespace)
//!         .map(|((_, v), _)| v)
//!         .or_trans(|i: CharSpan, ep| {
//!             let loc = i.loc_range(ep).unwrap();
//!             Err(JsonParserError {
//!                 loc,
//!                 msg: "Invaild character".to_string(),
//!             })
//!         })
//!         .parse(input)
//! }
//! 
//! fn array(input: CharSpan) -> Option<JsonResult> {
//!     one('[')
//!         .and({
//!             value
//!                 .and(one(',').and(value).many())
//!                 .map(|(f, v)| {
//!                     let mut vals: Vec<JsonVal> = vec![f?];
//!                     for vv in v {
//!                         let (_, val) = vv;
//!                         vals.push(val?);
//!                     }
//!                     Ok(vals)
//!                 })
//!                 .or(whitespace.map(|_| Ok(vec![])))
//!         })
//!         .and(one(']').map(|_| Ok(())).or_trans(|i: CharSpan, ep| {
//!             let loc = i.loc_range(ep);
//!             let loc = loc.unwrap();
//!             Err(JsonParserError {
//!                 loc,
//!                 msg: "Need \"]\" but not found it".to_string(),
//!             })
//!         }))
//!         .map(
//!             |((_, v), e): ((_, JsonResults<Vec<JsonVal>>), JsonResults<()>)| {
//!                 e?;
//!                 let v = v?;
//!                 Ok(JsonVal::Array(v))
//!             },
//!         )
//!         .parse(input)
//! }
//! 
//! fn object(input: CharSpan) -> Option<JsonResult> {
//!     fn kv(input: CharSpan) -> Option<JsonResults<(String, JsonVal)>> {
//!         whitespace
//!             .and(stringval)
//!             .and(whitespace)
//!             .and(one(':').map(|v| Ok(v)).or_trans(|i: CharSpan, ep| {
//!                 let loc = i.loc_range(ep).unwrap();
//!                 Err(JsonParserError {
//!                     loc,
//!                     msg: "Need \":\" but not found it".to_string(),
//!                 })
//!             }))
//!             .and(value)
//!             .map(|((((_, k), _), col), v)| {
//!                 let k = k?;
//!                 let _ = col?;
//!                 let v = v?;
//!                 let key = if let JsonVal::String(key) = k {
//!                     key
//!                 } else {
//!                     panic!("never")
//!                 };
//!                 Ok((key, v))
//!             })
//!             .parse(input)
//!     }
//!     one('{')
//!         .and({
//!             kv.and({ one(',').and(kv).many() })
//!                 .map(|(f, vs)| {
//!                     let mut vals: HashMap<String, JsonVal> = HashMap::new();
//!                     let (k, v) = f?;
//!                     vals.insert(k, v);
//!                     for vv in vs {
//!                         let (_, val) = vv;
//!                         let (k, v) = val?;
//!                         vals.insert(k, v);
//!                     }
//!                     Ok(vals)
//!                 })
//!                 .or(whitespace.map(|_| Ok(HashMap::new())))
//!         })
//!         .and(one('}').map(|_| Ok(())).or_trans(|i: CharSpan, ep| {
//!             let loc = i.loc_range(ep).unwrap();
//!             Err(JsonParserError {
//!                 loc,
//!                 msg: "Need \"}\" but not found it".to_string(),
//!             })
//!         }))
//!         .map(
//!             |((_, v), e): ((_, JsonResults<HashMap<String, JsonVal>>), JsonResults<()>)| {
//!                 e?;
//!                 let v = v?;
//!                 Ok(JsonVal::Object(v))
//!             },
//!         )
//!         .parse(input)
//! }
//! 
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! pub struct JsonParserError {
//!     pub loc: LocRange,
//!     pub msg: String,
//! }
//! pub type JsonResults<T> = Result<T, JsonParserError>;
//! pub type JsonResult = Result<JsonVal, JsonParserError>;
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! pub enum JsonVal {
//!     String(String),
//!     Number(f64),
//!     Object(HashMap<String, JsonVal>),
//!     Array(Vec<JsonVal>),
//!     Bool(bool),
//!     Null,
//! }
//! 
//! ```

pub mod combinators;
pub mod common;
pub mod utils;

pub use combinators::*;
pub use common::*;
pub use utils::*;

use std::ops::Range;

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

    /// Pass if subparser pass, otherwise calls f with error point
    #[inline]
    fn or_trans<F>(self, f: F) -> OrTrans<Self, I, F>
    where
        Self: Sized,
        F: FnMut(I, Range<usize>) -> Self::Output,
    {
        OrTrans::new(self, false, f)
    }
    #[inline]
    fn or_trans_noend<F>(self, f: F) -> OrTrans<Self, I, F>
    where
        Self: Sized,
        F: FnMut(I, Range<usize>) -> Self::Output,
    {
        OrTrans::new(self, true, f)
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

#[cfg(test)]
mod test_json {
    use crate::*;
    use std::collections::HashMap;
    use std::f64;
    use std::ops::Range;

    static CODE: &'static str =
        "{ \"a\": 1, \"b\": true, \"c\": [null, 1.5, false], \"d\": { \"v\": \"asd\" } }";

    #[test]
    fn test_json() {
        let r = json(CODE);
        println!("{:?}", r);
        assert_eq!(
            r,
            Ok(JsonVal::Object({
                let mut map = HashMap::new();
                map.insert("a".to_string(), JsonVal::Number(1.0));
                map.insert("b".to_string(), JsonVal::Bool(true));
                map.insert(
                    "c".to_string(),
                    JsonVal::Array(vec![
                        JsonVal::Null,
                        JsonVal::Number(1.5),
                        JsonVal::Bool(false),
                    ]),
                );
                map.insert(
                    "d".to_string(),
                    JsonVal::Object({
                        let mut map = HashMap::new();
                        map.insert("v".to_string(), JsonVal::String("asd".to_string()));
                        map
                    }),
                );
                map
            }))
        )
    }

    pub fn json(code: &str) -> JsonResult {
        let code = code.span();
        value.parse(code).unwrap()
    }

    fn boolval(input: CharSpan) -> Option<JsonVal> {
        substr("true")
            .map(|_| JsonVal::Bool(true))
            .or(substr("false").map(|_| JsonVal::Bool(false)))
            .parse(input)
    }

    fn nullval(input: CharSpan) -> Option<JsonVal> {
        substr("null").map(|_| JsonVal::Null).parse(input)
    }

    fn numberval(input: CharSpan) -> Option<JsonVal> {
        fn num_start(input: CharSpan) -> Option<Range<usize>> {
            satisfy(|c: Char| {
                c.char(|c: char, _| c != '0' && c.is_ascii_digit())
                    .unwrap_or(false)
            })
            .parse(input)
        }
        fn num_body(input: CharSpan) -> Option<Range<usize>> {
            satisfy(|c: Char| c.char(|c: char, _| c.is_ascii_digit()).unwrap_or(false)).parse(input)
        }
        one('-')
            .may()
            .and(
                one('0').or(num_start
                    .many1()
                    .and(num_body.many())
                    .map(range_of_many1_many)),
            )
            .map(range_of_optrange_range)
            .and(
                one('.')
                    .and(num_body.many1())
                    .map(range_of_range_many1)
                    .may(),
            )
            .map(range_of_range_optrange)
            .and(
                one('e')
                    .or(one('E'))
                    .and(one('-').or(one('+')).may())
                    .map(range_of_range_optrange)
                    .and(num_body.many1())
                    .map(range_of_range_many1)
                    .may(),
            )
            .map(range_of_range_optrange)
            .map(|v: Range<usize>| {
                let s: String = input.com_string(v).unwrap();
                let v: f64 = s.parse::<f64>().unwrap();
                JsonVal::Number(v)
            })
            .parse(input.ref_clone())
    }

    fn stringval(input: CharSpan) -> Option<JsonResult> {
        fn str_esc(input: CharSpan) -> Option<Result<char, JsonParserError>> {
            one('\\')
                .and({
                    one('"')
                        .map(|_| '"')
                        .or(one('\\').map(|_| '\\'))
                        .or(one('/').map(|_| '/'))
                        .or(one('b').map(|_| ''))
                        .or(one('f').map(|_| ''))
                        .or(one('n').map(|_| '\n'))
                        .or(one('r').map(|_| '\r'))
                        .or(one('t').map(|_| '\t'))
                        .or(one('u')
                            .and(
                                satisfy(|c: Char| {
                                    c.char(|c: char, _| c.is_digit(16)).unwrap_or(false)
                                })
                                .some(4),
                            )
                            .map(|(_, u)| {
                                let s = input.com_string(range_of_many1(u)).unwrap();
                                let hex: u32 = u32::from_str_radix(&*s, 16).unwrap();
                                let c = std::char::from_u32(hex).unwrap();
                                c
                            }))
                        .map(|v| Result::<char, JsonParserError>::Ok(v))
                        .or_trans(|i: CharSpan, ep| {
                            let loc = i.loc_range(ep).unwrap();
                            Err(JsonParserError {
                                loc,
                                msg: "Illegal escape character".to_string(),
                            })
                        })
                })
                .map(|(_, v)| v)
                .parse(input.ref_clone())
        }
        fn str_body(input: CharSpan) -> Option<Result<char, JsonParserError>> {
            satisfy(|c: Char| c == '' || c == '' || c == '\n' || c == '\r' || c == '\t')
                .map(|i| {
                    let loc = input.loc_range(i).unwrap();
                    Err(JsonParserError {
                        loc,
                        msg: "Control characters are not allowed in the string".to_string(),
                    })
                })
                .or(str_esc)
                .or(satisfy(|c: Char| c != '"').map(|i| {
                    let s = input.com_string(i).unwrap();
                    let c = s.chars().next().unwrap();
                    Ok(c)
                }))
                .parse(input.ref_clone())
        }
        one('"')
            .and(str_body.many())
            .and(one('"'))
            .map(
                |((_, v), _): (
                    (Range<usize>, Vec<Result<char, JsonParserError>>),
                    Range<usize>,
                )| {
                    let mut val = String::new();
                    for c in v {
                        let c = c?;
                        val.push(c);
                    }
                    let jv = JsonVal::String(val);
                    Ok(jv)
                },
            )
            .parse(input.ref_clone())
    }

    fn whitespace(input: CharSpan) -> Option<()> {
        satisfy(|c: Char| c.is_wrap() || c == ' ' || c == '\t')
            .many()
            .map(|_| {})
            .parse(input)
    }

    fn value(input: CharSpan) -> Option<JsonResult> {
        whitespace
            .and(
                stringval
                    .or(object)
                    .or(array)
                    .or(numberval.or(boolval).or(nullval).map(|v| Ok(v))),
            )
            .and(whitespace)
            .map(|((_, v), _)| v)
            .or_trans(|i: CharSpan, ep| {
                let loc = i.loc_range(ep).unwrap();
                Err(JsonParserError {
                    loc,
                    msg: "Invaild character".to_string(),
                })
            })
            .parse(input)
    }

    fn array(input: CharSpan) -> Option<JsonResult> {
        one('[')
            .and({
                value
                    .and(one(',').and(value).many())
                    .map(|(f, v)| {
                        let mut vals: Vec<JsonVal> = vec![f?];
                        for vv in v {
                            let (_, val) = vv;
                            vals.push(val?);
                        }
                        Ok(vals)
                    })
                    .or(whitespace.map(|_| Ok(vec![])))
            })
            .and(one(']').map(|_| Ok(())).or_trans(|i: CharSpan, ep| {
                let loc = i.loc_range(ep);
                let loc = loc.unwrap();
                Err(JsonParserError {
                    loc,
                    msg: "Need \"]\" but not found it".to_string(),
                })
            }))
            .map(
                |((_, v), e): ((_, JsonResults<Vec<JsonVal>>), JsonResults<()>)| {
                    e?;
                    let v = v?;
                    Ok(JsonVal::Array(v))
                },
            )
            .parse(input)
    }

    fn object(input: CharSpan) -> Option<JsonResult> {
        fn kv(input: CharSpan) -> Option<JsonResults<(String, JsonVal)>> {
            whitespace
                .and(stringval)
                .and(whitespace)
                .and(one(':').map(|v| Ok(v)).or_trans(|i: CharSpan, ep| {
                    let loc = i.loc_range(ep).unwrap();
                    Err(JsonParserError {
                        loc,
                        msg: "Need \":\" but not found it".to_string(),
                    })
                }))
                .and(value)
                .map(|((((_, k), _), col), v)| {
                    let k = k?;
                    let _ = col?;
                    let v = v?;
                    let key = if let JsonVal::String(key) = k {
                        key
                    } else {
                        panic!("never")
                    };
                    Ok((key, v))
                })
                .parse(input)
        }
        one('{')
            .and({
                kv.and({ one(',').and(kv).many() })
                    .map(|(f, vs)| {
                        let mut vals: HashMap<String, JsonVal> = HashMap::new();
                        let (k, v) = f?;
                        vals.insert(k, v);
                        for vv in vs {
                            let (_, val) = vv;
                            let (k, v) = val?;
                            vals.insert(k, v);
                        }
                        Ok(vals)
                    })
                    .or(whitespace.map(|_| Ok(HashMap::new())))
            })
            .and(one('}').map(|_| Ok(())).or_trans(|i: CharSpan, ep| {
                let loc = i.loc_range(ep).unwrap();
                Err(JsonParserError {
                    loc,
                    msg: "Need \"}\" but not found it".to_string(),
                })
            }))
            .map(
                |((_, v), e): ((_, JsonResults<HashMap<String, JsonVal>>), JsonResults<()>)| {
                    e?;
                    let v = v?;
                    Ok(JsonVal::Object(v))
                },
            )
            .parse(input)
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct JsonParserError {
        pub loc: LocRange,
        pub msg: String,
    }
    pub type JsonResults<T> = Result<T, JsonParserError>;
    pub type JsonResult = Result<JsonVal, JsonParserError>;

    #[derive(Debug, Clone, PartialEq)]
    pub enum JsonVal {
        String(String),
        Number(f64),
        Object(HashMap<String, JsonVal>),
        Array(Vec<JsonVal>),
        Bool(bool),
        Null,
    }

    #[test]
    fn test_bool_true() {
        let code = "true".span();
        let r = boolval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(JsonVal::Bool(true)))
    }

    #[test]
    fn test_bool_false() {
        let code = "false".span();
        let r = boolval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(JsonVal::Bool(false)))
    }

    #[test]
    fn test_bool_none() {
        let code = "123".span();
        let r = boolval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, None)
    }

    #[test]
    fn test_null() {
        let code = "null".span();
        let r = nullval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(JsonVal::Null))
    }

    #[test]
    fn test_null_none() {
        let code = "true".span();
        let r = nullval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, None)
    }

    #[test]
    fn test_number() {
        let code = "-123.456e-11".span();
        let r = numberval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(JsonVal::Number(-123.456e-11)))
    }
    #[test]
    fn test_number2() {
        let code = "123.".span();
        let r = numberval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(JsonVal::Number(123.)))
    }
    #[test]
    fn test_number3() {
        let code = "0.456".span();
        let r = numberval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(JsonVal::Number(0.456)))
    }
    #[test]
    fn test_number4() {
        let code = "-0".span();
        let r = numberval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(JsonVal::Number(-0.0)))
    }
    #[test]
    fn test_number5() {
        let code = "123e+123".span();
        let r = numberval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(JsonVal::Number(123e+123)))
    }
    #[test]
    fn test_number6() {
        let code = "123".span();
        let r = numberval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(JsonVal::Number(123f64)))
    }
    #[test]
    fn test_number_none() {
        let code = "asd".span();
        let r = numberval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, None)
    }

    #[test]
    fn test_string() {
        let code = "\"asd\"".span();
        let r = stringval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(Ok(JsonVal::String("asd".to_string()))))
    }

    #[test]
    fn test_string2() {
        let code = "\"asd\\n\"".span();
        let r = stringval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(Ok(JsonVal::String("asd\n".to_string()))))
    }
    #[test]
    fn test_string3() {
        let code = "\"asd\\u2a5f\"".span();
        let r = stringval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, Some(Ok(JsonVal::String("asd\u{2a5f}".to_string()))))
    }

    #[test]
    fn test_string4() {
        let code = "\"asd\n\"".span();
        let r = stringval.parse(code);
        println!("{:?}", r);
        let loc = (4, 0, 4);
        let msg = "Control characters are not allowed in the string".to_string();
        let err = JsonParserError {
            loc: loc.into(),
            msg,
        };
        assert_eq!(r, Some(Err(err)))
    }

    #[test]
    fn test_string5() {
        let code = "\"asd\\a\"".span();
        let r = stringval.parse(code);
        println!("{:?}", r);
        let loc = (5, 0, 5);
        let msg = "Illegal escape character".to_string();
        let err = JsonParserError {
            loc: loc.into(),
            msg,
        };
        assert_eq!(r, Some(Err(err)))
    }

    #[test]
    fn test_string_none() {
        let code = "asd".span();
        let r = stringval.parse(code);
        println!("{:?}", r);
        assert_eq!(r, None)
    }

    #[test]
    fn test_array() {
        let code = "[ 1, 2, 3 ]".span();
        let r = array.parse(code);
        println!("{:?}", r);
        assert_eq!(
            r,
            Some(Ok(JsonVal::Array(vec![
                JsonVal::Number(1.0),
                JsonVal::Number(2.0),
                JsonVal::Number(3.0)
            ])))
        )
    }

    #[test]
    fn test_array2() {
        let code = "[ 1, 2 3 ]".span();
        let r = array.parse(code);
        println!("{:?}", r);
        assert_eq!(
            r,
            Some(Err(JsonParserError {
                loc: (7, 0, 7).into(),
                msg: "Need \"]\" but not found it".to_string()
            }))
        )
    }

    #[test]
    fn test_array3() {
        let code = "[ 1, 2 ".span();
        let r = array.parse(code);
        println!("{:?}", r);
        assert_eq!(
            r,
            Some(Err(JsonParserError {
                loc: (6, 0, 6).into(),
                msg: "Need \"]\" but not found it".to_string()
            }))
        )
    }

    #[test]
    fn test_object() {
        let code = "{ \"a\" : 1 }".span();
        let r = object.parse(code);
        println!("{:?}", r);
        assert_eq!(
            r,
            Some(Ok(JsonVal::Object({
                let mut map = HashMap::new();
                map.insert("a".to_string(), JsonVal::Number(1.0));
                map
            })))
        )
    }

    #[test]
    fn test_object2() {
        let code = "{ \"a\" 1 }".span();
        let r = object.parse(code);
        println!("{:?}", r);
        assert_eq!(
            r,
            Some(Err(JsonParserError {
                loc: (6, 0, 6).into(),
                msg: "Need \":\" but not found it".to_string()
            }))
        )
    }

    #[test]
    fn test_object3() {
        let code = "{ \"a\" : 1 2 }".span();
        let r = object.parse(code);
        println!("{:?}", r);
        assert_eq!(
            r,
            Some(Err(JsonParserError {
                loc: (10, 0, 10).into(),
                msg: "Need \"}\" but not found it".to_string()
            }))
        )
    }
    #[test]
    fn test_object4() {
        let code = "{ \"a\" : 1 ".span();
        let r = object.parse(code);
        println!("{:?}", r);
        assert_eq!(
            r,
            Some(Err(JsonParserError {
                loc: (9, 0, 9).into(),
                msg: "Need \"}\" but not found it".to_string()
            }))
        )
    }
}
