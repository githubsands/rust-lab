use std::marker;

/// PhantomData is necessary when a structure needs a lifetime but all it's fields do not
///
/// Imbedding a struct with PhantomData ensures correct variance and drop checking. Given type for
/// the purpose of static analysis
struct Iter<'a, T: 'a> {
    ptr: *const T,
    end: *const T,
    _marker: marker::PhantomData<&'a T>,
}

/// Lifetime will be bounded, and your iterator will be covariant over 'a and T
/// Using unique instead of emberdding structures with Phantomdata
pub struct Unique<T: ?Sized> {
    pointer: NonNull<T>,
    _marker: PhantomData<T>,
}
