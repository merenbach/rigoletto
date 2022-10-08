use masc::transform as masc_transform;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_works() {
        let rows = &[
            (vec![], vec![], 0),
            (vec![], vec![], 1),
            (vec![1], vec![1], 0),
            (vec![1], vec![1], 1),
            (vec![1, 2], vec![1, 2], 0),
            (vec![1, 2], vec![1, 2], 1),
            (vec![1, 2], vec![1, 2], 2),
            (vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], 0),
            (vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], 1),
            (vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], 2),
            (vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], 3),
            (vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], 4),
            (vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], 5),
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row.0, dummy(&row.1, row.2));
        }
    }

    #[test]
    fn vigenere_works() {
        let rows = &[
            (vec![], vec![], 0),
            (vec![], vec![], 1),
            (vec![1], vec![1], 0),
            (vec![1], vec![1], 1),
            (vec![1, 2], vec![1, 2], 0),
            (vec![2, 1], vec![1, 2], 1),
            (vec![1, 2], vec![1, 2], 2),
            (vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], 0),
            (vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 4], 1),
            (vec![2, 3, 4, 0, 1], vec![0, 1, 2, 3, 4], 2),
            (vec![3, 4, 0, 1, 2], vec![0, 1, 2, 3, 4], 3),
            (vec![4, 0, 1, 2, 3], vec![0, 1, 2, 3, 4], 4),
            (vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], 5),
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row.0, vigenere(&row.1, row.2));
        }
    }

    #[test]
    fn beaufort_works() {
        let rows = &[
            (vec![], vec![], 0),
            (vec![], vec![], 1),
            (vec![1], vec![1], 0),
            (vec![1], vec![1], 1),
            (vec![1, 2], vec![1, 2], 0),
            (vec![2, 1], vec![1, 2], 1),
            (vec![1, 2], vec![1, 2], 2),
            (vec![0, 4, 3, 2, 1], vec![0, 1, 2, 3, 4], 0),
            (vec![1, 0, 4, 3, 2], vec![0, 1, 2, 3, 4], 1),
            (vec![2, 1, 0, 4, 3], vec![0, 1, 2, 3, 4], 2),
            (vec![3, 2, 1, 0, 4], vec![0, 1, 2, 3, 4], 3),
            (vec![4, 3, 2, 1, 0], vec![0, 1, 2, 3, 4], 4),
            (vec![0, 4, 3, 2, 1], vec![0, 1, 2, 3, 4], 5),
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row.0, beaufort(&row.1, row.2));
        }
    }

    #[test]
    fn variant_beaufort_works() {
        let rows = &[
            (vec![], vec![], 0),
            (vec![], vec![], 1),
            (vec![1], vec![1], 0),
            (vec![1], vec![1], 1),
            (vec![1, 2], vec![1, 2], 0),
            (vec![2, 1], vec![1, 2], 1),
            (vec![1, 2], vec![1, 2], 2),
            (vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], 0),
            (vec![4, 0, 1, 2, 3], vec![0, 1, 2, 3, 4], 1),
            (vec![3, 4, 0, 1, 2], vec![0, 1, 2, 3, 4], 2),
            (vec![2, 3, 4, 0, 1], vec![0, 1, 2, 3, 4], 3),
            (vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 4], 4),
            (vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], 5),
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row.0, variant_beaufort(&row.1, row.2));
        }
    }

    #[test]
    fn owrap_works() {
        let rows = &[
            (vec![], vec![], 0),
            (vec![], vec![], 1),
            (vec![1], vec![1], 0),
            (vec![1], vec![1], 1),
            (vec![1, 2], vec![1, 2], 0),
            (vec![1, 2], vec![1, 2], 1),
            (vec![1, 2], vec![1, 2], 2),
            (
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                0,
            ),
            (
                vec![1, 2, 3, 0, 7, 4, 5, 6],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                1,
            ),
            (
                vec![2, 3, 0, 1, 6, 7, 4, 5],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                2,
            ),
            (vec![0, 1, 2, 3, 4, 5, 6], vec![0, 1, 2, 3, 4, 5, 6], 0),
            (vec![1, 2, 0, 3, 6, 4, 5], vec![0, 1, 2, 3, 4, 5, 6], 1),
            (vec![2, 0, 1, 3, 5, 6, 4], vec![0, 1, 2, 3, 4, 5, 6], 2),
        ];
        for row in rows {
            println!("{:?}", row);
            assert_eq!(row.0, owrap(&row.1, row.2));
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

/// Wrap a slice outwards the specified number of indices.
/// Wrap will leave the center item unchanged if the element count is odd.
fn owrap<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
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
                .push((listlen + idx - i + listlen % 2) % midpoint + midpoint + listlen % 2);
        }
    }

    std_ext::backpermute(&xs, &new_positions)
}

// // Perform a pivoted Caesar transform on an array slice and return as a vector.
// pub fn pivoted_caesar<T: Copy>(xs: &[T], shift: usize, pivot: usize) -> Vec<T> {
//     affine(xs, 1, shift)
// }

// Perform a dummy (no-op) transform on an array slice and return as a vector.
pub fn dummy<T: Copy>(xs: &[T], _: usize) -> Vec<T> {
    if xs.is_empty() {
        xs.into()
    } else {
        masc_transform::dummy(&xs)
    }
}

/// Vigenere transform on an array slice, returned as a vector.
/// This is equivalent to a standard Caesar cipher, shifted by the row index.
/// Encryption: CT_grid = PT_axis + K_axis
/// Decryption: PT_axis = CT_grid - K_axis
pub fn vigenere<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    if xs.is_empty() {
        xs.into()
    } else {
        masc_transform::caesar(&xs, i)
    }
}

/// Beaufort transform on an array slice, returned as a vector.
/// This is equivalent to a Vigenere with base ciphertext alphabet and key alphabet reversed.
/// Encryption: CT_axis = K_grid - PT_axis
/// Decryption: PT_axis = K_grid - CT_axis
pub fn beaufort<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    if xs.is_empty() {
        xs.into()
    } else {
        masc_transform::affine(&xs, xs.len() - 1, i)
    }
}

/// Atbash transform on an array slice, returned as a vector.
/// This is equivalent to a Vigenere with plaintext alphabet, base ciphertext alphabet, and key alphabet reversed.
// pub fn weird_beaufort<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
//     if xs.is_empty() {
//         xs.into()
//     } else {
//         masc_transform::affine(&xs, xs.len() - 1, xs.len() - i)
//     }
// }

/// Variant Beaufort transform on an array slice, returned as a vector.
/// This is equivalent to a Vigenere with plaintext alphabet and base ciphertext alphabet reversed.
/// The Variant Beaufort is simply the Vigenere with encipherment and decipherment reversed.
/// Encryption: CT_axis = PT_grid - K_axis
/// Decryption: PT_grid = CT_axis + K_axis
pub fn variant_beaufort<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    if xs.is_empty() {
        xs.into()
    } else {
        masc_transform::caesar(&xs, xs.len() - i)
    }
}

// Perform a Della Porta transform on an array slice and return as a vector.
pub fn della_porta<T: Copy>(xs: &[T], i: usize) -> Vec<T> {
    // TODO: should we enforce an even-length requirement here?
    // TODO: this is an inefficient repeated call to to_vec() under the hood

    if xs.is_empty() {
        xs.into()
    } else {
        let midpoint: usize = xs.len() / 2;
        let ys = wrap(xs, midpoint);
        owrap(&ys, i)
    }
}
