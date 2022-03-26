use cipher::Cipher;
use derive_builder::Builder;
use masc::tableau::Atom;
use masc::SubstitutionCipherBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        keyword: Vec<T>,

        charset: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                keyword: [5, 3, 3].to_vec(),
                charset: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0].to_vec(),
                strict: false,
            },
            TestCase {
                keyword: [5, 3, 3].to_vec(),
                charset: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [5, 3, 1, 2, 4, 4, 2, 1, 3, 5].to_vec(),
                strict: true,
            },
        ];
        for x in xs {
            let c = KeywordBuilder::default()
                .charset(x.charset.to_vec())
                .strict(x.strict)
                .keyword(x.keyword.to_vec())
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
                charset: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0].to_vec(),
                output: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                strict: false,
            },
            TestCase {
                keyword: [5, 3, 3].to_vec(),
                charset: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 5, 3, 1, 2, 4, 4, 2, 1, 3, 5, 0].to_vec(),
                output: [1, 2, 3, 4, 5, 5, 4, 3, 2, 1].to_vec(),
                strict: true,
            },
        ];
        for x in xs {
            let c = KeywordBuilder::default()
                .charset(x.charset.to_vec())
                .strict(x.strict)
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

    charset: Vec<T>,
    strict: bool,
}

impl<T: Atom> Cipher<T> for Keyword<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let c = SubstitutionCipherBuilder::default()
            .with_keyword(&self.keyword)
            .pt_alphabet(self.charset.to_vec())
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let c = SubstitutionCipherBuilder::default()
            .with_keyword(&self.keyword)
            .pt_alphabet(self.charset.to_vec())
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
