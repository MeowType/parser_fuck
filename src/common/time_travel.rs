use std::ops::Range;

pub trait RefClone: Clone {
    fn ref_clone(&self) -> Self;
}
pub trait TimeTravel: Iterator + RefClone {
    fn get(&mut self, index: usize) -> Option<Self::Item>;
    fn is_complete(&self) -> bool;
    fn re_ready(&mut self);
    fn do_ready(&mut self);
    fn save(&self) -> usize;
    fn back(&mut self, index: usize);
    fn make_range(&self, from: usize) -> Range<usize> {
        from..self.save()
    }
    /// let other = self
    fn sync_to(&self, other: &mut Self);
}

pub trait ComString {
    type ComStringData;

    fn com_string(&self, data: Self::ComStringData) -> Option<String>;
}
pub trait GetString {
    fn get_string(&self) -> String;
}

pub trait ComChar {
    type ComCharData;

    fn com_char(&self, data: Self::ComCharData) -> Option<char>;
}
pub trait GetChar {
    fn get_char(&self) -> char;
}
