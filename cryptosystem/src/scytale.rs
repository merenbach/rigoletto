use cipher::Cipher;
use derive_builder::Builder;
use transposition::{Atom, ColumnarTranspositionCipherBuilder};

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        turns: usize,

        input: Vec<T>,
        output: Vec<T>,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                turns: 1,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                turns: 2,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 2, 4, 5, 3, 1, 1, 3, 5, 4, 2, 0],
            },
            TestCase {
                turns: 3,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 3, 5, 2, 1, 4, 4, 1, 2, 5, 3, 0],
            },
        ];
        for x in xs {
            let c = make(x.turns);
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                turns: 1,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                turns: 2,
                input: vec![0, 2, 4, 5, 3, 1, 1, 3, 5, 4, 2, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                turns: 3,
                input: vec![0, 3, 5, 2, 1, 4, 4, 1, 2, 5, 3, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
        ];
        for x in xs {
            let c = make(x.turns);
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

/// Make a substitution cipher.
pub fn make<T: Atom>(turns: usize) -> impl Cipher<T, T> {
    let ys = match turns {
        1 => vec![0],
        _ => (0..turns).collect(),
    };

    // Prepare a scytale cipher.
    // N.b.: The scytale cipher is a special case of a columnar transposition cipher
    //       with a key equal to an ascending consecutive integer sequence
    //       as long as the number of turns.
    //       A sequence with all the same digit may also work, but may depend on a stable sort.
    ColumnarTranspositionCipherBuilder::with_generic_key(&ys)
        .myszkowski(true)
        .build()
        .unwrap()
}
