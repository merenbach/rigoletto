use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;

/// Transform an array slice based on indices provided and return the output as a vector.
/// This function should be independent of the type of transposition being performed.
pub fn transpose<T: Copy>(xs: &[T], ys: &[usize]) -> Vec<T> {
    std_ext::backpermute(&xs, &std_ext::argsort(&ys))
}

// Find the relative lexical ordering of passed parameters.
pub fn lexical_order<T>(xs: &[T]) -> Vec<usize>
where
    T: Hash + Ord,
{
    // Sort and deduplicate input.
    let set: BTreeSet<_> = xs.iter().collect();

    // Assign cardinal positions to input characters based on sort order.
    let map: HashMap<_, usize> = set.iter().zip(0..).collect();

    // Map each input character to its first-seen position.
    xs.iter().map(|e| map[&e]).collect()
}
