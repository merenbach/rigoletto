use crate::{Cipher, SubstitutionCipher};
use derive_builder::Builder;
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
pub struct ReciprocalTable {
    #[builder(setter(into))]
    key: Vec<char>,

    #[builder(setter(into))]
    pt_alphabet: Vec<char>,
    #[builder(setter(into))]
    ct_alphabets: Vec<Vec<char>>,
    #[builder(setter(into))]
    key_alphabet: Vec<char>,
    strict: bool,

    #[builder(setter(skip))]
    tableau: RefCell<pasc::SubstitutionCipher<char>>,
}

impl ReciprocalTable {
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

impl Cipher<char, char> for ReciprocalTable {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[char]) -> Vec<char> {
        self.initialize();
        self.tableau.borrow().encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[char]) -> Vec<char> {
        self.initialize();
        self.tableau.borrow().decipher(xs)
    }
}

impl SubstitutionCipher<char> for ReciprocalTable {
    /// Encipher a sequence.
    fn encipher_retain(&self, xs: &[char]) -> Vec<char> {
        self.initialize();
        self.tableau.borrow().encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher_retain(&self, xs: &[char]) -> Vec<char> {
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
pub fn make<F>(
    pt_alphabet: &[char],
    ct_alphabet: &[char],
    key_alphabet: &[char],
    key: &[char], // TODO: should this be on encipher or decipher instead? kind of torn
    strict: bool,
    f: F,
) -> impl SubstitutionCipher<char>
where
    F: Fn(&[char], usize) -> Vec<char>,
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
