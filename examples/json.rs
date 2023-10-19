use parser_fuck::*;
use std::collections::HashMap;
use std::f64;
use std::ops::Range;

static CODE: &'static str =
    "{ \"a\": 1, \"b\": true, \"c\": [null, 1.5, false], \"d\": { \"v\": \"asd\" } }";

fn main() {
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
                            satisfy(|c: Char| c.char(|c: char, _| c.is_digit(16)).unwrap_or(false))
                                .some(4),
                        )
                        .map(|(_, u)| {
                            let s = input.com_string(range_of_many1(u)).unwrap();
                            let hex: u32 = u32::from_str_radix(&*s, 16).unwrap();
                            let c = std::char::from_u32(hex).unwrap();
                            c
                        }))
                    .map(|v| Result::<char, JsonParserError>::Ok(v))
                    .or_trans(true, |i: CharSpan, ep| {
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
        .or_trans(true, |i: CharSpan, ep| {
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
        .and(one(']').map(|_| Ok(())).or_trans(true, |i: CharSpan, ep| {
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
            .and(one(':').map(|v| Ok(v)).or_trans(true, |i: CharSpan, ep| {
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
            kv.and(one(',').and(kv).many())
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
        .and(one('}').map(|_| Ok(())).or_trans(true, |i: CharSpan, ep| {
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
