use crate::common::cell::*;
use crate::common::*;
use std::cmp::{Eq, PartialEq};
use std::convert::From;
use std::default::Default;
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;
use std::marker::PhantomData;
use std::ops::{Deref, Index, Range, RangeFrom, RangeFull, RangeTo};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Default)]
struct TimelineBox<I: Iterator> {
    uuid: Uuid,
    pub iter: I,
    pub buf: Vec<I::Item>,
    pub end: bool,
}
impl<I: Iterator> TimelineBox<I> {
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            iter,
            buf: vec![],
            end: false,
        }
    }
}
impl<I: Iterator> PartialEq for TimelineBox<I> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
impl<I: Iterator> Eq for TimelineBox<I> {}
impl<I: Iterator> Hash for TimelineBox<I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state)
    }
}
impl<I: Iterator> Debug for TimelineBox<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TimelineBox")
            .field("uuid", &self.uuid)
            .field("iter", &"...")
            .field("buf", &"...")
            .field("end", &self.end)
            .finish()
    }
}

//\/////////////////////////////////////////////////////////////////////////////////////////////////

/// Collection of storage history  
/// Wrap an iterator
pub struct Timeline<I: Iterator> {
    inner: Rc<ExtRefCell<TimelineBox<I>>>,
}
impl<I: Iterator> Timeline<I> {
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            inner: Rc::new(ExtRefCell::new(TimelineBox::new(iter))),
        }
    }
    #[inline]
    pub fn iter(&self) -> TimelineIter<'_, I> {
        TimelineIter::new(self.clone())
    }
    /// Number of history records currently stored
    #[inline]
    pub fn now_len(&self) -> usize {
        self.inner.buf.len()
    }
    pub fn next(&mut self) -> Option<&I::Item> {
        if self.inner.end {
            return None;
        } else {
            let r = self.inner.get_mut().iter.next();
            if let Some(v) = r {
                self.inner.get_mut().buf.push(v);
            } else {
                self.inner.get_mut().end = true;
                return None;
            }
        }
        self.inner.buf.get(self.inner.buf.len() - 1)
    }
    /// Iterate internal iterator to completion
    pub fn to_end(&mut self) {
        while let Some(_) = self.next() {}
    }
    /// Check if the internal iterator has been completed
    pub fn is_end(&self) -> bool {
        self.inner.end
    }
    /// Get the value of the specified position  
    /// - None if the internal iterator is completed but not found  
    /// - None if index is less than 0  
    pub fn get(&mut self, index: usize) -> Option<&I::Item> {
        unsafe { self.unsafe_get(index) }
    }
    #[inline]
    unsafe fn unsafe_get(&self, index: usize) -> Option<&I::Item> {
        if index >= self.inner.buf.len() {
            if self.inner.end {
                return None;
            } else {
                loop {
                    let r = (*self.inner).get_mut().iter.next();
                    if let Some(v) = r {
                        (*self.inner).get_mut().buf.push(v);
                    } else {
                        (*self.inner).get_mut().end = true;
                        return None;
                    }
                    if index < self.inner.buf.len() {
                        break;
                    }
                }
            }
        }
        self.inner.buf.get(index)
    }
}
impl<I: Iterator> Debug for Timeline<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TimeLine")
            .field("inner", &self.inner)
            .finish()
    }
}
impl<I: Iterator> Default for Timeline<I>
where
    I: Default,
    I::Item: Default,
{
    fn default() -> Self {
        Self {
            inner: Rc::new(ExtRefCell::new(TimelineBox::default())),
        }
    }
}
impl<I: Iterator> From<Rc<ExtRefCell<TimelineBox<I>>>> for Timeline<I> {
    #[inline]
    fn from(inner: Rc<ExtRefCell<TimelineBox<I>>>) -> Self {
        Self { inner }
    }
}
impl<I: Iterator> Clone for Timeline<I> {
    #[inline]
    fn clone(&self) -> Self {
        self.inner.clone().into()
    }
}
impl<I: Iterator> RefClone for Timeline<I> {
    #[inline]
    fn ref_clone(&self) -> Self {
        self.clone()
    }
}
impl<I: Iterator> PartialEq for Timeline<I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.deref().eq(other.inner.deref())
    }
}
impl<I: Iterator> Eq for Timeline<I> {}
impl<I: Iterator> Hash for Timeline<I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}
impl<I: Iterator> Index<Range<usize>> for Timeline<I> {
    type Output = [I::Item];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.inner.buf[index]
    }
}
impl<I: Iterator> Index<RangeFrom<usize>> for Timeline<I> {
    type Output = [I::Item];
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.inner.buf[index]
    }
}
impl<I: Iterator> Index<RangeTo<usize>> for Timeline<I> {
    type Output = [I::Item];
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.inner.buf[index]
    }
}
impl<I: Iterator> Index<RangeFull> for Timeline<I> {
    type Output = [I::Item];
    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.inner.buf[index]
    }
}
impl<I: Iterator> Index<usize> for Timeline<I> {
    type Output = I::Item;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner.buf[index]
    }
}

//\/////////////////////////////////////////////////////////////////////////////////////////////////

/// TimeLine's Iter
pub struct TimelineIter<'a, I: Iterator> {
    timeline: Timeline<I>,
    _a: PhantomData<&'a I::Item>,
}
impl<'a, I: Iterator> TimelineIter<'a, I> {
    #[inline]
    pub fn new(timeline: Timeline<I>) -> Self {
        Self {
            timeline,
            _a: PhantomData,
        }
    }
}
impl<'a, I: Iterator> Iterator for TimelineIter<'a, I>
where
    I::Item: 'a,
{
    type Item = &'a I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.timeline.next().map(|v| unsafe { &*(v as *const _) })
    }
}

#[cfg(test)]
mod tests {
    use super::Timeline;

    macro_rules! assert_get {
        { $iter:expr, $index:expr, $v:expr } => {
            if let Some(v) = $iter.get($index) {
                assert_eq!(*v, $v);
            } else {
                assert!(false);
            }
        };
    }

    macro_rules! assert_next {
        { $iter:expr, $v:expr } => {
            if let Some(v) = $iter.next() {
                assert_eq!(*v, $v);
            } else {
                assert!(false);
            }
        };
    }

    #[test]
    fn test_base() {
        let code = "asd123";
        let mut iter = Timeline::new(code.chars());

        assert_get!(iter, 0, 'a');
        assert_get!(iter, 1, 's');
        assert_get!(iter, 2, 'd');

        assert_get!(iter, 3, '1');
        assert_get!(iter, 4, '2');
        assert_get!(iter, 5, '3');

        assert!(matches!(iter.get(6), None))
    }

    #[test]
    fn test_re_get() {
        let code = "asd123";
        let mut iter = Timeline::new(code.chars());

        assert_get!(iter, 0, 'a');
        assert_get!(iter, 0, 'a');
        assert_get!(iter, 0, 'a');

        assert_get!(iter, 5, '3');

        assert_get!(iter, 3, '1');
    }

    #[test]
    fn test_next() {
        let code = "asd123";
        let mut iter = Timeline::new(code.chars());

        assert_next!(iter, 'a');
        assert_next!(iter, 's');
        assert_next!(iter, 'd');

        assert_next!(iter, '1');
        assert_next!(iter, '2');
        assert_next!(iter, '3');

        assert!(matches!(iter.next(), None))
    }
}
