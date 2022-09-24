use crate::Cipher;
use derive_builder::Builder;
use masc::tableau::{Atom, Tableau};
use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;

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
            let t = Tableau::new(&x.pt_alphabet, &x.ct_alphabet);
            let c = SimpleBuilder::default()
                .tableau(t)
                .strict(x.strict)
                .build()
                .unwrap();
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
            let t = Tableau::new(&x.pt_alphabet, &x.ct_alphabet);
            let c = SimpleBuilder::default()
                .tableau(t)
                .strict(x.strict)
                .build()
                .unwrap();
            let out = c.decipher(&x.input);
            assert_eq!(x.output, out);
        }
    }
}

#[derive(Default, Builder)]
pub struct Simple<T: Atom> {
    #[builder(default)]
    strict: bool,

    #[builder(default)]
    tableau: Tableau<T, T>,
}

impl<T: Atom> Simple<T> {
    /// Encipher an element.
    fn encipher_one(&self, x: &T) -> Option<T> {
        self.tableau.encode(x)
    }

    /// Decipher an element.
    fn decipher_one(&self, x: &T) -> Option<T> {
        self.tableau.decode(x)
    }

    fn transcipher(&self, xs: &[T], cb: impl Fn(&T) -> Option<T>) -> Vec<T> {
        if self.strict {
            xs.iter().filter_map(|x| cb(x)).collect()
        } else {
            xs.iter().map(|x| cb(x).unwrap_or(*x)).collect()
        }
    }
}

impl<T: Atom> Cipher<T, T> for Simple<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        self.transcipher(xs, |x| self.encipher_one(x))
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        self.transcipher(xs, |x| self.decipher_one(x))
    }
}

// TODO: ensure we have tests for this
impl fmt::Display for Simple<char> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pt_alphabet: String = self.tableau.pt_alphabet.iter().collect();
        let ct_alphabet: String = self.tableau.ct_alphabet.iter().collect();
        write!(f, "Simple <PT: {}, CT: {}>", &pt_alphabet, &ct_alphabet)
    }
}

/// Make a substitution cipher.
pub fn make<T, F>(pt_alphabet: &[T], f: F, strict: bool) -> impl Cipher<T, T>
where
    T: Atom,
    F: Fn(&[T]) -> Vec<T>,
{
    let ct_alphabet = f(&pt_alphabet);
    let tableau = Tableau::new(&pt_alphabet, &ct_alphabet);
    SimpleBuilder::default()
        .tableau(tableau)
        .strict(strict)
        .build()
        .unwrap()
}
