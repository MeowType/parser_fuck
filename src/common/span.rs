use super::cell::*;
use super::*;
use std::ops::Range;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct SpanData<I: Iterator> {
    index: usize,
    ready: bool,
    timeline: Timeline<I>,
}
impl<I: Iterator> SpanData<I> {
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            index: 0,
            ready: true,
            timeline: Timeline::new(iter),
        }
    }
    #[inline]
    pub fn new_restart(&self) -> Self {
        Self {
            index: 0,
            ready: true,
            timeline: self.timeline.clone(),
        }
    }
    // #[inline]
    // pub fn raw_clone(&self) -> Self {
    //     Self {
    //         index: self.index,
    //         ready: self.ready,
    //         timeline: self.timeline.clone(),
    //     }
    // }
}
impl<I: Iterator> Clone for SpanData<I> {
    #[inline]
    fn clone(&self) -> Self {
        self.new_restart()
    }
}

/// A time-travelable collection  
/// See [TimeTravel](trait.TimeTravel.html)
#[derive(Debug, PartialEq, Eq)]
pub struct Span<I: Iterator> {
    inner: Rc<ExtRefCell<SpanData<I>>>,
}
impl<I: Iterator> Span<I> {
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            inner: Rc::new(ExtRefCell::new(SpanData::new(iter))),
        }
    }
}
impl<I: Iterator> From<I> for Span<I> {
    #[inline]
    fn from(iter: I) -> Self {
        Self::new(iter)
    }
}
impl<I: Iterator> Clone for Span<I> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::new(ExtRefCell::new(self.inner.new_restart())),
        }
    }
}
impl<I: Iterator> RefClone for Span<I> {
    fn ref_clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
impl<I: Iterator> Iterator for Span<I>
where
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let this: &mut SpanData<I> = self.inner.get_mut();
        let r = this.timeline.get(this.index)?;
        this.index += 1;
        this.ready = false;
        Some(r.clone())
    }
}
impl<I: Iterator> TimeTravel for Span<I>
where
    I::Item: Clone,
{
    fn get(&mut self, index: usize) -> Option<Self::Item> {
        let this: &mut SpanData<I> = self.inner.get_mut();
        this.timeline.get(index).cloned()
    }
    fn re_ready(&mut self) {
        let this: &mut SpanData<I> = self.inner.get_mut();
        if !this.ready && !(this.timeline.is_end() && this.index >= this.timeline.now_len()) {
            this.index -= 1;
            this.ready = true;
        }
    }
    fn do_ready(&mut self) {
        let this: &mut SpanData<I> = self.inner.get_mut();
        this.ready = true;
    }
    fn is_complete(&self) -> bool {
        self.inner.timeline.is_end()
    }
    fn save(&self) -> usize {
        self.inner.index
    }
    fn back(&mut self, index: usize) {
        let this = self.inner.get_mut();
        this.index = index;
        this.ready = true;
    }
}
impl<I: Iterator> SyncTo for Span<I> {
    fn sync_to(&self, other: &mut Self) {
        other.inner = self.inner.clone();
    }
}
impl<I: Iterator> ComString for Span<I>
where
    I::Item: GetString,
{
    type ComStringData = Range<usize>;

    fn com_string(&self, range: Range<usize>) -> Option<String> {
        let this = self.inner.get();
        let Range { start: _, end } = range;
        if end > this.timeline.now_len() {
            None
        } else {
            let s = this.timeline[range].iter();
            let s = s.map(|c| c.get_string());
            let s: String = s.collect();
            Some(s)
        }
    }
}
impl<I: Iterator> ComLoc for Span<I>
where
    I::Item: GetLoc,
{
    type ComLocData = usize;

    fn loc(&self, index: usize) -> Option<Loc> {
        let this = unsafe { (*self.inner).get_mut() };
        let c = this.timeline.get(index)?;
        let loc = c.loc();
        Some(loc)
    }
}
impl<I: Iterator> ComLocRange for Span<I>
where
    I::Item: GetLoc + Clone,
{
    type ComLocRangeData = Range<usize>;

    fn loc_range(&self, range: Range<usize>) -> Option<LocRange> {
        let this = unsafe { (*self.inner).get_mut() };
        let Range { start, end } = range;
        let now_len = this.timeline.now_len();
        if end > now_len {
            debug_assert!(end > now_len);
            None
        } else {
            debug_assert!(end <= now_len);
            let s = this.timeline.get(start)?.clone();
            let e = this.timeline.get(end - 1)?.clone();
            let loc_range = LocRange::new(s.loc(), e.loc());
            Some(loc_range)
        }
    }
}


/// Into Span
pub trait SpanOf {
    type SpanOfTarget;
    fn span(self) -> Self::SpanOfTarget;
}
