use crate::Cipher;
use alphabet::Alphabet;
use derive_builder::Builder;
use pasc::SubstitutionCipherBuilder;

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
pub struct ReciprocalTable {
    key: Vec<char>,
    ct_alphabets: Vec<Vec<char>>,

    #[builder(default)]
    key_alphabet: Option<Vec<char>>,

    pt_alphabet: Option<Vec<char>>,

    #[builder(default)]
    strict: bool,
}

impl Cipher<char, char> for ReciprocalTable {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[char]) -> Vec<char> {
        let pt_alphabet = self
            .pt_alphabet
            .as_ref()
            .unwrap_or(&Alphabet::Latin.to_vec())
            .to_vec();
        let key_alphabet = self.key_alphabet.as_ref().unwrap_or(&pt_alphabet).to_vec();
        let c = SubstitutionCipherBuilder::default()
            .key(self.key.to_vec())
            .pt_alphabet(pt_alphabet)
            .ct_alphabets(self.ct_alphabets.to_vec())
            .key_alphabet(key_alphabet)
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[char]) -> Vec<char> {
        let pt_alphabet = self
            .pt_alphabet
            .as_ref()
            .unwrap_or(&Alphabet::Latin.to_vec())
            .to_vec();
        let key_alphabet = self.key_alphabet.as_ref().unwrap_or(&pt_alphabet).to_vec();
        let c = SubstitutionCipherBuilder::default()
            .key(self.key.to_vec())
            .pt_alphabet(pt_alphabet)
            .ct_alphabets(self.ct_alphabets.to_vec())
            .key_alphabet(key_alphabet)
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
