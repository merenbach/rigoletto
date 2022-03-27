use super::Cipher;
use crate::simple;
use derive_builder::Builder;
use masc::tableau::Atom;
use masc::transform;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        slope: usize,
        intercept: usize,

        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                slope: 7,
                intercept: 3,
                pt_alphabet: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [0, 4, 1, 3, 5, 2, 2, 5, 3, 1, 4, 0].to_vec(),
                strict: false,
            },
            TestCase {
                slope: 7,
                intercept: 3,
                pt_alphabet: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [4, 1, 3, 5, 2, 2, 5, 3, 1, 4].to_vec(),
                strict: true,
            },
        ];
        for x in xs {
            let c = AffineBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .slope(x.slope)
                .intercept(x.intercept)
                .strict(x.strict)
                .build()
                .unwrap();
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                slope: 7,
                intercept: 3,
                pt_alphabet: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 4, 1, 3, 5, 2, 2, 5, 3, 1, 4, 0].to_vec(),
                output: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                strict: false,
            },
            TestCase {
                slope: 7,
                intercept: 3,
                pt_alphabet: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 4, 1, 3, 5, 2, 2, 5, 3, 1, 4, 0].to_vec(),
                output: [1, 2, 3, 4, 5, 5, 4, 3, 2, 1].to_vec(),
                strict: true,
            },
        ];
        for x in xs {
            let c = AffineBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .slope(x.slope)
                .intercept(x.intercept)
                .strict(x.strict)
                .build()
                .unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

#[derive(Default, Builder)]
pub struct Affine<T: Atom> {
    slope: usize,
    intercept: usize,

    pt_alphabet: Vec<T>,
    strict: bool,
}

impl<T: Atom> Cipher<T, T> for Affine<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let ct_alphabet = transform::affine(&self.pt_alphabet, self.slope, self.intercept);
        let c = simple::SimpleBuilder::default()
            .pt_alphabet(self.pt_alphabet.to_vec())
            .ct_alphabet(ct_alphabet)
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let ct_alphabet = transform::affine(&self.pt_alphabet, self.slope, self.intercept);
        let c = simple::SimpleBuilder::default()
            .pt_alphabet(self.pt_alphabet.to_vec())
            .ct_alphabet(ct_alphabet)
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
