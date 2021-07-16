use winapi::um::fileapi::UnlockFile;

use std::ops;
use std::os::windows::prelude::*;

use crate::FdLock;

use super::utils::syscall;

/// RAII structure used to release the exclusive write access of a lock when
/// dropped.
///
/// This structure is created by the [`write`] and [`try_write`] methods
/// on [`FdLock`].
///
/// [`write`]: FdLock::write
/// [`try_write`]: FdLock::try_write
///
/// # Panics
///
/// Dropping this type may panic if the lock fails to unlock.
#[must_use = "if unused the FdLock will immediately unlock"]
#[derive(Debug)]
pub struct FdLockWriteGuard<'fdlock, T: AsRawHandle> {
    pub(crate) lock: &'fdlock mut FdLock<T>,
}

impl<T: AsRawHandle> ops::Deref for FdLockWriteGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.inner
    }
}

impl<T: AsRawHandle> ops::DerefMut for FdLockWriteGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lock.inner
    }
}

impl<T: AsRawHandle> Drop for FdLockWriteGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let handle = self.lock.inner.as_raw_handle();
        syscall(unsafe { UnlockFile(handle, 0, 0, 1, 0) })
            .expect("Could not unlock the file descriptor");
    }
}
