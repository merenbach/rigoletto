use crate::affine;
use cipher::Cipher;
use masc::transform;
use masc::Atom;

#[cfg(test)]
mod tests {
    use super::make;
    use cipher::Cipher;
    use masc::Atom;

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

/// Make a monoalphabetic substitution cipher.
pub fn make<T: Atom>(pt_alphabet: &[T], strict: bool) -> impl Cipher<T, T> {
    // don't allow m to be less than zero
    let m = pt_alphabet.len().saturating_sub(1);
    affine::make(pt_alphabet, m, m, strict)
}
