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
    }

    #[test]
    fn encipher_works() {
        let xs = &[TestCase {
            keyword: vec![5, 3, 3],
            pt_alphabet: vec![1, 2, 3, 4, 5],
            input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            output: vec![0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0],
        }];
        for x in xs {
            let c = KeywordBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .keyword(x.keyword.to_vec())
                .build()
                .unwrap();
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[TestCase {
            keyword: vec![5, 3, 3],
            pt_alphabet: vec![1, 2, 3, 4, 5],
            input: vec![0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0],
            output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
        }];
        for x in xs {
            let c = KeywordBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .keyword(x.keyword.to_vec())
                .build()
                .unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

#[derive(Default, Builder)]
pub struct Keyword<T: Atom> {
    keyword: Vec<T>,

    #[builder(setter(into))]
    pt_alphabet: Vec<T>,
}

impl<T: Atom> Cipher<T, T> for Keyword<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let c = simple::make(&self.pt_alphabet, |xs| {
            transform::keyword(xs, &self.keyword)
        });
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let c = simple::make(&self.pt_alphabet, |xs| {
            transform::keyword(xs, &self.keyword)
        });
        c.decipher(xs)
    }
}
