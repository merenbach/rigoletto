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
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "PAJUK DWNJM hello world".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "HELLO WORLD hello world".chars().collect(),
                output: "PAJUKDWNJM".chars().collect(),
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
                input: "PAJUK DWNJM pajuk dwnjm".chars().collect(),
                output: "HELLO WORLD pajuk dwnjm".chars().collect(),
                strict: false,
            },
            TestCase {
                key: "SECRET".chars().collect(),
                pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
                input: "PAJUK DWNJM pajuk dwnjm".chars().collect(),
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
pub fn make<T: Atom>(pt_alphabet: &[T], key: &[T], strict: bool) -> impl Cipher<T, T> {
    let ct_alphabets: Vec<_> = pt_alphabet
        .iter()
        .enumerate()
        .map(|(i, _)| transform::variant_beaufort(pt_alphabet, i))
        .collect();

    SubstitutionCipherBuilder::default()
        .key(key)
        .pt_alphabet(pt_alphabet)
        .ct_alphabets(ct_alphabets)
        .key_alphabet(pt_alphabet)
        .strict(strict)
        .build()
        .unwrap()
}
