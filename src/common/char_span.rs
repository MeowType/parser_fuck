use crate::*;
use std::str::Chars;

pub type CharSpan<'a> = Span<CharChars<Chars<'a>>>;

impl<'a> SpanOf for CharChars<Chars<'a>> {
    type SpanOfTarget = CharSpan<'a>;

    #[inline]
    fn span(self) -> Self::SpanOfTarget {
        CharSpan::new(self)
    }
}

impl<'a> SpanOf for Chars<'a> {
    type SpanOfTarget = CharSpan<'a>;

    #[inline]
    fn span(self) -> Self::SpanOfTarget {
        CharSpan::new(CharChars::new(self))
    }
}

impl<'a> SpanOf for &'a str {
    type SpanOfTarget = CharSpan<'a>;

    #[inline]
    fn span(self) -> Self::SpanOfTarget {
        CharSpan::new(CharChars::new(self.chars()))
    }
}

impl<'a> SpanOf for &'a String {
    type SpanOfTarget = CharSpan<'a>;

    #[inline]
    fn span(self) -> Self::SpanOfTarget {
        CharSpan::new(CharChars::new(self.chars()))
    }
}