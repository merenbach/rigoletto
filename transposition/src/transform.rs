use std::cmp;
use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;

#[cfg(test)]
mod tests {
    use super::{lexorder, zigzag};

    #[test]
    fn zigzag_works() {
        let xs = &[
            (0, vec![]),
            (2, vec![0, 1]),
            (4, vec![0, 1, 2, 1]),
            (6, vec![0, 1, 2, 3, 2, 1]),
            (8, vec![0, 1, 2, 3, 4, 3, 2, 1]),
            (10, vec![0, 1, 2, 3, 4, 5, 4, 3, 2, 1]),
            (12, vec![0, 1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]),
            (14, vec![0, 1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1]),
            (16, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1]),
            (
                18,
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1],
            ),
            (
                20,
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
            ),
        ];
        for x in xs {
            assert_eq!(&x.1, &zigzag(x.0));
        }
    }

    #[test]
    fn lexorder_works() {
        let xs = &[
            ("TOMATO".chars().collect::<Vec<_>>(), vec![3, 2, 1, 0, 3, 2]),
            ("ZEBRAS".chars().collect::<Vec<_>>(), vec![5, 2, 1, 3, 0, 4]),
        ];
        for x in xs {
            assert_eq!(&x.1, &lexorder(&x.0));
        }
    }
}

/// Zigzag sequence, of primary use in the rail fence cipher.
/// The period is the length of the sequence before any repetition would occur.
/// A single period will be returned.
pub fn zigzag(period: usize) -> Vec<usize> {
    (0..period).map(|n| cmp::min(n, period - n)).collect()
}

/// Transform an array slice based on indices provided and return the output as a vector.
/// This function should be independent of the type of transposition being performed.
pub fn transpose<T: Copy>(xs: &[T], ys: &[usize]) -> Vec<T> {
    std_ext::backpermute(&xs, &std_ext::argsort(&ys))
}

// Lexorder returns the relative lexical ordering of a sequence.
// This is spiritually similar to a Schwartzian transform or decorate-sort-undecorate.
pub fn lexorder<T>(xs: &[T]) -> Vec<usize>
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
