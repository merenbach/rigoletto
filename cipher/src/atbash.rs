use crate::simple;
use crate::Cipher;
use derive_builder::Builder;
use masc::tableau::Atom;
use masc::transform;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
    }

    #[test]
    fn encipher_works() {
        let xs = &[TestCase {
            pt_alphabet: vec![1, 2, 3, 4, 5],
            input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            output: vec![0, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 0],
        }];
        for x in xs {
            let c = AtbashBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .build()
                .unwrap();
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[TestCase {
            pt_alphabet: vec![1, 2, 3, 4, 5],
            input: vec![0, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 0],
            output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
        }];
        for x in xs {
            let c = AtbashBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .build()
                .unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

#[derive(Default, Builder)]
pub struct Atbash<T: Atom> {
    pt_alphabet: Vec<T>,
}

impl<T: Atom> Cipher<T, T> for Atbash<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let c = simple::make(&self.pt_alphabet, |xs| transform::atbash(xs));
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let c = simple::make(&self.pt_alphabet, |xs| transform::atbash(xs));
        c.decipher(xs)
    }
}
