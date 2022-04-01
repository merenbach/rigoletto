use crate::reciprocal_table;
use crate::{Cipher, SubstitutionCipher};
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
            let c = make(&x.pt_alphabet, &x.keyword, &x.primer, x.strict);
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
            let c = make(&x.pt_alphabet, &x.keyword, &x.primer, x.strict);
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

const KEY_ALPHABET: &str = "0123456789";

/// Make a substitution cipher.
pub fn make(
    pt_alphabet: &[char],
    keyword: &[char],
    primer: &[u32],
    strict: bool,
) -> impl SubstitutionCipher<char> {
    let key_alphabet: Vec<_> = KEY_ALPHABET.chars().collect();
    let ys = masc::transform::keyword(&pt_alphabet, &keyword);
    let ct_alphabet_base = ColumnarTranspositionCipherBuilder::with_generic_key(&keyword)
        .build()
        .unwrap()
        .encipher(&ys);
    let xs_len = 1000; // TODO: this is a kludge till we move to iterator for this
    let key = makegromarkkey(&primer, xs_len);

    reciprocal_table::make(
        pt_alphabet,
        &ct_alphabet_base,
        &key_alphabet,
        &key,
        strict,
        |xs, i| transform::vigenere(xs, i),
    )
}
