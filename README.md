 Rust Lab

Resources for understanding rust **_fundamentals_** on an **_indepth level_**

## I. Resources
  
Resources loosely ordered by difficulty.

[rust by example](https://doc.rust-lang.org/rust-by-example/index.html)

[rust easy](https://dhghomon.github.io/easy_rust/Chapter_0.html)

[rust for c++ programmers](https://aminb.gitbooks.io/rust-for-c/content/index.html)

[rust cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)

[rust move](https://move-language.github.io/move/)
    
[rust book](https://doc.rust-lang.org/book/)
  
[rust traits](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md)

[rust effective](https://www.lurklurk.org/effective-rust/cover.html)

[rust traits async](https://rust-lang.github.io/async-book/02_execution/02_future.html)

[rust-macros](https://danielkeep.github.io/tlborm/book/README.html)
  
[rust-async-book](https://github.com/rust-lang/async-book)
 
[rust-inaction](https://www.rustinaction.com/)

[rust scripting](https://rhai.rs/book/)

[rust embedded](https://docs.rust-embedded.org/book/)

[rust linked list](https://rust-unofficial.github.io/too-many-lists/)
  
[rust atomic locks book](https://www.goodreads.com/en/book/show/63291820)

[c++ atomics documentation](https://en.cppreference.com/w/cpp/atomic/memory_order)

[c++ atomics flaws](https://plv.mpi-sws.org/c11comp/popl15.pdf)

[rust elements](https://github.com/ferrous-systems/elements-of-rust)

[rust contiguous data](https://github.com/paulkernfeld/contiguous-data-in-rust)

[rust performance book](https://nnethercote.github.io/perf-book/title-page.html)

[rust performance book #2](https://www.amazon.com/Rust-High-Performance-performance-applications/dp/178839948X)

[rust performance code](https://github.com/PacktPublishing/Rust-High-Performance)

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


## Scope

1. [drop scopes](https://doc.rust-lang.org/reference/destructors.html#destructors)

    The entire function scope is the outer most scope.
    
    The function body block is contained within the scope of the entire function.
    
    The parent of the expression in an expression statement is the scope of the statement.
    
    The parent of the initializer of a let statement is the let statement's scope.
    
    The parent of a statement scope is the scope of the block that contains the statement.
    
    The parent of the expression for a match guard is the scope of the arm that the guard is for.
    
    The parent of the expression after the => in a match expression is the scope of the arm that it's in.
    
    The parent of the arm scope is the scope of the match expression that it belongs to.
    
    The parent of all other scopes is the scope of the immediately enclosing expression.

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

###  Unwinding order of operations:

| Tiered Error-Handling Scheme |
|------------------------------|
| 1. Option: If something might reasonably be absent |
| 2. Result: If something goes wrong and can reasonably be handled |
| 3. Thread panic: If something goes wrong and cannot reasonably be handled |
| 4. Program abort: If something catastrophic happens |

