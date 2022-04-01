use crate::reciprocal_table;
use crate::{Cipher, SubstitutionCipher};
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
            let c = make(&x.pt_alphabet, &x.key, x.strict);
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

/// Make a substitution cipher.
pub fn make(pt_alphabet: &[char], key: &[char], strict: bool) -> impl SubstitutionCipher<char> {
    reciprocal_table::make(
        pt_alphabet,
        pt_alphabet,
        pt_alphabet,
        key,
        strict,
        |xs, i| transform::vigenere(xs, i),
    )
}
