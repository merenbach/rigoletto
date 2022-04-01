use crate::simple;
use crate::{Cipher, SubstitutionCipher};
use masc::tableau::Atom;
use masc::transform;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        multiplier: usize,

        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                multiplier: 3,
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 1, 4, 2, 5, 3, 3, 5, 2, 4, 1, 0],
                strict: false,
            },
            TestCase {
                multiplier: 3,
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![1, 4, 2, 5, 3, 3, 5, 2, 4, 1],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, x.multiplier);
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
                multiplier: 3,
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 4, 2, 5, 3, 3, 5, 2, 4, 1, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                strict: false,
            },
            TestCase {
                multiplier: 3,
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 4, 2, 5, 3, 3, 5, 2, 4, 1, 0],
                output: vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, x.multiplier);
            let out = if x.strict {
                c.decipher(&x.input)
            } else {
                c.decipher_retain(&x.input)
            };
            assert_eq!(x.output, out);
        }
    }
}

/// Make a substitution cipher.
pub fn make<T: Atom>(pt_alphabet: &[T], multiplier: usize) -> impl SubstitutionCipher<T> {
    simple::make(pt_alphabet, move |xs| transform::decimation(xs, multiplier))
}
