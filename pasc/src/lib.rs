mod tableau;
pub mod transform;

use alphabet::Alphabet;
use derive_builder::Builder;
use lfg::LFGBuilder;
use masc;
use masc::tableau::Atom;
use num::{Integer, Unsigned};
use std::cell::RefCell;
use std::collections::VecDeque;
use tableau::ReciprocalTable;
use transposition::ColumnarTranspositionCipherBuilder;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

struct KeyQueue<T: Atom>(VecDeque<T>);

impl<T: Atom> From<Vec<T>> for KeyQueue<T> {
    fn from(xs: Vec<T>) -> Self {
        Self(VecDeque::from(xs))
    }
}

impl<T: Atom> KeyQueue<T> {
    fn get(&self) -> &T {
        self.0.front().unwrap()
    }

    fn pop(&mut self) -> T {
        self.0.pop_front().unwrap()
    }

    fn push(&mut self, x: T) {
        self.0.push_back(x)
    }
}

// // Create ciphertext alphabets
// pub fn transform_alphabets<T: Atom>(
//     xs: &[T],
//     count: usize,
//     f: impl Fn(&[T], usize) -> Vec<T>,
// ) -> Vec<Vec<T>> {
//     (0..count).map(|i| (f)(xs, i)).collect()
// }

// TODO: validation that the key contains only characters from the key alphabet?
// TODO: allow custom key alphabets for PASCs, but don't allow Gronsfeld without key alphabet being digits?

pub fn caseless_keycheck(c: &char, keychars: &[char]) -> bool {
    keychars.contains(&c.to_ascii_uppercase()) || keychars.contains(&c.to_ascii_lowercase())
}

pub fn caseless_encipher(
    c: &char,
    k: &char,
    m: &ReciprocalTable<char, char, char>,
) -> Option<char> {
    if let Some(o) = m.encode(&c, &k) {
        Some(o)
    } else if let Some(o) = m.encode(&c.to_ascii_uppercase(), k) {
        Some(o.to_ascii_lowercase())
    } else if let Some(o) = m.encode(&c.to_ascii_lowercase(), k) {
        Some(o.to_ascii_uppercase())
    } else {
        None
    }
}

pub fn caseless_decipher(
    c: &char,
    k: &char,
    m: &ReciprocalTable<char, char, char>,
) -> Option<char> {
    if let Some(o) = m.decode(&c, &k) {
        Some(o)
    } else if let Some(o) = m.decode(&c.to_ascii_uppercase(), k) {
        Some(o.to_ascii_lowercase())
    } else if let Some(o) = m.decode(&c.to_ascii_lowercase(), k) {
        Some(o.to_ascii_uppercase())
    } else {
        None
    }
}

/// An Autoclave configuration.
#[derive(Copy, Clone)]
pub enum AutoclaveKind {
    None,
    Key,
    Text,
}

impl Default for AutoclaveKind {
    fn default() -> Self {
        Self::None
    }
}

/*
    // /// A key for encipherment and decipherment.
    // /// TODO: validation that the key contains only characters from the key alphabet?
    // pub fn key(&mut self, v: &str) -> &mut Self {
    //     self.key = v.to_string();
    //     self
    // }

    /// Create a new Vigenère cipher.
    /// Traditional construction:
    ///   K:  A-Z
    ///   PT: A-Z
    ///   CT: A-Z
    /// Alternate construction:
    ///   C(i) = caesar(shift=i)
    pub fn with_vigenere(key: &str) -> Self {
        let mut b = Self::default();
        b.operation = Operation::Vigenere;
        b.key = key.to_string();
        b
    }

    /// Create a new Gronsfeld cipher.
    /// Traditional construction:
    ///   K:  0-9
    ///   PT: A-Z
    ///   CT: A-Z
    /// Alternate construction:
    ///   C(i) = caesar(shift=i)
    pub fn with_gronsfeld(key: &str) -> Self {
        let mut b = Self::with_vigenere(key);
        b.key_alphabet = Alphabet::Digits;
        b
    }

    /// Create a new Beaufort cipher.
    /// Traditional construction:
    ///   K:  Z-A
    ///   PT: A-Z
    ///   CT: Z-A
    /// Alternate construction:
    ///   C(i) = affine(slope=(-1), intercept=i)
    pub fn with_beaufort(key: &str) -> Self {
        let mut b = Self::default();
        b.operation = Operation::Beaufort;
        b.key = key.to_string();
        b
    }

    /// Create a new variant Beaufort cipher.
    /// Traditional construction:
    ///   K:  A-Z
    ///   PT: Z-A
    ///   CT: Z-A
    /// Alternate construction:
    ///   C(i) = caesar(shift=(-i))
    pub fn with_variant_beaufort(key: &str) -> Self {
        let mut b = Self::default();
        b.operation = Operation::VariantBeaufort;
        b.key = key.to_string();
        b
    }

    // /// Create a new Trithemius cipher.
    // pub fn with_trithemius() -> Self {
    //     Self::with_vigenere(&self.pt_alphabet.to_string())
    // }

    /// Create a new Della Porta cipher.
    /// TODO: create a custom MASC for this? With a "shift" of X, will run owrap as necessary
    pub fn with_della_porta(key: &str) -> Self {
        let mut b = Self::default();
        b.operation = Operation::DellaPorta;
        b.key = key.to_string();
        b
    }
}
*/

// NOTES on design decisions:
// We can do a map of HashMap<char, masc::Cipher>, but this requires ciphers to be able to translate individual chars.
// It's a nice way to go in a language that treats chars as one-character strings (i.e., Python), but more wasteful
// in Rust.
//
// So instead we could consider one of:
//
// 1. 2x HashMap<char, HashMap<char, char>, with translation table (tabula recta) logic surrounding
// 2. 2x HashMap<char, translation_table> -- where a translation_table is a struct wrapping a HashMap<char, char> but also handling caseless translation
// 3. 2x HashMap<(char, char), char> -- a much larger hashmap, essentially a flattened version of #1. Cannot be ported to all languages (e.g., cannot be implemented in Go)
//    This mechanism is also potentially much simpler because only one map lookup needs to occur. The flip-side is that a new container for (char, char) may need to be
//    implemented, since there may (or may not) be borrow issues with multiple keys. Meanwhile, we may need to filter for invalid key characters beforehand because w
//    lose clarity into two separate error cases:
//
//    a. Key char found, but no plaintext/ciphertext counterpart found (i.e., non-transcodeable character) -- strict mode fallback logic to take effect as necessary
//    b. No key char found, so this was an invalid key char. Simply skip, or throw an error, but either way, do NOT cycle to the next transcodeable character yet.
//
// 4. No map, just math. Given key alphabet (a_K), plaintext alphabet (a_PT), and ciphertext alphabet (a_CT)...
//    a. Find key character position i_K in key alphabet as integer.
//    b. Find plaintext position i_PT in a_PT
//    c. Add these mod 26 to encipher
//    d. Find corresponding character in a_CT
//
// For decipherment:
//
//    a. Find key character position i_K in key alphabet as integer.
//    b. Find ciphertext position i_PT in a_CT
//    c. Subtract these mod 26 to decipher
//    d. Find corresponding character in a_PT
//
// This can be memoized, yielding results similar to the precomputed hashmap/translation table methods, but then is there a point doing this instead of precomputing?
// Also concerned that this method may become problematic with alternative ciphertext, plaintext, or key alphabets.
// Beaufort and Variant Beaufort may require special logic
// Finally, the Della Porta cipher becomes more difficult because the grid is the same, but the way to generate the alphabets within is different.

/*
/// A TabulaRecta implements a tabula recta.
pub struct TabulaRecta(HashMap<char, translation::Table>);

impl TabulaRecta {
    pub fn new(k: Alphabet, pt: Alphabet, ct: Alphabet) -> Self {
        let mut v = Vec::new();
        for i in 0..k.len() {
            let ct2 = ct.with_affine(1, i);
            let tt = translation::Table::new(&pt.to_string(), &ct2.to_string());
            v.push(tt);
        }
        Self(k.chars().zip(v).collect())
    }

    pub fn map(&self, s: &str, k: &str, strict: bool, caseless: bool) -> String {
        let mut key_iter = k.chars().filter(|c| self.0.contains_key(c)).cycle();

        let mut k: char = key_iter.next().unwrap();
        s.chars()
            .filter_map(|c| {
                let table = self.0.get(&k).unwrap();
                match table.get(c, caseless) {
                    Some(o) => {
                        k = key_iter.next().unwrap();
                        Some(o)
                    }
                    None => {
                        if !strict {
                            Some(c)
                        } else {
                            None
                        }
                    }
                }
            })
            .collect()
    }
}
*/

fn chainadder<T>(m: T, count: usize, primer: &[T]) -> Vec<T>
where
    T: Copy + Integer + Unsigned + std::iter::Product + std::iter::Sum,
{
    let g = LFGBuilder::default()
        .additive()
        .modulus(m)
        .seed(primer.to_vec())
        .taps(vec![1, 2])
        .build()
        .unwrap();
    primer.iter().copied().chain(g).take(count).collect()
}

pub fn makegromarkkey(primer: &[u32], msglen: usize) -> Vec<char> {
    // let primer: Vec<_> = k.chars().filter_map(|c| c.to_digit(10)).collect();
    chainadder(10, msglen, &primer)
        .iter()
        .filter_map(|&i| char::from_digit(i, 10))
        .collect()
}

// fn newgromark(primer: &[usize]) {
//     let key = "ENIGMA";
//     let primer = "23452";
//     let msg = "THEREAREUPTOTENSUBSTITUTESPERLETTER";

//     // TODO: msglen doesn't _need_ to go here; can just take() here in future version
//     // NOTE: this is the key to pass to the Gromark cipher now
//     // NOTE: this is the RK in Gromark (running key)
//     let k = makegromarkkey(primer, msg.chars().count());
//     let gronsfeld_key: String = k.chars().collect();

//     // NOTE: this is the MA in Gromark (mixed alphabet)
//     let pt_alphabet = alphabet::Alphabet::Latin.to_vec();
//     let key_vec: Vec<_> = key.chars().collect();
//     let ct_alphabet_input = masc::keyword_transform(&pt_alphabet, &key_vec);

//     let transposition_key: Vec<_> = key.chars().collect();
//     // let tc = transposition::ColumnarBuilder::default(&[&transposition_key], false);
//     // let transposed_ct_alphabet = tc.encipher(&ct_alphabet_input);

//     // let c = pasc::CipherBuilder::with_gronsfeld(&gronsfeld_key)
//     //     .ct_alphabet(&transposed_ct_alphabet)
//     //     .build();
//     // let m = message::Message::new(msg);
//     // m.encipher(&c);
//     // println!("enciphered message = {:?}", m);
//     // // NOTE: this is the GRO in Gromark (Gronsfeld)
// }

impl SubstitutionCipherBuilder<char, char> {
    // pub fn standard() -> Self {
    //     Self {
    //         pt_alphabet: Some(Alphabet::Latin.to_vec()),
    //         ..Default::default()
    //     }
    // }

    // // Prepare a Gromark cipher.
    // pub fn with_gromark(&mut self, keyword: &str, primer: &str) -> &mut Self {
    //     self.cipher(CipherKind::Gromark {
    //         keyword: keyword.chars().collect(),
    //         primer: primer.to_string(),
    //     })
    // }

    // pub fn str_key(&mut self, v: &str) -> &mut Self {
    //     self.key(v.chars().collect())
    // }

    pub fn caseless(&mut self, v: bool) -> &mut Self {
        if v {
            self.key_lookup = Some(Some(caseless_keycheck));
            self.enc_lookup = Some(Some(caseless_encipher));
            self.dec_lookup = Some(Some(caseless_decipher));
        } else {
            self.key_lookup = Some(None);
            self.enc_lookup = Some(None);
            self.dec_lookup = Some(None);
        }
        self
    }
}

/// A Cipher implements a polyalphabetic substitution cipher.
#[derive(Default, Builder, Clone)]
#[builder(default)]
pub struct SubstitutionCipher<T: Atom, K: Atom> {
    key: Vec<K>,

    #[builder(setter(into))]
    pt_alphabet: Vec<T>,
    #[builder(setter(into))]
    ct_alphabets: Vec<Vec<T>>,
    #[builder(setter(into))]
    key_alphabet: Vec<K>,

    autoclave: AutoclaveKind,
    strict: bool,

    #[builder(private)]
    key_lookup: Option<fn(&T, &[T]) -> bool>,
    #[builder(private)]
    enc_lookup: Option<fn(&T, &T, &ReciprocalTable<T, T, T>) -> Option<T>>,
    #[builder(private)]
    dec_lookup: Option<fn(&T, &T, &ReciprocalTable<T, T, T>) -> Option<T>>,

    #[builder(setter(skip))]
    ready: RefCell<bool>,

    #[builder(setter(skip))]
    tableau: RefCell<ReciprocalTable<K, T, T>>,
    // pt2ct: HashMap<char, translation::Table>,
    // ct2pt: HashMap<char, translation::Table>,
}

// 379 | impl<T: Atom, K: Atom> SubstitutionCipher<T, K> where Vec<T>: FromIterator<K> {
impl<T: Atom, K: Atom> SubstitutionCipher<T, K> {
    pub fn is_ready(&self) -> bool {
        *self.ready.borrow()
    }

    /// Encipher a single message atom.
    fn encipher_one(&self, c: &T, k: &K, t: &ReciprocalTable<K, T, T>) -> Option<T> {
        t.encode(&c, &k)
        // match self.enc_lookup {
        //     Some(f) => (f)(c, k, t),
        //     None => t.encode(&c, &k),
        // }
    }

    /// Decipher a single message atom.
    fn decipher_one(&self, c: &T, k: &K, t: &ReciprocalTable<K, T, T>) -> Option<T> {
        t.decode(&c, &k)
        // match self.dec_lookup {
        //     Some(f) => (f)(c, k, t),
        //     None => t.decode(&c, &k),
        // }
    }

    // /// Printable version of this cipher.
    // pub fn printable(&self) -> String {
    //     format!("PT: {}\nCT: {}", self.config.alphabet, self.config.alphabet)
    // }

    // TODO: msglen is currently ignored for non-Gromark. This is a kludge.
    fn initialize(&self) {
        if *self.ready.borrow() {
            return;
        }
        *self.ready.borrow_mut() = true;
        *self.tableau.borrow_mut() =
            ReciprocalTable::new(&self.pt_alphabet, &self.ct_alphabets, &self.key_alphabet);
    }

    // TODO: msglen is currently ignored for non-Gromark. This is a kludge.
    fn make_key(&self, msglen: usize) -> Vec<K> {
        let keychars = self.tableau.borrow().keyset();
        self.key
            .iter()
            .filter(|c| keychars.contains(&c))
            // .filter(|c| match self.key_lookup {
            //     Some(f) => (f)(&c, &keychars),
            //     None => keychars.contains(&c),
            // })
            .copied()
            .collect()
    }

    /// Encipher a string.
    pub fn encipher(&self, xs: &[T]) -> Vec<T> {
        self.initialize();
        let mut kq = KeyQueue::from(self.make_key(xs.len()));

        let tr = self.tableau.borrow();

        xs.iter()
            // can use .scan(0, |cursor, &c| if we're not going to return None
            .filter_map(|&c| {
                let k = kq.get(); // TODO: add back caseless checks if we keep caseless option
                                  // let raw_out = self.encipher_one(&c, &k, &tr);
                let raw_out = tr.encode(&c, &k);
                match raw_out {
                    Some(o) => {
                        let elem = kq.pop();
                        match self.autoclave {
                            AutoclaveKind::None => kq.push(elem),
                            AutoclaveKind::Key => kq.push(elem),
                            AutoclaveKind::Text => kq.push(elem),
                            // AutoclaveKind::Key => kq.push(o),
                            // AutoclaveKind::Text => kq.push(c),
                        };
                        Some(o)
                    }
                    None => {
                        if !self.strict {
                            Some(c)
                        } else {
                            None
                        }
                    }
                }
            }) // .filter_map for caseless
            .collect()
    }

    /// Decipher a string.
    pub fn decipher(&self, xs: &[T]) -> Vec<T> {
        self.initialize();
        let mut kq = KeyQueue::from(self.make_key(xs.len()));

        let tr = self.tableau.borrow();

        xs.iter()
            // can use .scan(0, |cursor, &c| if we're not going to return None
            .filter_map(|&c| {
                let k = kq.get(); // TODO: add back caseless checks if we keep caseless option
                                  // let raw_out = self.decipher_one(&c, &k, &tr);
                let raw_out = tr.decode(&c, &k);
                match raw_out {
                    Some(o) => {
                        let elem = kq.pop();
                        match self.autoclave {
                            AutoclaveKind::None => kq.push(elem),
                            AutoclaveKind::Key => kq.push(elem),
                            AutoclaveKind::Text => kq.push(elem),
                            // AutoclaveKind::Key => kq.push(c),
                            // AutoclaveKind::Text => kq.push(o),
                        };
                        Some(o)
                    }
                    None => {
                        if !self.strict {
                            Some(c)
                        } else {
                            None
                        }
                    }
                }
            })
            .collect()
    }
}
