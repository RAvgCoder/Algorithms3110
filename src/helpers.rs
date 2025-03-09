/// # Safety
/// This function assumes that the `Vec` has sufficient capacity to accommodate at least one more element.
/// If this assumption is violated, the program may exhibit undefined behavior.
///
/// ## When to Use
/// This function is useful in performance-critical code where capacity checks are redundant and can be safely omitted.
/// It should only be used when you can **guarantee** that the vector has enough capacity before inserting elements.
///
/// # Example
/// ```rust
/// use std::vec::Vec;
///
/// #[inline(always)]
/// pub unsafe fn assume_sufficient_capacity<T>(list: &Vec<T>) {
///     core::hint::assert_unchecked(list.capacity() > list.len())
/// }
///
/// fn main() {
///     let mut numbers = Vec::with_capacity(5);
///     unsafe { assume_sufficient_capacity(&numbers) }; // Safe because we just allocated with capacity 5.
///     numbers.push(42); // No reallocation happens.
/// }
/// ```
///
/// # More Complex Example
/// ```rust
/// struct Interval {
///     start: i32,
///     end: i32,
/// }
///
/// impl Interval {
///     fn new(start: i32, end: i32) -> Self {
///         Self { start, end }
///     }
/// }
///
/// fn process_intervals(intervals: &[Interval]) -> Vec<Interval> {
///     let mut processed = Vec::with_capacity(intervals.len());
///     let mut iter = 0..intervals.len();
///     
///     while let Some(idx) = iter.next() {
///         // SAFETY: We allocated with `intervals.len()` capacity, so we ensure we never exceed it.
///         unsafe { assume_sufficient_capacity(&processed); }
///         
///         let current = &intervals[idx];
///         processed.push(Interval::new(current.start, current.end));
///         
///         let Some(next) = intervals.get(idx + 1) else {
///             break;
///         };
///
///         let Some(next_next) = intervals.get(idx + 2) else {
///             processed.push(Interval::new(next.start, next.end));
///             break;
///         };
///         
///         if current.end + 1 == next_next.start {
///             _ = iter.next(); // Skip `next` interval
///         }
///     }
///     
///     processed
/// }
/// ```
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn assume_sufficient_capacity<T>(list: &Vec<T>) {
    core::hint::assert_unchecked(list.capacity() > list.len())
}
