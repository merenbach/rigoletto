use crate::simple;
use crate::Cipher;
use derive_builder::Builder;
use masc::tableau::Atom;
use masc::transform;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        offset: usize,

        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
    }

    #[test]
    fn encipher_works() {
        let xs = &[TestCase {
            offset: 3,
            pt_alphabet: vec![1, 2, 3, 4, 5],
            input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            output: vec![0, 4, 5, 1, 2, 3, 3, 2, 1, 5, 4, 0],
        }];
        for x in xs {
            let c = CaesarBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .offset(x.offset)
                .build()
                .unwrap();
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[TestCase {
            offset: 3,
            pt_alphabet: vec![1, 2, 3, 4, 5],
            input: vec![0, 4, 5, 1, 2, 3, 3, 2, 1, 5, 4, 0],
            output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
        }];
        for x in xs {
            let c = CaesarBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .offset(x.offset)
                .build()
                .unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

#[derive(Default, Builder)]
pub struct Caesar<T: Atom> {
    offset: usize,

    pt_alphabet: Vec<T>,
}

impl<T: Atom> Cipher<T, T> for Caesar<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let c = simple::make(&self.pt_alphabet, |xs| transform::caesar(xs, self.offset));
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let c = simple::make(&self.pt_alphabet, |xs| transform::caesar(xs, self.offset));
        c.decipher(xs)
    }
}
