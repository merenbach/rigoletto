// use cipher::Atom;
use cipher::{Cipher, SubstitutionCipher};
use std::fmt;

// use std::collections::HashSet;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

/// A Message to transcode.
pub struct Message<T>(Vec<T>)
where
    T: Copy;

impl<T> Message<T>
where
    T: Copy,
{
    /// Encipher the current state of the message.
    pub fn encipher(&mut self, c: &dyn Cipher<T, T>) -> &mut Self {
        self.0 = c.encipher(&self.0);
        self
    }

    /// Decipher the current state of the message.
    pub fn decipher(&mut self, c: &dyn Cipher<T, T>) -> &mut Self {
        self.0 = c.decipher(&self.0);
        self
    }

    /// Encipher the current state of the message.
    /// Be lenient with characters that cannot be enciphered.
    pub fn encipher_retain(&mut self, c: &dyn SubstitutionCipher<T>) -> &mut Self {
        self.0 = c.encipher_retain(&self.0);
        self
    }

    /// Decipher the current state of the message.
    /// Be lenient with characters that cannot be deciphered.
    pub fn decipher_retain(&mut self, c: &dyn SubstitutionCipher<T>) -> &mut Self {
        self.0 = c.decipher_retain(&self.0);
        self
    }

    // /// Retain only certain characters.
    // pub fn retain(&mut self, xs: &[T]) -> &mut Self {
    //     self.0.retain(|y| xs.contains(y));
    //     self
    // }

    // /// Retain only characters in a given string, effectively a union operation.
    // pub fn retain_str(&mut self, v: &str) -> &mut Self {
    //     let chars: HashSet<_> = v.chars().collect();
    //     self.0.retain(|c| chars.contains(&c));
    //     self
    // }

    // /// Expunge symbols marked for deletion.
    // pub fn expunge(&mut self) -> &mut Self {
    //     self.0.retain(|c| match c {
    //         Atom::StandardChar(_) => true,
    //         Atom::DeletedChar(_) => false,
    //         Atom::Null(_) => true,
    //         Atom::DeletedCharIfNotCaseless(_) => false,
    //     });
    //     self
    // }

    // chunk
    // rm spaces (or strip to alphabet)
    // pad to length (or pad to multiple of...)
}

// TODO: allow from String as well?
impl From<String> for Message<char> {
    fn from(s: String) -> Self {
        let v: Vec<_> = s.chars().collect();
        Message(v)
    }
}

impl fmt::Display for Message<char> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let s: String = self.0.iter().collect();
        write!(f, "{}", s)
    }
}
