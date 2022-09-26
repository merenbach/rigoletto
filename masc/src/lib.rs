pub mod tableau;
pub mod transform;

use cipher::Cipher;
use std::cell::RefCell;
use std::fmt;
use tableau::Atom;
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

// TODO: reduce locations where we might use clone
#[derive(Default, Clone)]
pub struct SubstitutionCipher<T: Atom> {
    pt_alphabet: Vec<T>,
    ct_alphabet: Vec<T>,

    pt2ct: Table<T>,
    ct2pt: Table<T>,

    strict: bool,
}

impl<T: Atom> SubstitutionCipher<T> {
    pub fn new(pt_alphabet: &[T], ct_alphabet: &[T], strict: bool) -> Self {
        let pt2ct = TableBuilder::default()
            .src(pt_alphabet.to_owned())
            .dst(ct_alphabet.to_owned())
            .build()
            .unwrap();

        let ct2pt = TableBuilder::default()
            .src(ct_alphabet.to_owned())
            .dst(pt_alphabet.to_owned())
            .build()
            .unwrap();

        Self {
            pt_alphabet: pt_alphabet.to_owned(),
            ct_alphabet: ct_alphabet.to_owned(),
            pt2ct,
            ct2pt,
            strict,
        }
    }

    /// Encipher an element.
    pub fn encipher_one(&self, x: &T) -> Option<T> {
        self.pt2ct.translate_one(x, |_| None)
    }

    /// Decipher an element.
    pub fn decipher_one(&self, x: &T) -> Option<T> {
        self.ct2pt.translate_one(x, |_| None)
    }
}

impl<T> Cipher<T, T> for SubstitutionCipher<T>
where
    T: Atom,
{
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        self.pt2ct
            .translate(xs, |x| if self.strict { None } else { Some(x) })
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        self.ct2pt
            .translate(xs, |x| if self.strict { None } else { Some(x) })
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
