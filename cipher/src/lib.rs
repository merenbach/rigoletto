pub mod affine;
pub mod atbash;
pub mod beaufort;
pub mod caesar;
// pub mod columnar;
pub mod chaocipher;
pub mod decimation;
pub mod della_porta;
pub mod dummy;
pub mod gromark;
pub mod gronsfeld;
pub mod trithemius;
// pub mod hill;
pub mod homophonic;
pub mod keyword;
pub mod rail_fence;
pub mod reciprocal_table;
pub mod scytale;
pub mod simple;
pub mod variant_beaufort;
pub mod vigenere;
// pub mod vic;

// use std::fmt;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

/// A Cipher implements a generic cipher.
pub trait Cipher<T, U>
where
    T: Copy,
    U: Copy,
{
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<U>;

    /// Decipher a sequence.
    fn decipher(&self, xs: &[U]) -> Vec<T>;
}

// pub trait Atom: Hash + Eq + Copy + Default {}
// impl<T> Atom for T where T: Hash + Eq + Copy + Default {}

// impl<T:Copy> Cipher<T,T> for SubstitutionCipher<T> {}

// TODO: strict mode/caseless mode can be during encipherment, but we can also use sentinels
// if this were a Message(Vec<(char, char, bool>) instead--or similar--we could use the second or third slots of each tuple to indicate
// if a char was in the original but didn't exist in the translated version, or if a case change would resolve this issue
// Then we could have a to_string(caseless, strict) option that would then resolve these sentinels
// This removes strict/caseless from the encipherment/decipherment process, simplifying both internal and external APIs

// // TODO: rename to Atom?
// #[derive(Clone, Copy, Debug)]
// pub struct Symbol {
//     pub c: char, // For character contents
//     //  i:i32, // For integer contents
//     pub deleted: bool,
//     // recased: bool,
//     // isnull: bool,
// }

// #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
// pub enum Atom {
//     StandardChar(char),
//     DeletedChar(char),
//     DeletedCharIfNotCaseless(char),
//     Null(char),
// }

// impl Atom {
//     // TODO: Option<char>
//     pub fn contents(&self) -> char {
//         match &self {
//             Atom::StandardChar(c) => *c,
//             Atom::DeletedChar(c) => *c,
//             Atom::DeletedCharIfNotCaseless(c) => *c,
//             Atom::Null(c) => *c,
//             // _ => ...
//         }
//     }
// }

// impl fmt::Display for Atom {
//     // This trait requires `fmt` with this exact signature.
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write strictly the first element into the supplied output
//         // stream: `f`. Returns `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         write!(f, "{:?}", self)
//     }
// }

// impl Symbol {
//     // Replace the contents of this symbol.
//     pub fn replace(&self, v: char) -> Self {
//         let mut s = self.clone();
//         s.c = v;
//         s
//     }

//     // Mark as deleted.
//     pub fn delete(&self) -> Self {
//         let mut s = self.clone();
//         s.deleted = true;
//         s
//     }
// }

// {before: char, after: Option<char>}

// TODO: use enum with Some(c) and Missing(c)
