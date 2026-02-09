
use core::cell::UnsafeCell;
use core::ops::Deref;

pub struct SyncUnsafeCell<T>(UnsafeCell<T>);

impl<T> SyncUnsafeCell<T> {
    pub const fn new(inner: T) -> Self {
        SyncUnsafeCell(UnsafeCell::new(inner))
    }
}

impl<T> Deref for SyncUnsafeCell<T> {
    type Target = UnsafeCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl<T> Sync for SyncUnsafeCell<T> {}