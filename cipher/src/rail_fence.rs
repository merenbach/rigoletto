use crate::Cipher;
use derive_builder::Builder;
use transposition::Atom;
use transposition::ColumnarTranspositionCipherBuilder;

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
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
            },
            TestCase {
                rails: 2,
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [0, 2, 4, 5, 3, 1, 1, 3, 5, 4, 2, 0].to_vec(),
            },
            TestCase {
                rails: 3,
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [0, 4, 3, 1, 3, 5, 4, 2, 0, 2, 5, 1].to_vec(),
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
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
            },
            TestCase {
                rails: 2,
                input: [0, 2, 4, 5, 3, 1, 1, 3, 5, 4, 2, 0].to_vec(),
                output: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
            },
            TestCase {
                rails: 3,
                input: [0, 4, 3, 1, 3, 5, 4, 2, 0, 2, 5, 1].to_vec(),
                output: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
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
        let c = ColumnarTranspositionCipherBuilder::with_rail_fence(self.rails)
            .myszkowski(true)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let c = ColumnarTranspositionCipherBuilder::with_rail_fence(self.rails)
            .myszkowski(true)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
