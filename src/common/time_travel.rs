use std::ops::Range;

/// RefClone clone a shared reference, equivalent to Clone on e.g. Rc, Arc
pub trait RefClone: Clone {
    /// Clone a shared reference
    fn ref_clone(&self) -> Self;
}
impl<T> RefClone for std::rc::Rc<T> {
    #[inline]
    fn ref_clone(&self) -> Self {
        self.clone()
    }
}
impl<T> RefClone for std::rc::Weak<T> {
    #[inline]
    fn ref_clone(&self) -> Self {
        self.clone()
    }
}
impl<T> RefClone for std::sync::Arc<T> {
    #[inline]
    fn ref_clone(&self) -> Self {
        self.clone()
    }
}
impl<T> RefClone for std::sync::Weak<T> {
    #[inline]
    fn ref_clone(&self) -> Self {
        self.clone()
    }
}

/// Abstraction of a timeline that stores historical records
pub trait TimeTravel: Iterator + RefClone + SyncTo {
    /// Get the value of the specified position  
    /// - None if the timeline is completed but not found  
    /// - None if index is less than 0  
    fn get(&mut self, index: usize) -> Option<Self::Item>;
    /// Check if the timeline is complete
    fn is_complete(&self) -> bool;
    /// Check is ready to next again
    fn re_ready(&mut self);
    /// Re-ready to next
    fn do_ready(&mut self);
    /// Save the current time point
    fn save(&self) -> usize;
    /// Time travel to a point in history
    fn back(&mut self, index: usize);
    /// Calculate the range from save point to current
    fn make_range(&self, from: usize) -> Range<usize> {
        from..self.save()
    }
}

/// Make another instance equal to yourself  
pub trait SyncTo {
    /// let other = self
    fn sync_to(&self, other: &mut Self);
}

/// Calculate String
pub trait ComString {
    type ComStringData;

    /// Calculate String
    fn com_string(&self, data: Self::ComStringData) -> Option<String>;
}
/// Get String
pub trait GetString {
    /// Get String
    fn get_string(&self) -> String;
}

/// Calculate char
pub trait ComChar {
    type ComCharData;

    /// Calculate char
    fn com_char(&self, data: Self::ComCharData) -> Option<char>;
}
/// Get char
pub trait GetChar {
    /// Get char
    fn get_char(&self) -> char;
}
