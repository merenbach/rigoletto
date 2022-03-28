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
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "ZINCS PGVNU hello world".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "ZINCSPGVNU".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = VigenereBuilder::default()
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
                input: "ZINCS PGVNU zincs pgvnu".chars().collect(),
                output: "HELLO WORLD zincs pgvnu".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "ZINCS PGVNU zincs pgvnu".chars().collect(),
                output: "HELLOWORLD".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = VigenereBuilder::default()
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
pub struct Vigenere {
    key: Vec<char>,

    pt_alphabet: Vec<char>,
    strict: bool,
}

impl Cipher<char, char> for Vigenere {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[char]) -> Vec<char> {
        let ct_alphabets: Vec<_> = (0..self.pt_alphabet.len())
            .map(|i| transform::vigenere(&self.pt_alphabet, i))
            .collect();
        let c = reciprocal_table::ReciprocalTableBuilder::default()
            .key(self.key.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .ct_alphabets(ct_alphabets)
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[char]) -> Vec<char> {
        let ct_alphabets: Vec<_> = (0..self.pt_alphabet.len())
            .map(|i| transform::vigenere(&self.pt_alphabet, i))
            .collect();
        let c = reciprocal_table::ReciprocalTableBuilder::default()
            .key(self.key.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .ct_alphabets(ct_alphabets)
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
