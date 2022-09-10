use std::cell::RefCell;
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

/// Create a translation table. This is modeled off of the Python str.maketrans method.
fn maketrans<T, U>(xs: &[T], ys: &[U]) -> HashMap<T, U>
where
    T: Copy + Hash + Eq,
    U: Copy + Eq,
{
    xs.iter().zip(ys.iter()).map(|(&x, &y)| (x, y)).collect()
}

// use macro_rules! <name of macro>{<Body>}
macro_rules! maketrans2{
    // macth like arm for macro
       ($xs:expr,$ys:expr)=>{
    // macro expand to this code
           {
   // $a and $b will be templated using the value/variable provided to macro
               $xs.iter().zip($ys.iter()).map(|(&x, &y)| (x, y)).collect()
           }
       }
   }

pub trait Atom: Hash + Eq + Copy + Default {}
impl<T> Atom for T where T: Hash + Eq + Copy + Default {}

#[derive(Default, Clone)]
pub struct Tableau2<T, U>
where
    T: Atom,
    U: Atom,
{
    pub pt_alphabet: Vec<T>,
    pub ct_alphabet: Vec<U>,

    tableau: Tableau<T, U>,
}

impl<T, U> Tableau2<T, U>
where
    T: Atom,
    U: Atom,
{
    pub fn new(pt_alphabet: &[T], ct_alphabet: &[U]) -> Self {
        return Self {
            pt_alphabet: pt_alphabet.to_owned(),
            ct_alphabet: ct_alphabet.to_owned(),
            tableau: Tableau::new(pt_alphabet, ct_alphabet),
        };
    }

    /// Encode an element.
    pub fn encode(&self, x: &T) -> Option<U> {
        self.tableau.encode(x)
    }

    /// Decode an element.
    pub fn decode(&self, x: &U) -> Option<T> {
        self.tableau.decode(x)
    }
}

/// A Tableau implements the mechanism underlying a simple monoalphabetic substitution cipher.
// #[derive(Default, Builder, Clone)]
// #[builder(default)]
#[derive(Default, Clone)]
pub struct Tableau<T: Atom, U: Atom>(HashMap<T, U>, HashMap<U, T>);

impl<T: Atom, U: Atom> Tableau<T, U> {
    pub fn new(xs: &[T], ys: &[U]) -> Self {
        Self(maketrans(xs, ys), maketrans2!(ys, xs))
    }

    /// Encode an element.
    pub fn encode(&self, x: &T) -> Option<U> {
        self.0.get(x).copied()
    }

    /// Decode an element.
    pub fn decode(&self, x: &U) -> Option<T> {
        self.1.get(x).copied()
    }

    /// Empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }
}

// impl<T: Atom> Tableau<T, T> {
//     /// Encode an element or return the supplied element unchanged.
//     pub fn encode_lenient(&self, x: &T) -> T {
//         self.encode(x).unwrap_or(*x)
//     }

//     /// Decode an element or return the supplied element unchanged.
//     pub fn decode_lenient(&self, x: &T) -> T {
//         self.decode(x).unwrap_or(*x)
//     }
// }

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
