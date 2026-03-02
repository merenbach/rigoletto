pub mod transform;

use cipher::Cipher;
use derive_builder::Builder;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use translation::{Table, TableBuilder};

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

pub trait Atom: Hash + Eq + Copy + Default {}
impl<T> Atom for T where T: Hash + Eq + Copy + Default {}

#[derive(Default, Builder)]
#[builder(default)]
pub struct SubstitutionCipher<T: Atom> {
    #[builder(setter(into))]
    pt_alphabet: Vec<T>,
    #[builder(setter(into))]
    ct_alphabet: Vec<T>,

    #[builder(setter(skip))]
    pt2ct: RefCell<HashMap<T, T>>,
    #[builder(setter(skip))]
    ct2pt: RefCell<HashMap<T, T>>,

    strict: bool,
}

impl<T: Atom> SubstitutionCipher<T> {
    fn initialize(&self) {
        if self.pt2ct.borrow().is_empty() {
            *self.pt2ct.borrow_mut() = self
                .pt_alphabet
                .to_owned()
                .into_iter()
                .zip(self.ct_alphabet.to_owned())
                .collect();
        }

        if self.ct2pt.borrow().is_empty() {
            *self.ct2pt.borrow_mut() = self
                .ct_alphabet
                .to_owned()
                .into_iter()
                .zip(self.pt_alphabet.to_owned())
                .collect();
        }
    }

    /// Encipher an element.
    pub fn encipher_one(&self, x: &T) -> Option<T> {
        self.initialize();
        if let Some(y) = self.pt2ct.borrow().get(x) {
            return Some(*y);
        } else {
            if self.strict {
                return None;
            } else {
                return Some(*x);
            }
        }
    }

    /// Decipher an element.
    pub fn decipher_one(&self, x: &T) -> Option<T> {
        self.initialize();
        if let Some(y) = self.ct2pt.borrow().get(x) {
            return Some(*y);
        } else {
            if self.strict {
                return None;
            } else {
                return Some(*x);
            }
        }
    }
}

impl<T> Cipher<T, T> for SubstitutionCipher<T>
where
    T: Atom,
{
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        xs.iter().filter_map(|x| self.encipher_one(x)).collect()
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        xs.iter().filter_map(|x| self.decipher_one(x)).collect()
    }
}

// TODO: ensure we have tests for this
impl fmt::Display for SubstitutionCipher<char> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pt_alphabet: String = self.pt_alphabet.iter().collect();
        let ct_alphabet: String = self.ct_alphabet.iter().collect();
        write!(f, "Simple <PT: {}, CT: {}>", &pt_alphabet, &ct_alphabet)
    }
}
