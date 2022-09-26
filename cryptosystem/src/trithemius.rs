use crate::vigenere;
use cipher::Cipher;
use masc::tableau::Atom;

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
            let c = make(&x.pt_alphabet, x.strict);
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
            let c = make(&x.pt_alphabet, x.strict);
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

/// Make a substitution cipher.
pub fn make<T: Atom>(pt_alphabet: &[T], strict: bool) -> impl Cipher<T, T> {
    vigenere::make(pt_alphabet, pt_alphabet, strict)
}
