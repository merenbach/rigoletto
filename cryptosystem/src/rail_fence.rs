use cipher::Cipher;
use derive_builder::Builder;
use transposition::transform;
use transposition::{Atom, ColumnarTranspositionCipherBuilder};

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        rails: usize,

        input: Vec<T>,
        output: Vec<T>,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                rails: 1,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                rails: 2,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 2, 4, 5, 3, 1, 1, 3, 5, 4, 2, 0],
            },
            TestCase {
                rails: 3,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 4, 3, 1, 3, 5, 4, 2, 0, 2, 5, 1],
            },
        ];
        for x in xs {
            let c = RailFenceBuilder::default().rails(x.rails).build().unwrap();
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                rails: 1,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                rails: 2,
                input: vec![0, 2, 4, 5, 3, 1, 1, 3, 5, 4, 2, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                rails: 3,
                input: vec![0, 4, 3, 1, 3, 5, 4, 2, 0, 2, 5, 1],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
        ];
        for x in xs {
            let c = RailFenceBuilder::default().rails(x.rails).build().unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

#[derive(Default, Builder)]
pub struct RailFence {
    rails: usize,
}

impl<T: Atom> Cipher<T, T> for RailFence {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let ys = match self.rails {
            1 => vec![0],
            _ => {
                let period = 2 * (self.rails - 1);
                transform::zigzag(period)
            }
        };

        // Prepare a rail fence cipher.
        // N.b.: The rail fence cipher is a special case of a columnar transposition cipher
        //       with Myszkowski transposition and a key equal to a zigzag sequence
        //       that converts the row count into the appropriate period.
        let c = ColumnarTranspositionCipherBuilder::with_generic_key(&ys)
            .myszkowski(true)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let ys = match self.rails {
            1 => vec![0],
            _ => {
                let period = 2 * (self.rails - 1);
                transform::zigzag(period)
            }
        };

        // Prepare a rail fence cipher.
        // N.b.: The rail fence cipher is a special case of a columnar transposition cipher
        //       with Myszkowski transposition and a key equal to a zigzag sequence
        //       that converts the row count into the appropriate period.
        let c = ColumnarTranspositionCipherBuilder::with_generic_key(&ys)
            .myszkowski(true)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
