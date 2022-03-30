use crate::simple;
use crate::{Cipher, SubstitutionCipher};
use masc::tableau::Atom;
use masc::transform;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 0],
                strict: false,
            },
            TestCase {
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![5, 4, 3, 2, 1, 1, 2, 3, 4, 5],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet);
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
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                strict: false,
            },
            TestCase {
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 0],
                output: vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet);
            let out = if x.strict {
                c.decipher(&x.input)
            } else {
                c.decipher_retain(&x.input)
            };
            assert_eq!(x.output, out);
        }
    }
}

pub fn make<T: Atom>(pt_alphabet: &[T]) -> impl SubstitutionCipher<T> {
    simple::make(pt_alphabet, |xs| transform::atbash(xs))
}
