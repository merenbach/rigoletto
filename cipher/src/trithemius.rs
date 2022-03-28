use crate::vigenere;
use crate::Cipher;
use derive_builder::Builder;
use pasc::SubstitutionCipherBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        pt_alphabet: Vec<char>,
        input: Vec<char>,
        output: Vec<char>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "HFNOS BUYTM hello world".chars().collect(),
                strict: false,
            },
            TestCase {
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "HFNOSBUYTM".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = VigenereBuilder::default()
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
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HFNOS BUYTM hfnos buytm".chars().collect(),
                output: "HELLO WORLD hfnos buytm".chars().collect(),
                strict: false,
            },
            TestCase {
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HFNOS BUYTM hfnos buytm".chars().collect(),
                output: "HELLOWORLD".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = VigenereBuilder::default()
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
    pt_alphabet: Vec<char>,
    strict: bool,
}

impl Cipher<char, char> for Vigenere {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[char]) -> Vec<char> {
        let c = vigenere::VigenereBuilder::default()
            .key(self.pt_alphabet.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[char]) -> Vec<char> {
        let c = vigenere::VigenereBuilder::default()
            .key(self.pt_alphabet.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
