use crate::{Cipher, SubstitutionCipher};
use derive_builder::Builder;
use masc::tableau::Atom;
use pasc::SubstitutionCipherBuilder;
use std::cell::RefCell;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     struct TestCase {
//         key: Vec<char>,

//         pt_alphabet: Vec<char>,
//         ct_alphabets: Vec<Vec<char>>,
//         key_alphabet: Vec<char>,
//         input: Vec<char>,
//         output: Vec<char>,
//         strict: bool,
//     }

//     #[test]
//     fn encipher_works() {
//         let xs = &[
//             TestCase {
//                 key: "SECRET".chars().collect(),
//                 pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
//                 input: "HELLO WORLD hello world".chars().collect(),
//                 output: "ZINCS PGVNU hello world".chars().collect(),
//                 strict: false,
//             },
//             TestCase {
//                 key: "SECRET".chars().collect(),
//                 pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
//                 input: "HELLO WORLD hello world".chars().collect(),
//                 output: "ZINCSPGVNU".chars().collect(),
//                 strict: true,
//             },
//         ];
//         for x in xs {
//             let c = ReciprocalTableBuilder::default()
//                 .key(x.key.to_vec())
//                 .pt_alphabet(x.pt_alphabet.to_vec())
//                 .strict(x.strict)
//                 .build()
//                 .unwrap();
//             assert_eq!(x.output, c.encipher(&x.input));
//         }
//     }

//     #[test]
//     fn decipher_works() {
//         let xs = &[
//             TestCase {
//                 key: "SECRET".chars().collect(),
//                 pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
//                 input: "ZINCS PGVNU zincs pgvnu".chars().collect(),
//                 output: "HELLO WORLD zincs pgvnu".chars().collect(),
//                 strict: false,
//             },
//             TestCase {
//                 key: "SECRET".chars().collect(),
//                 pt_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
//                 input: "ZINCS PGVNU zincs pgvnu".chars().collect(),
//                 output: "HELLOWORLD".chars().collect(),
//                 strict: true,
//             },
//         ];
//         for x in xs {
//             let c = ReciprocalTableBuilder::default()
//                 .key(x.key.to_vec())
//                 .pt_alphabet(x.pt_alphabet.to_vec())
//                 .strict(x.strict)
//                 .build()
//                 .unwrap();
//             assert_eq!(x.output, c.decipher(&x.input));
//         }
//     }
// }

#[derive(Default, Builder)]
pub struct ReciprocalTable<T: Atom> {
    #[builder(setter(into))]
    key: Vec<T>,

    #[builder(setter(into))]
    pt_alphabet: Vec<T>,
    #[builder(setter(into))]
    ct_alphabets: Vec<Vec<T>>,
    #[builder(setter(into))]
    key_alphabet: Vec<T>,
    strict: bool,

    #[builder(setter(skip))]
    tableau: RefCell<pasc::SubstitutionCipher<T>>,
}

impl<T: Atom> ReciprocalTable<T> {
    fn initialize(&self) {
        if !self.tableau.borrow().is_ready() {
            *self.tableau.borrow_mut() = SubstitutionCipherBuilder::default()
                .key(self.key.to_vec())
                .pt_alphabet(self.pt_alphabet.to_vec())
                .ct_alphabets(self.ct_alphabets.to_vec())
                .key_alphabet(self.key_alphabet.to_vec())
                .strict(self.strict)
                .build()
                .unwrap();
        }
    }
}

impl<T: Atom> Cipher<T, T> for ReciprocalTable<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        self.initialize();
        self.tableau.borrow().encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        self.initialize();
        self.tableau.borrow().decipher(xs)
    }
}

impl<T: Atom> SubstitutionCipher<T> for ReciprocalTable<T> {
    /// Encipher a sequence.
    fn encipher_retain(&self, xs: &[T]) -> Vec<T> {
        self.initialize();
        self.tableau.borrow().encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher_retain(&self, xs: &[T]) -> Vec<T> {
        self.initialize();
        self.tableau.borrow().decipher(xs)
    }
}

// impl<T: Atom> SubstitutionCipher<T> for Simple<T> {
//     /// Encipher a sequence.
//     fn encipher_retain(&self, xs: &[T]) -> Vec<T> {
//         self.initialize();
//         xs.iter()
//             .map(|x| self.encipher_one(x).unwrap_or(*x))
//             .collect()
//     }

//     /// Decipher a sequence.
//     fn decipher_retain(&self, xs: &[T]) -> Vec<T> {
//         self.initialize();
//         xs.iter()
//             .map(|x| self.decipher_one(x).unwrap_or(*x))
//             .collect()
//     }
// }

// // TODO: ensure we have tests for this
// impl fmt::Display for ReciprocalTable<char> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let pt_alphabet: String = self.pt_alphabet.iter().collect();
//         let ct_alphabet: String = self.ct_alphabet.iter().collect();
//         write!(f, "Simple <PT: {}, CT: {}>", &pt_alphabet, &ct_alphabet)
//     }
// }

/// Make a substitution cipher.
pub fn make<T, F>(
    pt_alphabet: &[T],
    ct_alphabet: &[T],
    key_alphabet: &[T],
    key: &[T], // TODO: should this be on encipher or decipher instead? kind of torn
    strict: bool,
    f: F,
) -> impl SubstitutionCipher<T>
where
    T: Atom,
    F: Fn(&[T], usize) -> Vec<T>,
{
    let ct_alphabets: Vec<_> = key_alphabet
        .iter()
        .enumerate()
        .map(|(i, _)| f(ct_alphabet, i))
        .collect();

    ReciprocalTableBuilder::default()
        .key(key)
        .pt_alphabet(pt_alphabet)
        .ct_alphabets(ct_alphabets)
        .key_alphabet(key_alphabet)
        .strict(strict)
        .build()
        .unwrap()
}
