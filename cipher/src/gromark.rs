use crate::reciprocal_table;
use crate::Cipher;
use lfg::LFGBuilder;
use masc::tableau::Atom;
use num::{Integer, Unsigned};
use pasc::transform;
use std::iter;
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

fn chain_adder<T>(m: T, count: usize, primer: &[T]) -> Vec<T>
where
    T: Copy + Integer + Unsigned + iter::Product + iter::Sum,
{
    let g = LFGBuilder::default()
        .additive()
        .modulus(m)
        .seed(primer.to_vec())
        .taps(vec![1, 2])
        .build()
        .unwrap();
    primer.iter().copied().chain(g).take(count).collect()
}

pub fn makegromarkkey(primer: &[u32], msglen: usize) -> Vec<char> {
    // let primer: Vec<_> = k.chars().filter_map(|c| c.to_digit(10)).collect();
    chain_adder(10, msglen, &primer)
        .iter()
        .filter_map(|&i| char::from_digit(i, 10))
        .collect()
}

/// Make a substitution cipher.
pub fn make<T: Atom + Ord>(
    pt_alphabet: &[T],
    keyword: &[T],
    primer: &[u32],
    strict: bool,
) -> impl Cipher<T, T> {
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
        |xs, i| transform::vigenere(xs, i),
        strict,
    )
}
