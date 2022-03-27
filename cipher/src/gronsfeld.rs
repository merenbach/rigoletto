use crate::Cipher;
use derive_builder::Builder;
use pasc::SubstitutionCipherBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        key: Vec<char>,

        pt_alphabet: Vec<char>,
        input: Vec<char>,
        output: Vec<char>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                key: "90210".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "QENMO FOTMD hello world".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "90210".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "QENMOFOTMD".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = GronsfeldBuilder::default()
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
                key: "90210".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "QENMO FOTMD qenmo fotmd".chars().collect(),
                output: "HELLO WORLD qenmo fotmd".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "90210".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "QENMO FOTMD qenmo fotmd".chars().collect(),
                output: "HELLOWORLD".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = GronsfeldBuilder::default()
                .key(x.key.to_vec())
                .pt_alphabet(x.pt_alphabet.to_vec())
                .strict(x.strict)
                .build()
                .unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

const KEY_ALPHABET: &str = "0123456789";

#[derive(Default, Builder)]
pub struct Gronsfeld {
    key: Vec<char>,

    pt_alphabet: Vec<char>,
    strict: bool,
}

impl Cipher<char, char> for Gronsfeld {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[char]) -> Vec<char> {
        let c = SubstitutionCipherBuilder::default()
            .with_vigenere()
            .key(self.key.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .key_alphabet(Some(KEY_ALPHABET.chars().collect()))
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[char]) -> Vec<char> {
        let c = SubstitutionCipherBuilder::default()
            .with_vigenere()
            .key(self.key.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .key_alphabet(Some(KEY_ALPHABET.chars().collect()))
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
