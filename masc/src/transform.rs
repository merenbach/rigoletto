use itertools::Itertools;
use lcg::LCGBuilder;
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_works() {
        let rows = &[
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row, &dummy(&row));
        }
    }

    #[test]
    fn affine_works() {
        let rows = &[
            (vec![], vec![], 3, 7),
            (vec![1], vec![1], 3, 7),
            (vec![1, 2], vec![1, 2], 1, 0),
            (vec![1, 2, 3], vec![3, 2, 1], 2, 2),
        ];
        for row in rows {
            assert_eq!(row.0, affine(&row.1, row.2, row.3));
        }
    }

    #[test]
    fn atbash_works() {
        let rows = &[
            (vec![], vec![]),
            (vec![1], vec![1]),
            (vec![1, 2], vec![2, 1]),
            (vec![1, 2, 3], vec![3, 2, 1]),
            (vec![1, 2, 3, 4], vec![4, 3, 2, 1]),
            (vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]),
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row.0, atbash(&row.1));
        }
    }

    #[test]
    fn caesar_works() {
        let rows = &[
            (vec![], vec![], 0),
            (vec![], vec![], 1),
            (vec![1], vec![1], 0),
            (vec![1], vec![1], 1),
            (vec![1, 2], vec![1, 2], 0),
            (vec![2, 1], vec![1, 2], 1),
            (vec![1, 2], vec![1, 2], 2),
            (vec![2, 1], vec![1, 2], 3),
            (vec![1, 2, 3], vec![1, 2, 3], 0),
            (vec![2, 3, 1], vec![1, 2, 3], 1),
            (vec![3, 1, 2], vec![1, 2, 3], 2),
            (vec![1, 2, 3], vec![1, 2, 3], 3),
            (vec![2, 3, 1], vec![1, 2, 3], 4),
            (vec![3, 1, 2], vec![1, 2, 3], 5),
            (vec![1, 2, 3, 4], vec![1, 2, 3, 4], 0),
            (vec![2, 3, 4, 1], vec![1, 2, 3, 4], 1),
            (vec![3, 4, 1, 2], vec![1, 2, 3, 4], 2),
            (vec![4, 1, 2, 3], vec![1, 2, 3, 4], 3),
            (vec![1, 2, 3, 4], vec![1, 2, 3, 4], 4),
            (vec![2, 3, 4, 1], vec![1, 2, 3, 4], 5),
            (vec![3, 4, 1, 2], vec![1, 2, 3, 4], 6),
            (vec![4, 1, 2, 3], vec![1, 2, 3, 4], 7),
            (vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], 0),
            (vec![2, 3, 4, 5, 1], vec![1, 2, 3, 4, 5], 1),
            (vec![3, 4, 5, 1, 2], vec![1, 2, 3, 4, 5], 2),
            (vec![4, 5, 1, 2, 3], vec![1, 2, 3, 4, 5], 3),
            (vec![5, 1, 2, 3, 4], vec![1, 2, 3, 4, 5], 4),
            (vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], 5),
            (vec![2, 3, 4, 5, 1], vec![1, 2, 3, 4, 5], 6),
            (vec![3, 4, 5, 1, 2], vec![1, 2, 3, 4, 5], 7),
            (vec![4, 5, 1, 2, 3], vec![1, 2, 3, 4, 5], 8),
            (vec![5, 1, 2, 3, 4], vec![1, 2, 3, 4, 5], 9),
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row.0, caesar(&row.1, row.2));
        }
    }

    #[test]
    fn decimation_works() {
        let rows = &[
            (vec![], vec![], 0),
            (vec![], vec![], 1),
            (vec![1], vec![1], 0),
            (vec![1], vec![1], 1),
            (vec![1, 2], vec![1, 2], 1),
            (vec![1, 2], vec![1, 2], 3),
            (vec![1, 2, 3], vec![1, 2, 3], 1),
            (vec![1, 3, 2], vec![1, 2, 3], 2),
            (vec![1, 2, 3], vec![1, 2, 3], 4),
            (vec![1, 3, 2], vec![1, 2, 3], 5),
            (vec![1, 2, 3, 4], vec![1, 2, 3, 4], 1),
            (vec![1, 4, 3, 2], vec![1, 2, 3, 4], 3),
            (vec![1, 2, 3, 4], vec![1, 2, 3, 4], 5),
            (vec![1, 4, 3, 2], vec![1, 2, 3, 4], 7),
            (vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], 1),
            (vec![1, 3, 5, 2, 4], vec![1, 2, 3, 4, 5], 2),
            (vec![1, 4, 2, 5, 3], vec![1, 2, 3, 4, 5], 3),
            (vec![1, 5, 4, 3, 2], vec![1, 2, 3, 4, 5], 4),
            (vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], 6),
            (vec![1, 3, 5, 2, 4], vec![1, 2, 3, 4, 5], 7),
            (vec![1, 4, 2, 5, 3], vec![1, 2, 3, 4, 5], 8),
            (vec![1, 5, 4, 3, 2], vec![1, 2, 3, 4, 5], 9),
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row.0, decimation(&row.1, row.2));
        }
    }

    #[test]
    fn keyword_works() {
        let rows = &[
            (vec![], vec![], vec![]),
            (vec![], vec![], vec![0]),
            (vec![1], vec![1], vec![0, 1]),
            (vec![1], vec![1], vec![1, 0]),
            (vec![1, 2], vec![2, 1], vec![1]),
            (vec![1, 2], vec![2, 1], vec![0, 1]),
            (vec![1, 2], vec![2, 1, 1, 2], vec![0, 1]),
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row.0, keyword(&row.1, &row.2));
        }
    }
}

/// Perform a dummy (no-op) transformation.
pub fn dummy<T: Copy>(xs: &[T]) -> Vec<T> {
    xs.to_vec()
}

/// Perform an affine transform on an array slice and return as a vector.
pub fn affine<T: Copy>(xs: &[T], slope: usize, intercept: usize) -> Vec<T> {
    let m = xs.len();
    // TODO: check for coprimality of m and slope
    match m {
        0..=1 => xs.to_vec(),
        _ => {
            let lcg: Vec<_> = LCGBuilder::default() // TODO: just use successors here?
                .modulus(m)
                .multiplier(1)
                .increment(slope % m)
                .seed(intercept % m)
                .build()
                .unwrap()
                .skip(m - 1)
                .take(m)
                .collect();
            std_ext::backpermute(&xs, &lcg)
        }
    }
}

/// Perform an Atbash transform on an array slice and return as a vector.
pub fn atbash<T: Copy>(xs: &[T]) -> Vec<T> {
    let m = xs.len().saturating_sub(1);
    affine(xs, m, m)
}

/// Perform a Caesar transform on an array slice and return as a vector.
pub fn caesar<T: Copy>(xs: &[T], shift: usize) -> Vec<T> {
    affine(xs, 1, shift)
}

/// Perform a decimation transform on an array slice and return as a vector.
pub fn decimation<T: Copy>(xs: &[T], multiplier: usize) -> Vec<T> {
    affine(xs, multiplier, 0)
}

/// Perform a keyword transform on a vector.
/// A keyword transform for a primary sequence and a keyword sequence:
///
/// 1. Filters from the keyword sequence any elements not in the primary sequence;
/// 2. Prepends the resulting sequence to the primary sequence;
/// 3. Returns the concatenated result with all duplicates removed, leaving the first appearance of each.
///
pub fn keyword<T>(xs: &[T], ys: &[T]) -> Vec<T>
where
    T: Copy + Eq + Hash,
{
    ys.iter()
        .filter(|&c| xs.contains(c))
        .chain(xs)
        .unique()
        .copied()
        .collect()
    // [ys, xs].concat().into_iter().unique().collect()
    // let mut set = std::collections::HashSet::new();
    // [ys, xs].concat().into_iter().filter(|&c| set.insert(c)).collect()
}
