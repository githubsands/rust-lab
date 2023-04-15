 Rust Lab

Resources for understanding rust **_fundamentals_** on an **_indepth level_**

## I. Resources
  
Resources loosely ordered by difficulty.

[rust by example](https://doc.rust-lang.org/rust-by-example/index.html)

[rust easy](https://dhghomon.github.io/easy_rust/Chapter_0.html)

[rust for c++ programmers](https://aminb.gitbooks.io/rust-for-c/content/index.html)

[rust move](https://move-language.github.io/move/)
    
[rust book](https://doc.rust-lang.org/book/)
  
[rust traits](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md)
[rust traits async](https://rust-lang.github.io/async-book/02_execution/02_future.html)
  
[rust-async-book](https://github.com/rust-lang/async-book)
 
[rust-inaction](https://www.rustinaction.com/)

[rust scripting][https://rhai.rs/book/]

[rust embedded](https://docs.rust-embedded.org/book/)

[rust async continue][https://rust-lang.github.io/wg-async/welcome.html]
  
[rust atomic locks book](https://www.goodreads.com/en/book/show/63291820)

[rust performance book](https://nnethercote.github.io/perf-book/title-page.html)

[rust bound checks](https://github.com/Shnatsel/bounds-check-cookbook/)

[rust borrow-system polonius](https://github.com/rust-lang/polonius)
  
[rust nomicon](https://doc.rust-lang.org/nomicon/intro.html)

[rust rfcs](https://rust-lang.github.io/rfcs/introduction.html)

## II. Indepth:

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

### III. Datatypes reference size x64

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

###  IV. Unwinding order of operations:

| Tiered Error-Handling Scheme |
|------------------------------|
| 1. Option: If something might reasonably be absent |
| 2. Result: If something goes wrong and can reasonably be handled |
| 3. Thread panic: If something goes wrong and cannot reasonably be handled |
| 4. Program abort: If something catastrophic happens |


### V. Iterators

https://doc.rust-lang.org/std/iter/trait.Iterator.html

| Iterator Method | Description | Example |
|-----------------|-------------|---------|
| `.chain()` | Chains two iterators together, so they are processed sequentially. | `iter1.chain(iter2)` |
| `.cloned()` | Creates an iterator that clones each element of the original iterator. | `iter.cloned()` |
| `.collect()` | Collects elements from the iterator into a collection (e.g., a `Vec` or a `HashMap`). | `iter.collect::<Vec<_>>()` |
| `.count()` | Returns the number of elements in the iterator. | `iter.count()` |
| `.cycle()` | Repeats the iterator indefinitely. | `iter.cycle()` |
| `.enumerate()` | Adds an index to each element of the iterator. | `iter.enumerate()` |
| `.filter()` | Filters the iterator based on a predicate function. | `iter.filter(|x| x % 2 == 0)` |
| `.filter_map()` | Applies a function to each element of the iterator and filters the `None` values. | `iter.filter_map(|x| x.checked_add(1))` |
| `.find()` | Returns the first element that satisfies a predicate function, if any. | `iter.find(|x| x % 2 == 0)` |
| `.find_map()` | Applies a function to each element and returns the first `Some(value)` returned, if any. | `iter.find_map(|x| x.checked_add(1))` |
| `.flat_map()` | Maps an iterator by a function and then flattens the result. | `iter.flat_map(|x| x.iter())` |
| `.flatten()` | Flattens nested iterators into a single iterator. | `iter.flatten()` |
| `.fold()` | Folds the iterator using a binary function, starting with an initial accumulator value. | `iter.fold(0, |acc, x| acc + x)` |
| `.inspect()` | Calls a function for each element of the iterator, passing a reference to it, and allows you to inspect the iterator elements. | `iter.inspect(|x| println!("Element: {:?}", x))` |
| `.is_partitioned()` | Checks if the iterator is partitioned according to a predicate function. | `iter.is_partitioned(|x| x % 2 == 0)` |
| `.last()` | Returns the last element of the iterator, if any. | `iter.last()` |
| `.map()` | Applies a function to each element of the iterator. | `iter.map(|x| x * 2)` |
| `.max()` | Returns the maximum element of the iterator, if any. | `iter.max()` |
| `.max_by()` | Returns the maximum element of the iterator based on a comparator function, if any. | `iter.max_by(|a, b| a.cmp(b))` |
| `.max_by_key()` | Returns the maximum element of the iterator based on a key extraction function, if any. | `iter.max_by_key(|x| x.abs())` |
| `.min()` | Returns the minimum element of the iterator, if any. | `iter.min()` |
| `.min_by()` | Returns the minimum element of the iterator based on a comparator function, if any. | `iter.min_by(|a, b| a.cmp(b))` |
| `.min_by_key()` | Returns the minimum element of the iterator based on a key extraction function, if any. | `iter.min_by_key(|x| x.abs())` |
| `.nth()` | Returns the nth element of the iterator, if any. | `iter.nth(3)` |
| `.peekable()` | Creates a peekable iterator, allowing you to peek at the next element without consuming it. | `iter.peekable()` |
| `.position()` | Returns the index of the first element that satisfies a predicate function, if any. | `iter.position(|x| x % 2 == 0)` |
| `.product()` | Computes the product of all elements in the iterator. | `iter.product()` |
| `.rev()` | Reverses the order of elements in the iterator. | `iter.rev()` |
| `.skip()` | Skips the first n elements of the iterator. | `iter.skip(3)` |
| `.skip_while()` | Skips elements of the iterator based on a predicate function until the predicate is not satisfied. | `iter.skip_while(|x| x % 2 == 0)` |
| `.step_by()` | Creates an iterator that skips elements based on a step value. | `iter.step_by(2)` |
| `.sum()` | Computes the sum of all elements in the iterator. | `iter.sum()` |
| `.take()` | Takes the first n elements of the iterator. | `iter.take(3)` |
| `.take_while()` | Takes elements of the iterator based on a predicate function until the predicate is not satisfied. | `iter.take_while(|x| x % 2 == 0)` |
| `.unzip()` | Transforms an iterator of pairs into two separate collections. | `iter.unzip()` |
| `.zip()` | Zips two iterators together, combining them into a single iterator of pairs. | `iter1.zip(iter2)` |
