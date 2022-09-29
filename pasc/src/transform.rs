use masc::transform as masc_transform;
use num::Integer;

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: add more tests!!!

    // #[test]
    // fn dummy_works() {
    //     let rows = &[
    //         vec![],
    //         vec![1],
    //         vec![1, 2],
    //         vec![1, 2, 3],
    //         vec![1, 2, 3, 4],
    //         vec![1, 2, 3, 4, 5],
    //     ];
    //     for row in rows {
    //         println!("{:?}", row);
    //         assert_eq!(row, &dummy(&row));
    //     }
    // }

    #[test]
    fn della_porta_works() {
        let rows = &[
            (vec![], vec![], 0),
            (vec![], vec![], 1),
            (vec![1], vec![1], 0),
            (vec![1], vec![1], 1),
            (vec![2, 1], vec![1, 2], 0),
            (vec![2, 1], vec![1, 2], 1),
            (vec![2, 1], vec![1, 2], 2),
            (
                vec![4, 5, 6, 7, 0, 1, 2, 3],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                0,
            ),
            (
                vec![5, 6, 7, 4, 3, 0, 1, 2],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                1,
            ),
            (
                vec![6, 7, 4, 5, 2, 3, 0, 1],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                2,
            ),
            // TODO: this is still broken for odd-length inputs
            (vec![4, 5, 6, 55, 1, 2, 3], vec![1, 2, 3, 4, 5, 6, 7], 0),
            // (vec![4, 5, 6, 55, 1, 2, 3], vec![55, 1, 2, 3, 4, 5, 6], 1),
            // (vec![5, 6, 4, 55, 3, 1, 2], vec![3, 1, 2, 55, 4, 5, 6], 2),
            // (vec![5, 6, 4, 55, 3, 1, 2], vec![3, 1, 2, 55, 4, 5, 6], 3),
            // (vec![6, 4, 5, 55, 2, 3, 1], vec![2, 3, 1, 55, 4, 5, 6], 4),
            // (vec![6, 4, 5, 55, 2, 3, 1], vec![2, 3, 1, 55, 4, 5, 6], 5),
            // (vec![1, 2, 3], vec![1, 2, 3], 0),
            // (vec![1, 2, 3], vec![1, 2, 3], 1),
            // (vec![1, 2, 3], vec![1, 2, 3], 2),
            // (vec![1, 2, 3, 4], vec![1, 2, 3, 4], 0),
            // (vec![2, 1, 4, 3], vec![1, 2, 3, 4], 1),
            // (vec![1, 2, 3, 4], vec![1, 2, 3, 4], 2),
            // (vec![2, 1, 4, 3], vec![1, 2, 3, 4], 3),
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row.0, della_porta(&row.1, row.2));
        }
    }
}

/// Wrap a slice the specified number of indices.
fn wrap<T: Copy>(s: &[T], i: usize) -> Vec<T> {
    let mut rr: Vec<_> = s.into();
    let len = rr.len();
    rr.rotate_left(i % len);
    rr
}

// // Perform a pivoted Caesar transform on an array slice and return as a vector.
// pub fn pivoted_caesar<T: Copy>(xs: &[T], shift: usize, pivot: usize) -> Vec<T> {
//     affine(xs, 1, shift)
// }

// Perform a dummy (no-op) transform on an array slice and return as a vector.
pub fn dummy<T: Copy>(xs: &[T], _: usize) -> Vec<T> {
    masc_transform::dummy(&xs)
}

// Perform a Vigenere transform on an array slice and return as a vector.
pub fn vigenere<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    masc_transform::caesar(&xs, i)
}

// Perform a Beaufort transform on an array slice and return as a vector.
pub fn beaufort<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    masc_transform::affine(&xs, xs.len() - 1, i)
}

// // Perform an Atbash transform on an array slice and return as a vector.
// pub fn weird_beaufort<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
//     masc_transform::affine(&xs, xs.len() - 1, xs.len() - i)
// }

// Perform a variant Beaufort transform on an array slice and return as a vector.
pub fn variant_beaufort<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    masc_transform::caesar(&xs, xs.len() - i)
}

// Perform a Della Porta transform on an array slice and return as a vector.
pub fn della_porta<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    // TODO: should we enforce an even-length requirement here?
    // TODO: this is an inefficient repeated call to to_vec() under the hood

    if xs.is_empty() {
        return xs.into();
    }

    let listlen = xs.len();
    let midpoint: usize = listlen / 2;

    let mut new_positions: Vec<usize> = vec![];
    for (idx, _) in xs.iter().enumerate() {
        if idx < midpoint {
            new_positions.push((idx + i) % midpoint);
        } else if idx == midpoint && listlen % 2 != 0 {
            new_positions.push(midpoint);
        } else if idx >= midpoint + listlen % 2 {
            new_positions
                .push((listlen + idx - i - listlen % 2) % midpoint + midpoint + listlen % 2);
        }
    }

    let ys = wrap(xs, midpoint);
    std_ext::backpermute(&ys, &new_positions)
}
