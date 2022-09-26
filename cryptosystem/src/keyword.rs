use cipher::Cipher;
use masc::Atom;
use masc::transform;
use masc::SubstitutionCipher;

#[cfg(test)]
mod tests {
    use super::make;
    use cipher::Cipher;
    use masc::Atom;

    struct TestCase<T: Atom> {
        keyword: Vec<T>,

        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                keyword: vec![5, 3, 3],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0],
                strict: false,
            },
            TestCase {
                keyword: vec![5, 3, 3],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![5, 3, 1, 2, 4, 4, 2, 1, 3, 5],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, &x.keyword, x.strict);
            let out = c.encipher(&x.input);
            assert_eq!(x.output, out);
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                keyword: vec![5, 3, 3],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                strict: false,
            },
            TestCase {
                keyword: vec![5, 3, 3],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0],
                output: vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, &x.keyword, x.strict);
            let out = c.decipher(&x.input);
            assert_eq!(x.output, out);
        }
    }
}

/// Make a substitution cipher.
pub fn make<T: Atom>(pt_alphabet: &[T], keyword: &[T], strict: bool) -> impl Cipher<T, T> {
    let kw = keyword.to_owned(); // lifetime specifier concerns
    let ct_alphabet = transform::keyword(pt_alphabet, &kw);
    SubstitutionCipher::new(&pt_alphabet, &ct_alphabet, strict)
}
