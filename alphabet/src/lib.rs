/*use lcg::LCG;
use std::collections::HashMap;
use std::fmt;

#[cfg(test)]
mod tests {
    // use super::deduplicate;
    // #[test]
    // fn test_deduplicate() {
    //     let tables = &[
    //         ("", ""),
    //         ("a", "a"),
    //         ("aa", "a"),
    //         ("aaA", "aA"),
    //         ("aaAA", "aA"),
    //         ("*", "*"),
    //         ("**", "*"),
    //         ("Hello, world!", "Helo, wrd!"),
    //     ];

    //     for (s, expected) in tables {
    //         assert_eq!(deduplicate(*s), *expected);
    //     }
    // }
}*/

const LATIN: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";

#[derive(Clone)]
pub enum Alphabet {
    Latin,
    Digits,
    Str(String),
}

impl Alphabet {
    pub fn to_string(&self) -> String {
        match self {
            Self::Latin => LATIN.to_string(),
            Self::Digits => DIGITS.to_string(),
            Self::Str(s) => s.to_string(),
        }
    }

    pub fn to_vec(&self) -> Vec<char> {
        match self {
            Self::Latin => LATIN.chars().collect(),
            Self::Digits => DIGITS.chars().collect(),
            Self::Str(s) => s.chars().collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.to_vec().len()
    }
}

impl Default for Alphabet {
    fn default() -> Self {
        Self::Latin
    }
}

/*
// An alphabet contains a deduplicated string for use in substitution ciphers.
#[derive(Clone)]
pub struct Alphabet(String);

impl Alphabet {
    /// New alphabet.
    pub fn new(s: &str) -> Self {
        Alphabet(String::from(s))
    }

    /// Length of the alphabet.
    pub fn len(&self) -> usize {
        self.0.chars().count()
    }

    /// Latin alphabet.
    pub fn with_latin() -> Self {
        Self::new(LATIN)
    }

    /// Digits as an alphabet.
    pub fn with_digits() -> Self {
        Self::new(DIGITS)
    }

    /// Convert to string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /*
    /// Create a new alphabet based on a transformation.
    // pub fn with_transform(&self, f: impl Fn(u32) -> u32) -> Self {
    fn with_transform(&self, m: Vec<usize>) -> Self {
        let mut out = String::new();

        let chars: Vec<_> = self.0.chars().collect();
        for i in m {
            let new_char = chars[i];
            out.push(new_char)
        }
        Self::new(&out)
    }*/
}

impl fmt::Display for Alphabet {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

// impl Iterator for Alphabet {
//     type Item = char;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.chars().next()
//     }
// }
*/
