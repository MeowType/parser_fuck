use std::ops::Range;

/// Vec<len >= 1>
pub fn range_of_many1(a: Vec<Range<usize>>) -> Range<usize> {
    let start = a.first().unwrap().start;
    let end = a.last().unwrap().end;
    start..end
}

/// (Vec<len >= 1>, Vec<len >= 0>)
pub fn range_of_many1_many((a, b): (Vec<Range<usize>>, Vec<Range<usize>>)) -> Range<usize> {
    let start = a.first().unwrap().start;
    let end = if b.is_empty() { a.last() } else { b.last() }.unwrap().end;
    start..end
}

/// (Vec<len >= 1>, Vec<len >= 1>)
pub fn range_of_many1_many1((a, b): (Vec<Range<usize>>, Vec<Range<usize>>)) -> Range<usize> {
    let start = a.first().unwrap().start;
    let end = b.last().unwrap().end;
    start..end
}

/// (Vec<len >= 0>, Vec<len >= 1>)
pub fn range_of_many_many1((a, b): (Vec<Range<usize>>, Vec<Range<usize>>)) -> Range<usize> {
    let start = if a.is_empty() { b.first() } else { a.first() }
        .unwrap()
        .start;
    let end = b.last().unwrap().end;
    start..end
}

/// (Vec<len >= 0>, Vec<len >= 0>)
pub fn range_of_many_many((a, b): (Vec<Range<usize>>, Vec<Range<usize>>)) -> Option<Range<usize>> {
    if a.is_empty() && b.is_empty() {
        return None;
    }
    let start = if a.is_empty() { b.first() } else { a.first() }
        .unwrap()
        .start;
    let end = if b.is_empty() { a.last() } else { b.last() }.unwrap().end;
    Some(start..end)
}

/// (Range, Vec<len >= 0>)
pub fn range_of_range_many((a, b): (Range<usize>, Vec<Range<usize>>)) -> Range<usize> {
    let start = a.start;
    let end = if b.is_empty() {
        a.end
    } else {
        b.last().unwrap().end
    };
    start..end
}

/// (Range, Vec<len >= 0>)
pub fn range_of_range_many1((a, b): (Range<usize>, Vec<Range<usize>>)) -> Range<usize> {
    let start = a.start;
    let end = b.last().unwrap().end;
    start..end
}

/// (Vec<len >= 0>, Range)
pub fn range_of_many1_range((a, b): (Vec<Range<usize>>, Range<usize>)) -> Range<usize> {
    let start = a.first().unwrap().start;
    let end = b.end;
    start..end
}

/// (Vec<len >= 0>, Range)
pub fn range_of_many_range((a, b): (Vec<Range<usize>>, Range<usize>)) -> Range<usize> {
    let start = if a.is_empty() {
        b.start
    } else {
        a.first().unwrap().start
    };
    let end = b.end;
    start..end
}

/// (Range, Range)
pub fn range_of_range_range((a, b): (Range<usize>, Range<usize>)) -> Range<usize> {
    let start = a.start;
    let end = b.end;
    start..end
}

/// (Option<Range>, Range)
pub fn range_of_optrange_range((a, b): (Option<Range<usize>>, Range<usize>)) -> Range<usize> {
    let start = if let Some(a) = a { a.start } else { b.start };
    let end = b.end;
    start..end
}

/// (Range, Option<Range>)
pub fn range_of_range_optrange((a, b): (Range<usize>, Option<Range<usize>>)) -> Range<usize> {
    let start = a.start;
    let end = if let Some(b) = b { b.end } else { a.end };
    start..end
}

/// (Option<Range>, Option<Range>)
pub fn range_of_optrange_optrange(
    (a, b): (Option<Range<usize>>, Option<Range<usize>>),
) -> Option<Range<usize>> {
    let start = if a.is_some() { a.clone() } else { b.clone() }.map(|v| v.start)?;
    let end = if b.is_some() { b } else { a }.map(|v| v.end)?;
    Some(start..end)
}

/// try but no return
/// # Example
/// ```rust
/// # use parser_fuck::*;
/// let a = Some(1);
/// let r = try_do!(a; Some(v) {
///     Some(v + 1)
/// });
/// assert_eq!(r, Some(2));
/// ```
/// ```rust
/// # use parser_fuck::*;
/// let a: Result<i32, ()> = Ok(1);
/// let r = try_do!(a; Ok(v) {
///     Ok(v + 1)
/// });
/// assert_eq!(r, Ok(2));
/// ```
#[macro_export(local_inner_macros)]
macro_rules! try_do {
    { $v:expr ; Some($n:ident) $b:block } => {
        match $v {
            Some($n) => $b
            None => None
        }
    };
    { $v:expr ; Ok($n:ident) $b:block } => {
        match $v {
            Ok($n) => $b
            Err(err) => Err(err)
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_try_do_some() {
        let a = Some(1);
        let r = try_do!(a; Some(v) {
            Some(v + 1)
        });
        assert_eq!(r, Some(2));
    }
    #[test]
    fn test_try_do_none() {
        let a: Option<i32> = None;
        let r = try_do!(a; Some(v) {
            Some(v + 1)
        });
        assert_eq!(r, None);
    }

    #[test]
    fn test_try_do_ok() {
        let a: Result<i32, ()> = Ok(1);
        let r = try_do!(a; Ok(v) {
            Ok(v + 1)
        });
        assert_eq!(r, Ok(2));
    }

    #[test]
    fn test_try_do_err() {
        let a: Result<i32, ()> = Err(());
        let r = try_do!(a; Ok(v) {
            Ok(v + 1)
        });
        assert_eq!(r, Err(()));
    }
}
