use crate::Cipher;
use derive_builder::Builder;
use masc::tableau::Atom;
use pasc::SubstitutionCipherBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        key: Vec<T>,

        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                key: vec![9, 3, 4, 1, 1, 0, 9],
                pt_alphabet: vec![0, 1, 2, 3, 4],
                input: vec![9, 2, 1, 0, 2, 2, 2, 2, 1, 1, 1, 2, 4, 4, 9],
                output: vec![9, 1, 3, 1, 4, 3, 1, 2, 0, 0, 4, 1, 0, 2, 9],
                strict: false,
            },
            TestCase {
                key: vec![9, 3, 4, 1, 1, 0, 9],
                pt_alphabet: vec![0, 1, 2, 3, 4],
                input: vec![9, 2, 1, 0, 2, 2, 2, 2, 1, 1, 1, 2, 4, 4, 9],
                output: vec![1, 3, 1, 4, 3, 1, 2, 0, 0, 4, 1, 0, 2],
                strict: true,
            },
        ];
        for x in xs {
            let c = BeaufortBuilder::default()
                .key(x.key.to_vec())
                .pt_alphabet(x.pt_alphabet.to_vec())
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
                key: vec![9, 3, 4, 1, 1, 0, 9],
                pt_alphabet: vec![0, 1, 2, 3, 4],
                input: vec![9, 1, 3, 1, 4, 3, 1, 2, 0, 0, 4, 1, 0, 2, 9],
                output: vec![9, 2, 1, 0, 2, 2, 2, 2, 1, 1, 1, 2, 4, 4, 9],
                strict: false,
            },
            TestCase {
                key: vec![9, 3, 4, 1, 1, 0, 9],
                pt_alphabet: vec![0, 1, 2, 3, 4],
                input: vec![9, 1, 3, 1, 4, 3, 1, 2, 0, 0, 4, 1, 0, 2, 9],
                output: vec![2, 1, 0, 2, 2, 2, 2, 1, 1, 1, 2, 4, 4],
                strict: true,
            },
        ];
        for x in xs {
            let c = BeaufortBuilder::default()
                .key(x.key.to_vec())
                .pt_alphabet(x.pt_alphabet.to_vec())
                .strict(x.strict)
                .build()
                .unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

#[derive(Default, Builder)]
pub struct Beaufort<T: Atom> {
    key: Vec<T>,

    pt_alphabet: Vec<T>,
    strict: bool,
}

impl<T: Atom> Cipher<T, T> for Beaufort<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let c = SubstitutionCipherBuilder::default()
            .with_beaufort()
            .key(self.key.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let c = SubstitutionCipherBuilder::default()
            .with_beaufort()
            .key(self.key.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
