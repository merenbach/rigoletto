use super::{Cipher, SubstitutionCipher};
use crate::simple;
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
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 4, 1, 3, 5, 2, 2, 5, 3, 1, 4, 0],
                strict: false,
            },
            TestCase {
                slope: 7,
                intercept: 3,
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![4, 1, 3, 5, 2, 2, 5, 3, 1, 4],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, x.slope, x.intercept);
            let out = if x.strict {
                c.encipher(&x.input)
            } else {
                c.encipher_retain(&x.input)
            };
            assert_eq!(x.output, out);
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                slope: 7,
                intercept: 3,
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 4, 1, 3, 5, 2, 2, 5, 3, 1, 4, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                strict: false,
            },
            TestCase {
                slope: 7,
                intercept: 3,
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 4, 1, 3, 5, 2, 2, 5, 3, 1, 4, 0],
                output: vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, x.slope, x.intercept);
            let out = if x.strict {
                c.decipher(&x.input)
            } else {
                c.decipher_retain(&x.input)
            };
            assert_eq!(x.output, out);
        }
    }
}

pub fn make<T: Atom>(
    pt_alphabet: &[T],
    slope: usize,
    intercept: usize,
) -> impl SubstitutionCipher<T> {
    simple::make(pt_alphabet, |xs| transform::affine(xs, slope, intercept))
}
