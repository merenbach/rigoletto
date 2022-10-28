use itertools::Itertools;
use num::integer::gcd;
use num::Integer;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[cfg(test)]
mod tests {
    #[test]
    fn backpermute_works() {
        struct TestCase<T: Copy>(Vec<T>, Vec<T>, Vec<usize>);

        let tables = &[
            TestCase(vec![], vec![], vec![]),
            TestCase(vec![], vec!['A'], vec![]),
            TestCase(vec!['A'], vec!['A'], vec![0]),
            TestCase(vec!['A', 'A'], vec!['A'], vec![0, 0]),
            TestCase(vec!['E', 'H'], vec!['H', 'E', 'L', 'L', 'O'], vec![1, 0]),
            TestCase(
                vec!['D', 'O', 'W', 'E', 'L', ' ', 'R', 'O', 'D'],
                vec!['H', 'E', 'L', 'L', 'O', ' ', 'W', 'O', 'R', 'L', 'D'],
                vec![10, 7, 6, 1, 3, 5, 8, 7, 10],
            ),
        ];
        for t in tables {
            let out: Vec<_> = super::backpermute(&t.1, &t.2);
            assert_eq!(&t.0, &out);
        }
    }

    #[test]
    fn argsort_works() {
        struct TestCase<T: Ord>(Vec<usize>, Vec<T>);

        let tables = &[
            TestCase(vec![], vec![]),
            TestCase(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4]),
            TestCase(vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5]),
            TestCase(vec![0, 1, 2, 3, 4], vec![10, 20, 30, 40, 50]),
            TestCase(vec![0, 1, 2, 4, 3], vec![10, 20, 30, 50, 40]),
            TestCase(vec![4, 3, 2, 1, 0], vec![50, 40, 30, 20, 10]),
            TestCase(vec![2, 0, 4, 1, 3], vec![11, 13, 10, 14, 12]),
            TestCase(vec![1, 3, 0, 4, 2], vec![2, 0, 4, 1, 3]),
            TestCase(vec![2, 0, 4, 1, 3], vec![1, 3, 0, 4, 2]),
            TestCase(
                vec![0, 8, 1, 7, 9, 15, 2, 6, 10, 14, 3, 5, 11, 13, 4, 12],
                vec![0, 1, 2, 3, 4, 3, 2, 1, 0, 1, 2, 3, 4, 3, 2, 1],
            ),
            TestCase(
                vec![0, 2, 6, 10, 14, 11, 7, 3, 1, 4, 8, 12, 15, 13, 9, 5],
                vec![0, 8, 1, 7, 9, 15, 2, 6, 10, 14, 3, 5, 11, 13, 4, 12],
            ),
            TestCase(
                vec![0, 8, 1, 7, 9, 15, 2, 6, 10, 14, 3, 5, 11, 13, 4, 12],
                vec![0, 2, 6, 10, 14, 11, 7, 3, 1, 4, 8, 12, 15, 13, 9, 5],
            ),
        ];
        for t in tables {
            let out: Vec<_> = super::argsort(&t.1);
            assert_eq!(&t.0, &out);
        }
    }

    #[test]
    fn coprime_works() {
        let tables = &[
            (true, 3, 5),
            (true, 7, 20),
            (true, 14, 15),
            (true, 172, 17),
            (false, 2, 4),
            (false, 2, 22),
            (false, 3, 15),
            (false, 14, 28),
        ];

        for t in tables {
            assert_eq!(t.0, super::coprime(t.1, t.2));
        }
    }
}

/// Backpermute a slice based on indices provided in a second slice.
/// Backpermute returns output as a vector.
/// Backpermute will panic if any indices are out of bounds.
pub fn backpermute<T: Copy>(xs: &[T], indices: &[usize]) -> Vec<T> {
    indices.iter().map(|&i| xs[i]).collect()
}

/// Backpermute a slice based on indices provided in a second slice.
/// Backpermute returns output as a vector.
/// Backpermute_safe will ignore out-of-bounds indices.
// pub fn backpermute_safe<T: Copy>(xs: &[T], indices: &[usize]) -> Vec<T> {
//     indices.iter().filter_map(|&i| xs.get(i)).copied().collect()
// }

/// Argsort returns the indices that would sort an array.
/// The naming is based on the numpy extension's name for this concept.
///
/// <https://numpy.org/doc/stable/reference/generated/numpy.argsort.html>
///
/// This is effectively a Schwartzian transform or decorate-sort-undecorate.
///
///   1. Attach numbers to each item in the collection.
///   2. Rearrange the collection such that it is now sorted lexically. This will scramble the numbers.
///   3. Return only the numbers now.
pub fn argsort<T: Ord+Copy>(xs: &[T]) -> Vec<usize> {
    (0..xs.len()).sorted_by_cached_key(|&v| &xs[v]).collect()
    // (0..xs.len()).sorted_by(|&a,&b| Ord::cmp(&xs[a], &xs[b])).collect()
    // xs.iter()
    //     .enumerate()
    //     .sorted_by_key(|v| v.1)
    //     .map(|t| t.0)
    //     .collect()
}

// pub fn invert_pairs<T: Ord>(xs: &[T]) -> Vec<(usize, usize)> {
//     let x = invert_internal(xs);
//     let y = invert_internal(&x);
//     x.iter().zip(y.iter()).map(|(&a, &b)| (a, b)).collect()
// }

// /// Deduplicate elements by order of first occurrence.
// pub fn deduplicate<T>(xs: &[T]) -> Vec<T>
// where
//     T: std::hash::Hash + Eq + Copy,
// {
//     let mut set = std::collections::HashSet::new();
//     xs.iter().filter(|&c| set.insert(c)).map(|&e| e).collect()
// }

// /// Deduplicate elements by order of first occurrence.
// pub fn deduplicate2<T>(xs: &mut Vec<T>)
// where
//     T: std::hash::Hash + Eq + Copy,
// {
//     let mut set = std::collections::HashSet::new();
//     xs.retain(|&c| set.insert(c));
// }

// pub fn zigzag<T>(period: T, count: usize) -> Vec<T>
// where
//     T: Integer + Unsigned + Copy,
// {
//     (..period)
//         .map(|i| i.checked_rem(period))
//         .map(|n| std::cmp::min(n, period - n))
//         .cycle()
//         .take(count)
//         .collect()
// }

/// Coprime determines whether two integers share no prime factors.
/// Coprime numbers are also called relatively prime.
/// Coprime does not care about the order of the parameters.
pub fn coprime<T: Integer>(x: T, y: T) -> bool {
    gcd(x, y).is_one()
}

/// Read and deserialize data from a file.
pub fn read_data_from_file<P, T>(path: P) -> Result<T, Box<dyn Error>>
where
    P: AsRef<Path>,
    T: for<'de> Deserialize<'de>,
{
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `T`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

/*
/// Read and deserialize data from a file.
fn read_data_from_file<T>(dir: &str, file: &str) -> Result<T, Box<Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let path = Path::new(dir).join(file);

    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `T`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}
*/
