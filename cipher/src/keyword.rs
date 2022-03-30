use crate::simple;
use crate::{Cipher, SubstitutionCipher};
use derive_builder::Builder;
use masc::tableau::Atom;
use masc::transform;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        keyword: Vec<T>,

        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                keyword: vec![5, 3, 3],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0],
                strict: false,
            },
            TestCase {
                keyword: vec![5, 3, 3],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![5, 3, 1, 2, 4, 4, 2, 1, 3, 5],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, &x.keyword);
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
                keyword: vec![5, 3, 3],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                strict: false,
            },
            TestCase {
                keyword: vec![5, 3, 3],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0],
                output: vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, &x.keyword);
            let out = if x.strict {
                c.decipher(&x.input)
            } else {
                c.decipher_retain(&x.input)
            };
            assert_eq!(x.output, out);
        }
    }
}

pub fn make<T: Atom>(pt_alphabet: &[T], keyword: &[T]) -> impl SubstitutionCipher<T> {
    simple::make(pt_alphabet, |xs| transform::keyword(xs, keyword))
}
