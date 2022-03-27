use crate::Cipher;
use derive_builder::Builder;
use transposition::Atom;
use transposition::ColumnarTranspositionCipherBuilder;

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
            let c = ScytaleBuilder::default().turns(x.turns).build().unwrap();
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
            let c = ScytaleBuilder::default().turns(x.turns).build().unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

#[derive(Default, Builder)]
pub struct Scytale {
    turns: usize,
}

impl<T: Atom> Cipher<T, T> for Scytale {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let c = ColumnarTranspositionCipherBuilder::with_scytale(self.turns)
            .myszkowski(true)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let c = ColumnarTranspositionCipherBuilder::with_scytale(self.turns)
            .myszkowski(true)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
