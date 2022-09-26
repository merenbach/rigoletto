use crate::Cipher;
use derive_builder::Builder;
use masc::tableau::{Atom, Tableau};
use std::cell::RefCell;
use std::fmt;
use translation::{Table, TableBuilder};

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
            let c = SimpleBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_owned())
                .ct_alphabet(x.ct_alphabet.to_owned())
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
            let c = SimpleBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_owned())
                .ct_alphabet(x.ct_alphabet.to_owned())
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
    pt_alphabet: Vec<T>,
    ct_alphabet: Vec<T>,

    #[builder(setter(skip))]
    pt2ct: RefCell<Table<T>>,
    #[builder(setter(skip))]
    ct2pt: RefCell<Table<T>>,

    #[builder(default)]
    strict: bool,
}

impl<T: Atom> Simple<T> {
    fn initialize(&self) {
        if self.pt2ct.borrow().is_empty() {
            *self.pt2ct.borrow_mut() = TableBuilder::default()
                .src(self.pt_alphabet.to_owned())
                .dst(self.ct_alphabet.to_owned())
                .build()
                .unwrap();
        }

        if self.ct2pt.borrow().is_empty() {
            *self.ct2pt.borrow_mut() = TableBuilder::default()
                .src(self.ct_alphabet.to_owned())
                .dst(self.pt_alphabet.to_owned())
                .build()
                .unwrap();
        }
    }

    /// Encipher an element.
    fn encipher_one(&self, x: &T) -> Option<T> {
        self.initialize();
        self.pt2ct.borrow().translate_one(x, |_| None)
    }

    /// Decipher an element.
    fn decipher_one(&self, x: &T) -> Option<T> {
        self.initialize();
        self.ct2pt.borrow().translate_one(x, |_| None)
    }
}

impl<T: Atom> Cipher<T, T> for Simple<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        self.initialize();
        self.pt2ct
            .borrow()
            .translate(xs, |x| if self.strict { None } else { Some(x) })
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        self.initialize();
        self.ct2pt
            .borrow()
            .translate(xs, |x| if self.strict { None } else { Some(x) })
    }
}

// TODO: ensure we have tests for this
impl fmt::Display for Simple<char> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pt_alphabet: String = self.pt_alphabet.iter().collect();
        let ct_alphabet: String = self.ct_alphabet.iter().collect();
        write!(f, "Simple <PT: {}, CT: {}>", &pt_alphabet, &ct_alphabet)
    }
}

/// Make a substitution cipher.
pub fn make2<T>(pt_alphabet: &[T], ct_alphabet: &[T], strict: bool) -> impl Cipher<T, T>
where
    T: Atom,
{
    SimpleBuilder::default()
        .pt_alphabet(pt_alphabet.to_owned())
        .ct_alphabet(ct_alphabet.to_owned())
        .strict(strict)
        .build()
        .unwrap()
}

/// Make a substitution cipher.
pub fn make<T, F>(pt_alphabet: &[T], f: F, strict: bool) -> impl Cipher<T, T>
where
    T: Atom,
    F: Fn(&[T]) -> Vec<T>,
{
    let ct_alphabet = f(&pt_alphabet);
    make2(pt_alphabet, &ct_alphabet, strict)
}
