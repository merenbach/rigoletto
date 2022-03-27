use crate::simple;
use crate::Cipher;
use derive_builder::Builder;
use masc::tableau::Atom;
use masc::transform;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        keyword: Vec<T>,

        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                keyword: [5, 3, 3].to_vec(),
                pt_alphabet: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0].to_vec(),
                strict: false,
            },
            TestCase {
                keyword: [5, 3, 3].to_vec(),
                pt_alphabet: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [5, 3, 1, 2, 4, 4, 2, 1, 3, 5].to_vec(),
                strict: true,
            },
        ];
        for x in xs {
            let c = KeywordBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .keyword(x.keyword.to_vec())
                .strict(x.strict)
                .build()
                .unwrap();
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                keyword: [5, 3, 3].to_vec(),
                pt_alphabet: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0].to_vec(),
                output: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                strict: false,
            },
            TestCase {
                keyword: [5, 3, 3].to_vec(),
                pt_alphabet: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0].to_vec(),
                output: [1, 2, 3, 4, 5, 5, 4, 3, 2, 1].to_vec(),
                strict: true,
            },
        ];
        for x in xs {
            let c = KeywordBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .keyword(x.keyword.to_vec())
                .strict(x.strict)
                .build()
                .unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

#[derive(Default, Builder)]
pub struct Keyword<T: Atom> {
    keyword: Vec<T>,

    pt_alphabet: Vec<T>,
    strict: bool,
}

impl<T: Atom> Cipher<T, T> for Keyword<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let ct_alphabet = transform::keyword(&self.pt_alphabet, &self.keyword);
        let c = simple::SimpleBuilder::default()
            .pt_alphabet(self.pt_alphabet.to_vec())
            .ct_alphabet(ct_alphabet)
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let ct_alphabet = transform::keyword(&self.pt_alphabet, &self.keyword);
        let c = simple::SimpleBuilder::default()
            .pt_alphabet(self.pt_alphabet.to_vec())
            .ct_alphabet(ct_alphabet)
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
