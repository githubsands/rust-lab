// questions:
//
// 1. how does a channel enforce only once semantics?
// 2. difference of Vec and VecDeque
// 3. can channels be based between channels in rust?
// 4. what is a condvar?
// 5. how does the internal type system deal with "T"
// 6. why is a condvar an "item_ready"?
// 7. in what circumstance would we need to use Send or Sync traits?
// 8. what about he compiler understands Mutex, and item_ready instead of other types?
// 9. how can we manipulate this channel to not grow and have bounds?
// 10. what are the downsides of mutex mentioned in this excercise?
// Condvar notes: will unlock the Mutex while waiting and relock before returning
// 1. notify_one:
// 2. pop_front:
pub struct Channel<T> {
    queue: Mutex<VecDeqeue<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }
    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }
    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }
        }
        b = self.item_ready.wait(b).unwrap();
    }
}

// using a UnsafeCell for storage and AtomicBool to indicate state.

// only-once implementation:

use std::mem::MaybeUnint;

pub struct ChannelOneShot<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

impl<T> ChannelOneShot<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }
}

pub unsafe fn send(&self, message: T) {
    (*self.message.get()).write(message);
    self.ready.store(true, Release);
}

pub fn is_ready(&self) -> bool {
    self.ready.load(Acquire)
}

pub unsafe fn receive(&self) -> T {
    (*self.message.get()).assume_init_read()
}
