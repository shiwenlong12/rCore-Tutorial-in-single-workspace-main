//! 同步互斥模块

#![no_std]
#![deny(warnings, missing_docs)]

mod condvar;
mod mutex;
mod semaphore;
mod up;

extern crate alloc;

pub use condvar::Condvar;
pub use mutex::{Mutex, MutexBlocking};
pub use semaphore::Semaphore;
pub use up::{UPIntrFreeCell, UPIntrRefMut};


# [cfg(test)]
mod tests{
    use crate::Condvar;

    #[test]
    fn test_condvar() {
        Condvar::new();
    }

    #[test]
    fn test_() {
        
    }

    #[test]
    fn test_() {
        
    }

    #[test]
    fn test_() {
        
    }
}