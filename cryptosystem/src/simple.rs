use cipher::Cipher;
use masc::{Atom, SubstitutionCipherBuilder};

/*pub struct Config<T: Atom> {
    alphabet: Vec<T>,
    caseless:bool,
    strict: bool,
}

// type ConfigOption<T:Atom> = fn(&mut Config<T>);

type ConfigOption<T:Atom> = impl FnOnce(&mut Config<T>);

fn with_strict<T:Atom>() -> ConfigOption<T> {
    |c| c.strict = true
}

fn with_caseless<T:Atom>() -> ConfigOption<T> {
    |c| c.caseless = true
}

fn with_alphabet<T:Atom>(alphabet: Vec<T>) -> ConfigOption<T> {
    |c| c.alphabet = alphabet
}*/

#[cfg(test)]
mod tests {
    use super::make;
    use cipher::Cipher;
    use masc::Atom;

    struct TestCase<T: Atom> {
        ct_alphabet: Vec<T>,

        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                ct_alphabet: vec![4, 5, 6, 7, 8],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 0],
                strict: false,
            },
            TestCase {
                ct_alphabet: vec![4, 5, 6, 7, 8],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![4, 5, 6, 7, 8, 8, 7, 6, 5, 4],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, &x.ct_alphabet, x.strict);
            let out = c.encipher(&x.input);
            assert_eq!(x.output, out);
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                ct_alphabet: vec![4, 5, 6, 7, 8],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                strict: false,
            },
            TestCase {
                ct_alphabet: vec![4, 5, 6, 7, 8],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 0],
                output: vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
                strict: true,
            },
        ];
        for x in xs {
            let c = make(&x.pt_alphabet, &x.ct_alphabet, x.strict);
            let out = c.decipher(&x.input);
            assert_eq!(x.output, out);
        }
    }
}

/// Make a monoalphabetic substitution cipher.
pub fn make<T: Atom>(pt_alphabet: &[T], ct_alphabet: &[T], strict: bool) -> impl Cipher<T, T> {
    SubstitutionCipherBuilder::default()
        .pt_alphabet(pt_alphabet)
        .ct_alphabet(ct_alphabet)
        .strict(strict)
        .build()
        .unwrap()
}
