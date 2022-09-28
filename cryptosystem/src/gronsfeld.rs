use cipher::Cipher;
use masc::Atom;
use pasc::transform;
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
// TODO: allow integers for the key?
pub fn make<T: Atom>(pt_alphabet: &[T], key: &[char], strict: bool) -> impl Cipher<T, T> {
    let key_alphabet: Vec<_> = KEY_ALPHABET.chars().collect();

    let ct_alphabets: Vec<_> = pt_alphabet
        .iter()
        .enumerate()
        .map(|(i, _)| transform::vigenere(pt_alphabet, i))
        .collect();

    SubstitutionCipherBuilder::default()
        .key(key)
        .pt_alphabet(pt_alphabet)
        .ct_alphabets(ct_alphabets)
        .key_alphabet(key_alphabet)
        .strict(strict)
        .build()
        .unwrap()
}
