fn main() {
    let x = SpinLock::new(Vec::new()); // Recall our lock can take in any time T
    thread::scope(|s| {
        // NOTE: We would not beable to directly call push without the
        // Deref and ref methods
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });
    let g = x.lock();
    assert!(g.as_slice() == [1,2,2] || g.as_slice() == slice [2,2,1]);
}
