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
    fn owrap_works() {
        let rows = &[
            (vec![], vec![], 0),
            (vec![], vec![], 1),
            (vec![1], vec![1], 0),
            (vec![1], vec![1], 1),
            (vec![1], vec![1], 2),
            (vec![1, 2], vec![1, 2], 3),
            (vec![1, 2, 3], vec![1, 2, 3], 0),
            (vec![1, 2, 3], vec![1, 2, 3], 1),
            (vec![1, 2, 3], vec![1, 2, 3], 2),
            (vec![1, 2, 3, 4], vec![1, 2, 3, 4], 0),
            (vec![2, 1, 4, 3], vec![1, 2, 3, 4], 1),
            (vec![1, 2, 3, 4], vec![1, 2, 3, 4], 2),
            (vec![2, 1, 4, 3], vec![1, 2, 3, 4], 3),
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row.0, owrap(&row.1, row.2));
        }
    }
}

/// Wrap a slice the specified number of indices.
fn wrap<T: Copy>(s: &[T], i: usize) -> Vec<T> {
    let mut rr: Vec<_> = s.to_vec();
    let len = rr.len();
    rr.rotate_left(i % len);
    rr
}

/// Wrap a string outward a specified number of indices.
/// TODO: simplify and move into std_ext
fn owrap<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    let m = xs.len();
    match m {
        0..=2 => xs.to_vec(),
        _ => {
            let half_len = m / 2;
            let rem = m % 2;
            let (first_half, middle, second_half) = (
                &xs[..half_len],
                &xs[half_len..half_len + rem], // will exist only when xs.len() is odd (in which case length will be 1)
                &xs[half_len + rem..], // account for unrotated middle element with odd xs.len()
            );

            let new_first_half = masc_transform::caesar(first_half, i % m);
            let new_second_half = masc_transform::caesar(second_half, m - i % m);

            let mut out: Vec<_> = Vec::new();
            out.extend(&new_first_half);
            out.extend(middle);
            out.extend(&new_second_half);
            out
        }
    }
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
    masc_transform::affine(&xs, 1, i)
}

// Perform a Beaufort transform on an array slice and return as a vector.
pub fn beaufort<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    masc_transform::affine(&xs, xs.len() - 1, i)
}

// Perform a variant Beaufort transform on an array slice and return as a vector.
pub fn variant_beaufort<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    masc_transform::affine(&xs, 1, xs.len() - i)
}

// Perform a Della Porta transform on an array slice and return as a vector.
pub fn della_porta<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    // TODO: should we enforce an even-length requirement here?
    // TODO: this is an inefficient repeated call to to_vec() under the hood
    let ys = wrap(&xs.to_vec(), xs.len() / 2);
    owrap(&ys, i / 2)
}
