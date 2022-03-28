use crate::reciprocal_table;
use crate::Cipher;
use derive_builder::Builder;
use pasc::makegromarkkey;
use pasc::transform;
use transposition::ColumnarTranspositionCipherBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        keyword: Vec<char>,
        primer: Vec<u32>,

        pt_alphabet: Vec<char>,
        input: Vec<char>,
        output: Vec<char>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                keyword: "ENIGMA".chars().collect(),
                primer: vec![2, 3, 4, 5, 2],
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "THERE ARE UP TO TEN SUBSTITUTES PER LETTER"
                    .chars()
                    .collect(),
                output: "NFYCK BTI JC NW ZYC ACJNAYNLQPW WST WPJQFL"
                    .chars()
                    .collect(),
                strict: false,
            },
            TestCase {
                keyword: "ENIGMA".chars().collect(),
                primer: vec![2, 3, 4, 5, 2],
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "THERE ARE UP TO TEN SUBSTITUTES PER LETTER"
                    .chars()
                    .collect(),
                output: "NFYCKBTIJCNWZYCACJNAYNLQPWWSTWPJQFL".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = GromarkBuilder::default()
                .keyword(x.keyword.to_vec())
                .primer(x.primer.to_vec())
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
                keyword: "ENIGMA".chars().collect(),
                primer: vec![2, 3, 4, 5, 2],
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "NFYCK BTI JC NW ZYC ACJNAYNLQPW WST WPJQFL"
                    .chars()
                    .collect(),
                output: "THERE ARE UP TO TEN SUBSTITUTES PER LETTER"
                    .chars()
                    .collect(),
                strict: false,
            },
            TestCase {
                keyword: "ENIGMA".chars().collect(),
                primer: vec![2, 3, 4, 5, 2],
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "NFYCK BTI JC NW ZYC ACJNAYNLQPW WST WPJQFL"
                    .chars()
                    .collect(),
                output: "THEREAREUPTOTENSUBSTITUTESPERLETTER".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = GromarkBuilder::default()
                .keyword(x.keyword.to_vec())
                .primer(x.primer.to_vec())
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
pub struct Gromark {
    keyword: Vec<char>,
    primer: Vec<u32>,

    pt_alphabet: Vec<char>,
    strict: bool,
}

impl Cipher<char, char> for Gromark {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[char]) -> Vec<char> {
        let key_alphabet: Vec<_> = KEY_ALPHABET.chars().collect();
        let ys = masc::transform::keyword(&self.pt_alphabet, &self.keyword);
        let ct_alphabet_base = ColumnarTranspositionCipherBuilder::with_generic_key(&self.keyword)
            .build()
            .unwrap()
            .encipher(&ys);
        let ct_alphabets: Vec<_> = (0..self.pt_alphabet.len())
            .map(|i| transform::vigenere(&ct_alphabet_base, i))
            .collect();
        let key = makegromarkkey(&self.primer, xs.len());
        let c = reciprocal_table::ReciprocalTableBuilder::default()
            .key(key)
            .pt_alphabet(self.pt_alphabet.to_vec())
            .ct_alphabets(ct_alphabets)
            .key_alphabet(Some(key_alphabet))
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[char]) -> Vec<char> {
        let key_alphabet: Vec<_> = KEY_ALPHABET.chars().collect();
        let ys = masc::transform::keyword(&self.pt_alphabet, &self.keyword);
        let ct_alphabet_base = ColumnarTranspositionCipherBuilder::with_generic_key(&self.keyword)
            .build()
            .unwrap()
            .encipher(&ys);
        let ct_alphabets: Vec<_> = (0..self.pt_alphabet.len())
            .map(|i| transform::vigenere(&ct_alphabet_base, i))
            .collect();
        let key = makegromarkkey(&self.primer, xs.len());
        let c = reciprocal_table::ReciprocalTableBuilder::default()
            .key(key)
            .pt_alphabet(self.pt_alphabet.to_vec())
            .ct_alphabets(ct_alphabets)
            .key_alphabet(Some(key_alphabet))
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
