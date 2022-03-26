use std::collections::HashMap;
use std::hash::Hash;

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

pub trait Atom: Hash + Eq + Copy + Default {}
impl<T> Atom for T where T: Hash + Eq + Copy + Default {}

/// A Tableau implements the mechanism underlying a simple monoalphabetic substitution cipher.
// #[derive(Default, Builder, Clone)]
// #[builder(default)]
#[derive(Default, Clone)]
pub struct Tableau<T: Atom, U: Atom>(HashMap<T, U>, HashMap<U, T>);

impl<T: Atom, U: Atom> Tableau<T, U> {
    pub fn new<F>(xs: &[T], ys: &[U], f: F) -> Self
    where
        F: Fn(&[U]) -> Vec<U>,
    {
        let zs = (f)(ys);
        Self(
            xs.iter().copied().zip(zs.iter().copied()).collect(),
            zs.iter().copied().zip(xs.iter().copied()).collect(),
        )
    }

    /// Encode an element.
    pub fn encode(&self, x: &T) -> Option<U> {
        self.0.get(x).copied()
    }

    /// Decode an element.
    pub fn decode(&self, x: &U) -> Option<T> {
        self.1.get(x).copied()
    }
}

// // TODO: ensure we have tests for this
// // TODO: require only displayable for this, not char
// impl<T, U> fmt::Display for Tableau<T, U>
// where
//     T: Atom,
//     U: Atom,
// {
//     // This trait requires `fmt` with this exact signature.
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write into the supplied output stream: `f`.
//         // Returns `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         write!(
//             f,
//             "<Tableau with PT len {} and CT len {}>",
//             &self.pt_alphabet.len(),
//             &self.ct_alphabet.len()
//         )
//     }
// }

// pub fn make_table<T, U, F>(xs: &[T], ys: &[U], f: F) -> Tableau<T, U>
// where
//     T: Atom,
//     U: Atom,
//     F: Fn(&[U]) -> Vec<U>,
// {
//     Tableau::new(xs, &(f)(ys))
// }
