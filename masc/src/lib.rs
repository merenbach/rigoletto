pub mod tableau;
pub mod transform;

use alphabet::Alphabet;
use cipher::Cipher;
use derive_builder::Builder;
use std::cell::RefCell;
use std::fmt;
use tableau::{Atom, Tableau};

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn test_affine_transform() {
//     //     assert_eq!(2 + 2, 4);
//     // }

//     // #[test]
//     // fn test_keyword_transform() {
//     //     assert_eq!(2 + 2, 4);
//     // }
// }

// Four total possible combinations yield three possible outcomes:
// 1. HashMap entry v found for key k, strict enabled: return Some(v)
// 2. HashMap entry v found for key k, strict disabled: return Some(v)
// 3. HashMap entry v not found for key k, strict enabled: return None
// 4. HashMap entry v not found for key k, strict disabled: return Some(k)
/*
// Get a transcoded rune (optionally ignoring case) and a boolean indicating success.
// Get (-1) instead if strict mode is enabled.
// Get the original rune back instead if strict mode is disabled.
*/

// /// Modulus performs a Euclidean remainder operation.
// /// // TODO: add test
// fn modulus<T: Integer + Copy>(x: T, y: T) -> T {
//     return ((x % y) + y) % y;
// }

// fn build_lcg<T>(seed: T, multiplier: T, increment: T, modulus: T, count: usize) -> Vec<T>
// where
//     T: Integer + Unsigned + Copy,
// {
//     use std::iter::successors;

//     successors(Some(seed), |n| multiplier * seed + increment % modulus)
//         .skip(1)
//         .take(count);
// }

// fn sentinel_subtract(x: usize) -> impl Fn(usize) -> usize {
//     return move |i: usize| -> usize { return i - x };
// }

// enum ModNum {
//     Natural(usize),
//     SubtractFrom(usize),
// }

// A CipherKind is any base case or special case of a cipher that has side effects.
#[derive(Clone)]
enum CipherKind<T: Atom> {
    None,

    Affine { slope: usize, intercept: usize },
    Atbash,
    Dummy,
    Caesar(usize),
    Decimation(usize),
    Keyword(Vec<T>),
    ROT13,
}

impl<T: Atom> Default for CipherKind<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T: Atom> CipherKind<T> {
    pub fn transform(&self, xs: &[T]) -> Vec<T> {
        match self {
            CipherKind::Affine { slope, intercept } => transform::affine(xs, *slope, *intercept), // TODO: check for coprimality between xs.len() and slope
            CipherKind::Atbash => transform::atbash(xs),
            CipherKind::Caesar(shift) => transform::caesar(xs, *shift),
            CipherKind::Decimation(multiplier) => transform::decimation(xs, *multiplier),
            CipherKind::Dummy => transform::dummy(xs),
            CipherKind::Keyword(keyword) => transform::keyword(xs, &keyword),
            CipherKind::ROT13 => transform::caesar(xs, 13),
            CipherKind::None => xs.to_vec(),
        }
    }
}

fn caseless_lookup<F>(c: &char, f: F) -> Option<char>
where
    F: Fn(char) -> Option<char>,
{
    if let Some(o) = f(*c) {
        Some(o)
    } else if let Some(o) = f(c.to_ascii_uppercase()) {
        Some(o.to_ascii_lowercase())
    } else if let Some(o) = f(c.to_ascii_lowercase()) {
        Some(o.to_ascii_uppercase())
    } else {
        None
    }
}

fn caseless_encipher(c: &char, m: &Tableau<char, char>) -> Option<char> {
    caseless_lookup(c, |c2| m.encode(&c2))
}

fn caseless_decipher(c: &char, m: &Tableau<char, char>) -> Option<char> {
    caseless_lookup(c, |c2| m.decode(&c2))
}

/// A SubstitutionCipher implements a monoalphabetic substitution cipher.
#[derive(Default, Builder, Clone)]
#[builder(default)]
pub struct SubstitutionCipher<T: Atom> {
    #[builder(private)]
    cipher: CipherKind<T>,
    strict: bool,

    pt_alphabet: Vec<T>,
    #[builder(setter(strip_option))]
    ct_alphabet: Option<Vec<T>>,

    #[builder(private)]
    enc_lookup: Option<fn(&T, &Tableau<T, T>) -> Option<T>>,
    #[builder(private)]
    dec_lookup: Option<fn(&T, &Tableau<T, T>) -> Option<T>>,

    #[builder(setter(skip))]
    ready: RefCell<bool>,
    #[builder(setter(skip))]
    tableau: RefCell<Tableau<T, T>>,
}

// TODO: add some notes here regarding why we don't make Caesar/decimation ctors call with_affine.
//       In short, it's so that we'll have transformation primitives available for reuse in other ciphers.
//       These primitives can be tested independently of cipher integration testing.
//       As a design policy here, we'll have one CipherKind per possible initialization type.
impl<T: Atom> SubstitutionCipherBuilder<T> {
    // Prepare an affine cipher.
    pub fn with_affine(&mut self, slope: usize, intercept: usize) -> &mut Self {
        self.cipher(CipherKind::Affine { slope, intercept })
    }

    // Prepare an Atbash cipher.
    pub fn with_atbash(&mut self) -> &mut Self {
        self.cipher(CipherKind::Atbash)
    }

    // Prepare a Caesar cipher.
    pub fn with_caesar(&mut self, shift: usize) -> &mut Self {
        self.cipher(CipherKind::Caesar(shift))
    }

    // Prepare a decimation cipher.
    pub fn with_decimation(&mut self, multiplier: usize) -> &mut Self {
        self.cipher(CipherKind::Decimation(multiplier))
    }

    // Prepare a dummy (no-op) cipher.
    pub fn with_dummy(&mut self) -> &mut Self {
        self.cipher(CipherKind::Dummy)
    }

    // Prepare a keyword cipher.
    pub fn with_keyword(&mut self, keyword: &[T]) -> &mut Self {
        self.cipher(CipherKind::Keyword(keyword.to_vec()))
    }

    // Prepare a Rot13 cipher.
    pub fn with_rot13(&mut self) -> &mut Self {
        self.cipher(CipherKind::ROT13)
    }
}

impl SubstitutionCipherBuilder<char> {
    pub fn standard() -> Self {
        Self {
            pt_alphabet: Some(Alphabet::Latin.to_vec()),
            ..Default::default()
        }
    }

    // pub fn with_pt_alphabet_str(&mut self, v: &str) -> &mut Self {
    //     self.pt_alphabet(v.chars().collect())
    // }

    // pub fn with_ct_alphabet_str(&mut self, v: &str) -> &mut Self {
    //     self.ct_alphabet(v.chars().collect())
    // }

    // Prepare a keyword cipher.
    pub fn with_keyword_str(&mut self, keyword: &str) -> &mut Self {
        let kw: Vec<_> = keyword.chars().collect();
        self.with_keyword(&kw)
    }

    pub fn caseless(&mut self, v: bool) -> &mut Self {
        if v {
            self.enc_lookup = Some(Some(caseless_encipher));
            self.dec_lookup = Some(Some(caseless_decipher));
        } else {
            self.enc_lookup = Some(None);
            self.dec_lookup = Some(None);
        }
        self
    }
}

impl<T: Atom> SubstitutionCipher<T> {
    fn initialize(&self) {
        if *self.ready.borrow() {
            return;
        }
        *self.ready.borrow_mut() = true;

        // let pt_alphabet: Vec<T> = match &self.cipher {
        //     CipherKind::ROT13 => Alphabet::Latin.to_vec(),
        //     _ => pt_alphabet_orig.to_vec(),
        // };

        *self.tableau.borrow_mut() = Tableau::new(
            &self.pt_alphabet,
            &self.ct_alphabet.as_ref().unwrap_or(&self.pt_alphabet),
            |xs| self.cipher.transform(xs),
        );
    }

    // /// Printable version of this cipher.
    // pub fn printable(&self) -> String {
    //     let ab = &self.alphabet;
    //     format!("PT: {}\nCT: {}", ab, self.encipher(ab))
    // }

    /// Encipher a single message atom.
    pub fn encipher_one(&self, c: &T) -> Option<T> {
        self.initialize();
        match self.enc_lookup {
            Some(f) => (f)(c, &self.tableau.borrow()),
            None => self.tableau.borrow().encode(&c),
        }
    }

    /// Decipher a single message atom.
    pub fn decipher_one(&self, c: &T) -> Option<T> {
        self.initialize();
        match self.dec_lookup {
            Some(f) => (f)(c, &self.tableau.borrow()),
            None => self.tableau.borrow().decode(&c),
        }
    }
}

impl<T: Atom> Cipher<T> for SubstitutionCipher<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        xs.iter()
            .filter_map(|&c| {
                self.encipher_one(&c).or({
                    if !self.strict {
                        Some(c)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        xs.iter()
            .filter_map(|&c| {
                self.decipher_one(&c).or({
                    if !self.strict {
                        Some(c)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

// TODO: ensure we have tests for this
impl fmt::Display for SubstitutionCipher<char> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write into the supplied output stream: `f`.
        // Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let pt_alphabet: String = self.pt_alphabet.iter().collect();
        let ct_alphabet: String = self
            .ct_alphabet
            .as_ref()
            .unwrap_or(&self.pt_alphabet)
            .iter()
            .collect();
        write!(f, "<PT: {}, CT: {}>", &pt_alphabet, &ct_alphabet)
    }
}
