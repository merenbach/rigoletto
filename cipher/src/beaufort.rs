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
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "LARGQ XENRO hello world".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "LARGQXENRO".chars().collect(),
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
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "LARGQ XENRO largq xenro".chars().collect(),
                output: "HELLO WORLD largq xenro".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "LARGQ XENRO largq xenro".chars().collect(),
                output: "HELLOWORLD".chars().collect(),
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
