use cipher::Cipher;
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
struct ReciprocalTable<T: Atom, K: Atom> {
    #[builder(setter(into))]
    key: Vec<K>,

    #[builder(setter(into))]
    pt_alphabet: Vec<T>,
    #[builder(setter(into))]
    ct_alphabets: Vec<Vec<T>>,
    #[builder(setter(into))]
    key_alphabet: Vec<K>,
    #[builder(default)]
    strict: bool,

    #[builder(setter(skip))]
    tableau: RefCell<pasc::SubstitutionCipher<T, K>>,
}

impl<T: Atom, K: Atom> ReciprocalTable<T, K> {
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

impl<T: Atom, K: Atom> Cipher<T, T> for ReciprocalTable<T, K> {
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

#[derive(Default, Builder)]
struct ReciprocalTableHomogeneous<T: Atom> {
    #[builder(setter(into))]
    key: Vec<T>,

    #[builder(setter(into))]
    pt_alphabet: Vec<T>,
    #[builder(setter(into))]
    ct_alphabets: Vec<Vec<T>>,
    #[builder(setter(into))]
    key_alphabet: Vec<T>,
    #[builder(default)]
    strict: bool,

    // TODO: replace with enums
    #[builder(default)]
    text_autokey: bool,
    #[builder(default)]
    key_autokey: bool,

    #[builder(setter(skip))]
    tableau: RefCell<pasc::SubstitutionCipher<T, T>>,
}

impl<T: Atom> ReciprocalTableHomogeneous<T> {
    fn initialize(&self) {
        if !self.tableau.borrow().is_ready() {
            let autoclave = if self.text_autokey {
                pasc::AutoclaveKind::Text
            } else if self.key_autokey {
                pasc::AutoclaveKind::Key
            } else {
                pasc::AutoclaveKind::None
            };
            *self.tableau.borrow_mut() = SubstitutionCipherBuilder::default()
                .key(self.key.to_vec())
                .pt_alphabet(self.pt_alphabet.to_vec())
                .ct_alphabets(self.ct_alphabets.to_vec())
                .key_alphabet(self.key_alphabet.to_vec())
                .autoclave(autoclave)
                .strict(self.strict)
                .build()
                .unwrap();
        }
    }
}

impl<T: Atom> Cipher<T, T> for ReciprocalTableHomogeneous<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        self.initialize();
        self.tableau.borrow().encipher_autokey(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        self.initialize();
        self.tableau.borrow().decipher_autokey(xs)
    }
}

// // TODO: ensure we have tests for this
// impl fmt::Display for ReciprocalTable<char> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let pt_alphabet: String = self.pt_alphabet.iter().collect();
//         let ct_alphabet: String = self.ct_alphabet.iter().collect();
//         write!(f, "Simple <PT: {}, CT: {}>", &pt_alphabet, &ct_alphabet)
//     }
// }

/// Make a substitution cipher.
pub fn make<T, K, F>(
    pt_alphabet: &[T],
    ct_alphabet: &[T],
    key_alphabet: &[K],
    key: &[K],
    f: F,
    strict: bool,
) -> impl Cipher<T, T>
where
    T: Atom,
    K: Atom,
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

/// Make a substitution cipher.
pub fn make_homogeneous<T, F>(
    pt_alphabet: &[T],
    ct_alphabet: &[T],
    key_alphabet: &[T],
    key: &[T],
    f: F,
    strict: bool,
    text_autokey: bool,
    key_autokey: bool,
) -> impl Cipher<T, T>
where
    T: Atom,
    F: Fn(&[T], usize) -> Vec<T>,
{
    let ct_alphabets: Vec<_> = key_alphabet
        .iter()
        .enumerate()
        .map(|(i, _)| f(ct_alphabet, i))
        .collect();

    ReciprocalTableHomogeneousBuilder::default()
        .key(key)
        .pt_alphabet(pt_alphabet)
        .ct_alphabets(ct_alphabets)
        .key_alphabet(key_alphabet)
        .text_autokey(text_autokey)
        .key_autokey(key_autokey)
        .strict(strict)
        .build()
        .unwrap()
}
