use crate::reciprocal_table;
use crate::Cipher;
use derive_builder::Builder;
use pasc::transform;

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
        let key_alphabet: Vec<_> = KEY_ALPHABET.chars().collect();
        let ct_alphabets: Vec<_> = (0..key_alphabet.len())
            .map(|i| transform::vigenere(&self.pt_alphabet, i))
            .collect();
        let c = reciprocal_table::ReciprocalTableBuilder::default()
            .key(self.key.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .ct_alphabets(ct_alphabets)
            .key_alphabet(key_alphabet)
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[char]) -> Vec<char> {
        let key_alphabet: Vec<_> = KEY_ALPHABET.chars().collect();
        let ct_alphabets: Vec<_> = (0..key_alphabet.len())
            .map(|i| transform::vigenere(&self.pt_alphabet, i))
            .collect();
        let c = reciprocal_table::ReciprocalTableBuilder::default()
            .key(self.key.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .ct_alphabets(ct_alphabets)
            .key_alphabet(key_alphabet)
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
