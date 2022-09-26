use cipher::Cipher;
use masc::tableau::Atom;
use masc::transform;
use masc::SubstitutionCipher;

#[cfg(test)]
mod tests {
    use super::make;
    use cipher::Cipher;
    use masc::tableau::Atom;

    struct TestCase<T: Atom> {
        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 0],
                strict: false,
            },
            TestCase {
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![5, 4, 3, 2, 1, 1, 2, 3, 4, 5],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, x.strict);
            let out = c.encipher(&x.input);
            assert_eq!(x.output, out);
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                strict: false,
            },
            TestCase {
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 0],
                output: vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, x.strict);
            let out = c.decipher(&x.input);
            assert_eq!(x.output, out);
        }
    }
}

/// Make a substitution cipher.
pub fn make<T: Atom>(pt_alphabet: &[T], strict: bool) -> impl Cipher<T, T> {
    let ct_alphabet = transform::atbash(pt_alphabet);
    SubstitutionCipher::new(&pt_alphabet, &ct_alphabet, strict)
}
