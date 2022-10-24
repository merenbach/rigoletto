use cipher::Cipher;
use masc::transform;
use masc::{Atom, SubstitutionCipherBuilder};

#[cfg(test)]
mod tests {
    use super::make;
    use cipher::Cipher;
    use masc::Atom;

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
            let c = make(&x.pt_alphabet, x.multiplier, x.strict);
            let out = c.encipher(&x.input);
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
            let c = make(&x.pt_alphabet, x.multiplier, x.strict);
            let out = c.decipher(&x.input);
            assert_eq!(x.output, out);
        }
    }
}

/// Make a monoalphabetic substitution cipher.
pub fn make<T: Atom>(pt_alphabet: &[T], multiplier: usize, strict: bool) -> impl Cipher<T, T> {
    let ct_alphabet = transform::decimation(pt_alphabet, multiplier);
    SubstitutionCipherBuilder::default()
        .pt_alphabet(pt_alphabet)
        .ct_alphabet(ct_alphabet)
        .strict(strict)
        .build()
        .unwrap()
}
