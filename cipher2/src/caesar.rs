use crate::Cipher;
use cipher::Cipher as _;
use derive_builder::Builder;
use masc::tableau::Atom;
use masc::SubstitutionCipherBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        offset: usize,

        charset: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                offset: 3,
                charset: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [0, 4, 5, 1, 2, 3, 3, 2, 1, 5, 4, 0].to_vec(),
                strict: false,
            },
            TestCase {
                offset: 3,
                charset: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                output: [4, 5, 1, 2, 3, 3, 2, 1, 5, 4].to_vec(),
                strict: true,
            },
        ];
        for x in xs {
            let c = CaesarBuilder::default()
                .charset(x.charset.to_vec())
                .strict(x.strict)
                .offset(x.offset)
                .build()
                .unwrap();
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                offset: 3,
                charset: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 4, 5, 1, 2, 3, 3, 2, 1, 5, 4, 0].to_vec(),
                output: [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0].to_vec(),
                strict: false,
            },
            TestCase {
                offset: 3,
                charset: [1, 2, 3, 4, 5].to_vec(),
                input: [0, 4, 5, 1, 2, 3, 3, 2, 1, 5, 4, 0].to_vec(),
                output: [1, 2, 3, 4, 5, 5, 4, 3, 2, 1].to_vec(),
                strict: true,
            },
        ];
        for x in xs {
            let c = CaesarBuilder::default()
                .charset(x.charset.to_vec())
                .strict(x.strict)
                .offset(x.offset)
                .build()
                .unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

#[derive(Default, Builder)]
pub struct Caesar<T: Atom> {
    offset: usize,

    charset: Vec<T>,
    strict: bool,
}

impl<T: Atom> Cipher<T, T> for Caesar<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let c = SubstitutionCipherBuilder::default()
            .with_caesar(self.offset)
            .pt_alphabet(self.charset.to_vec())
            .strict(self.strict)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let c = SubstitutionCipherBuilder::default()
            .with_caesar(self.offset)
            .pt_alphabet(self.charset.to_vec())
            .strict(self.strict)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}
