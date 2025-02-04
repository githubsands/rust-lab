 Rust Lab

Resources for understanding rust **_fundamentals_** on an **_indepth level_**

## I. Resources
  
Resources loosely ordered by difficulty.

[rust by example](https://doc.rust-lang.org/rust-by-example/index.html)

[rust easy](https://dhghomon.github.io/easy_rust/Chapter_0.html)

[rust for c++ programmers](https://aminb.gitbooks.io/rust-for-c/content/index.html)

[learn cpp](https://www.learncpp.com/)

[rust cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)

[rust too many linked list](https://rust-unofficial.github.io/too-many-lists/)

[rust move](https://move-language.github.io/move/)
    
[rust book](https://doc.rust-lang.org/book/)

[rustup](https://rust-lang.github.io/rustup/index.html)
  
[rust traits](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md)

[rust effective](https://www.lurklurk.org/effective-rust/cover.html)

[drop scopes](https://doc.rust-lang.org/reference/destructors.html#destructors)

[rust traits async](https://rust-lang.github.io/async-book/02_execution/02_future.html)

[rust-macros](https://danielkeep.github.io/tlborm/book/README.html)
  
[rust-async-book](https://github.com/rust-lang/async-book)

[wasm 1](https://rustwasm.github.io/docs/book/)

[wasm 2](https://rustwasm.github.io/wasm-bindgen/)
 
[rust-inaction](https://www.rustinaction.com/)

[rust scripting](https://rhai.rs/book/)

[rust embedded](https://docs.rust-embedded.org/book/)

[rust linked list](https://rust-unofficial.github.io/too-many-lists/)
  
[rust atomic locks book](https://www.goodreads.com/en/book/show/63291820)

[rust lock free blog](https://morestina.net/blog/749/exploring-lock-free-rust-2-atomics)

[c++ RAII](https://en.cppreference.com/w/cpp/language/raii)

[c++ atomics documentation](https://en.cppreference.com/w/cpp/atomic/memory_order)

[c++ lock free](https://en.cppreference.com/w/cpp/atomic/atomic/is_lock_free)

[c++ atomics flaws](https://plv.mpi-sws.org/c11comp/popl15.pdf)

[inline assembly](https://doc.rust-lang.org/reference/inline-assembly.html)

[latency tuning](https://rigtorp.se/low-latency-guide/)

[cache coherence continued](https://marabos.nl/atomics/hardware.html)

[rust elements](https://github.com/ferrous-systems/elements-of-rust)

[c programming guide](https://beej.us/guide/bgc/html/split/)

[rust contiguous data](https://github.com/paulkernfeld/contiguous-data-in-rust)

[rust performance book](https://nnethercote.github.io/perf-book/title-page.html)

[rust performance book #2](https://www.amazon.com/Rust-High-Performance-performance-applications/dp/178839948X)

[rust performance code](https://github.com/PacktPublishing/Rust-High-Performance)

[other](https://quinedot.github.io/rust-learning/index.html)

[cargo](https://doc.rust-lang.org/cargo/)

[rust performance blog](http://troubles.md/posts/rust-optimization/)

[rust vs C clang benchmarks](https://benchmarksgame-team.pages.debian.net/benchmarksgame/fastest/rust-clang.html)

[rust bound checks](https://github.com/Shnatsel/bounds-check-cookbook/)

[rust gpu programming](https://rust-gpu.github.io/Rust-CUDA/)

[rust for functional programmers](https://dr-knz.net/rust-for-functional-programmers.pdf)

[rust bound check blog]([https://shnatsel.medium.com/how-to-avoid-bounds-checks-in-rust-without-unsafe-f65e618b4c1e])

[rust chalk](https://rust-lang.github.io/chalk/book/what_is_chalk.html)

[rust borrow-system polonius](https://github.com/rust-lang/polonius)

[rust nomicon](https://doc.rust-lang.org/nomicon/intro.html)

[rust_compiler](https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html)

[rust rfcs](https://rust-lang.github.io/rfcs/introduction.html)

### Commmunity

[rust blog](https://users.rust-lang.org/)

### Datatypes reference size x64

| Rust Type | Description | Size (bytes) |
|-----------|-------------|-------------|
| `()`      | The unit type, also known as "unit" | 0 |
| `bool`    | The boolean type | 1 |
| `char`    | A character type | 4 |
| `u8`      | An 8-bit unsigned integer | 1 |
| `u16`     | A 16-bit unsigned integer | 2 |
| `u32`     | A 32-bit unsigned integer | 4 |
| `u64`     | A 64-bit unsigned integer | 8 |
| `u128`    | A 128-bit unsigned integer | 16 |
| usize   | The pointer-sized unsigned integer type | 32 on a x32; 64 on a x64|
| `i8`      | An 8-bit signed integer | 1 |
| `i16`     | A 16-bit signed integer | 2 |
| `i32`     | A 32-bit signed integer | 4 |
| `i64`     | A 64-bit signed integer | 8 |
| `i128`    | A 128-bit signed integer | 16 |
| `f32`     | A 32-bit floating point number | 4 |
| `f64`     | A 64-bit floating point number | 8 |
| `*const T` | Raw, unsafe pointer to a value of type `T` | 8 |
| `*mut T`  | Mutable raw, unsafe pointer to a value of type `T` | 8 |
| `&T`      | Shared reference to a value of type `T` | 8 |
| `&mut T`  | Mutable reference to a value of type `T` | 8 |
| `[T; N]`  | A fixed-size array of `N` elements of type `T` | `N * size_of::<T>()` |
| `[T]`     | A dynamically-sized slice of elements of type `T` | 16 (2 machine words) |
| `str`     | A UTF-8 encoded string slice | 16 (2 machine words) |
| `(T1, T2, ..)` | A tuple of values of types `T1`, `T2`, etc. | `size_of::<T1>() + size_of::<T2>() + ..` |
| `fn(usize) -> bool` | Function pointer | 16 (2 machine words) |
| `!`      | The "never" type, which has no values | 0 |
