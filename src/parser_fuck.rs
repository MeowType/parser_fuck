use super::*;
use crate::*;
use std::any::Any;

pub trait ParserError: Any {
    #[inline]
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized,
    {
        self
    }
}
