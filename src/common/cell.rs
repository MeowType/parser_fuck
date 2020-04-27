use std::cell::UnsafeCell;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
pub struct ExtRefCell<T: ?Sized> {
    value: UnsafeCell<T>,
}
impl<T> ExtRefCell<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }
    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
    #[inline]
    pub fn get(&self) -> &T {
        unsafe { &*self.value.get() }
    }
    #[inline]
    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.value.get()
    }
}
impl<T> Deref for ExtRefCell<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
impl<T> DerefMut for ExtRefCell<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value.get() }
    }
}
impl<T: Clone> Clone for ExtRefCell<T> {
    #[inline]
    fn clone(&self) -> ExtRefCell<T> {
        ExtRefCell::new(self.deref().clone())
    }
}
impl<T> From<T> for ExtRefCell<T> {
    #[inline]
    fn from(t: T) -> ExtRefCell<T> {
        ExtRefCell::new(t)
    }
}
impl<T: PartialEq> PartialEq for ExtRefCell<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.get().eq(other.get())
    }
}
impl<T: Eq> Eq for ExtRefCell<T> {}
impl<T: PartialOrd> PartialOrd for ExtRefCell<T> {
    #[inline]
    fn partial_cmp(&self, other: &ExtRefCell<T>) -> Option<Ordering> {
        self.get().partial_cmp(other.get())
    }
    #[inline]
    fn lt(&self, other: &ExtRefCell<T>) -> bool {
        self.get().lt(other.get())
    }
    #[inline]
    fn le(&self, other: &ExtRefCell<T>) -> bool {
        self.get().le(other.get())
    }
    #[inline]
    fn gt(&self, other: &ExtRefCell<T>) -> bool {
        self.get().gt(other.get())
    }
    #[inline]
    fn ge(&self, other: &ExtRefCell<T>) -> bool {
        self.get().ge(other.get())
    }
}
impl<T: Ord> Ord for ExtRefCell<T> {
    #[inline]
    fn cmp(&self, other: &ExtRefCell<T>) -> Ordering {
        self.get().cmp(other.get())
    }
}

//\/////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ExtRefCellExt {
    type Target;
    fn get_mut(&mut self) -> &mut Self::Target;
}
impl<T, D: Deref<Target = ExtRefCell<T>>> ExtRefCellExt for D {
    type Target = T;
    #[inline]
    fn get_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value.get() }
    }
}
