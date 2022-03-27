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
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "QTZTM AFCZY hello world".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "QTZTMAFCZY".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = DellaPortaBuilder::default()
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
                input: "QTZTM AFCZY qtztm afczy".chars().collect(),
                output: "HELLO WORLD qtztm afczy".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "QTZTM AFCZY qtztm afczy".chars().collect(),
                output: "HELLOWORLD".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = DellaPortaBuilder::default()
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
pub struct DellaPorta {
    key: Vec<char>,

    pt_alphabet: Vec<char>,
    strict: bool,
}

impl Cipher<char, char> for DellaPorta {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[char]) -> Vec<char> {
        let c = SubstitutionCipherBuilder::default()
            .with_della_porta()
            .key(self.key.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[char]) -> Vec<char> {
        let c = SubstitutionCipherBuilder::default()
            .with_della_porta()
            .key(self.key.to_vec())
            .pt_alphabet(self.pt_alphabet.to_vec())
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
