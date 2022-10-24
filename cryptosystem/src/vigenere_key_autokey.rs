use cipher::Cipher;
use masc::Atom;
use pasc::transform;
use pasc::{AutoclaveKind, AutoclaveSubstitutionCipherBuilder};

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
                output: "ZINCS PNZYF hello world".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "ZINCSPNZYF".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, &x.key, x.strict);
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "ZINCS PNZYF zincs pnzyf".chars().collect(),
                output: "HELLO WORLD zincs pnzyf".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "ZINCS PNZYF zincs pnzyf".chars().collect(),
                output: "HELLOWORLD".chars().collect(),
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, &x.key, x.strict);
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

/// Make a polyalphabetic substitution cipher.
pub fn make<T: Atom>(pt_alphabet: &[T], key: &[T], strict: bool) -> impl Cipher<T, T> {
    let ct_alphabets: Vec<_> = pt_alphabet
        .iter()
        .enumerate()
        .map(|(i, _)| transform::vigenere(pt_alphabet, i))
        .collect();

    AutoclaveSubstitutionCipherBuilder::default()
        .key(key)
        .pt_alphabet(pt_alphabet)
        .ct_alphabets(ct_alphabets)
        .key_alphabet(pt_alphabet)
        .strict(strict)
        .autoclave(AutoclaveKind::Key)
        .build()
        .unwrap()
}
