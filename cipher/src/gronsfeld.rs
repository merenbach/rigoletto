use crate::reciprocal_table;
use crate::Cipher;
use masc::tableau::Atom;
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
            let c = make(&x.pt_alphabet, &x.key, x.strict);
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
            let c = make(&x.pt_alphabet, &x.key, x.strict);
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

const KEY_ALPHABET: &str = "0123456789";

/// Make a substitution cipher.
pub fn make<T: Atom>(pt_alphabet: &[T], key: &[char], strict: bool) -> impl Cipher<T, T> {
    let key_alphabet: Vec<_> = KEY_ALPHABET.chars().collect();
    reciprocal_table::make(
        pt_alphabet,
        pt_alphabet,
        &key_alphabet,
        key,
        |xs, i| transform::vigenere(xs, i),
        strict,
    )
}
