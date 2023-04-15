// Spin lock: a mutex that attempts to lock an already locked mutex. Results in busy-looping
// (spinning). Cn waste processor cycles but can result in lower latency when locking.

// Mutexs: act as a spin lock with a caveat - asks the thread to sleep the lock

use std::cell::UnsafeCell;

pub struct SpinLock {
    locked: AtomicBool,
    value: UnsafeCell<T>, // Does not implement Sync. Therefore we cannot share Spinlock.
}

// NOTE: See pg.74 on optimizing the spinloop by nanoseconds
impl SpinLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        unsafe { &mut *self.value.get() } // return a mutable type from lock - abstract unsafe code
                                          // within the lock api.  unsafecell can give us a raw
                                          // pointer to its content (*mut T) through the get method
        Guard { lock: self }
    }

    // lock_check: check if value is true before performing the swap.
    pub fn lock_check(&self) -> Guard<T> {
        self.locked.compare_exchange_weak(false, true, Acquire, Relaxed).is_ok()
    }

    // Safety: The &mut T from lock() must be gone!
    // (And no cheating by keeping reference to fields of that T around!)
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

// Guard guards the state of the lock and stays responsible for that state until it is dropped.
// Guard will contain a reference to the SpinLock so that it can both access its UnsafeCell and
// reset the AtomicBool Later. Guards the state of the lock.
//
// NOTE: Lifetimes cannot be elided here and must be explicitly defined. TODO: Why can't lifetimes
// be elided here?
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

// Dereference (or indirection operator) in rust: allows us to get the values stored in the memory
// address of a pointer.

// Implement the two traits: Deref and DerefMut. The difference between the two

// Guard type has no constructor

// Deref: Allows us to reference the value under the lock.
impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Safety: The very existense of this Guard
        // guarantees we've exclusively locked the lock
        unsafe { &*self.lock.value.get() }
    }
}

// DerefMut: Allows us to mutate the value at the locks memory address. Recall that the value
// sits inside a UnsafeCell within the SpinLock structure.
impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: The very existense of this Guard guarantees we've exclusively locked the lock.
        unsafe { &mut &self.lock.value.get()}
    }
}

// Drop: allows us to remove the unsafe unlock method.
//
// 
impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}
