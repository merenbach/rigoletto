use cipher::Cipher;
use derive_builder::Builder;
use masc::Atom;
use masc::SubstitutionCipherBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        ct_alphabet: Vec<Vec<T>>,

        pt_alphabet: Vec<T>,
        input: Vec<T>,
        output: Vec<T>,
        strict: bool,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                ct_alphabet: vec![vec![4], vec![5], vec![6], vec![7], vec![8]],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 0],
                strict: false,
            },
            TestCase {
                ct_alphabet: vec![vec![4], vec![5], vec![6], vec![7], vec![8]],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![4, 5, 6, 7, 8, 8, 7, 6, 5, 4],
                strict: true,
            },
        ];
        for x in xs {
            let c = HomophonicBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .ct_alphabet(x.ct_alphabet.to_vec())
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
                ct_alphabet: vec![vec![4], vec![5], vec![6], vec![7], vec![8]],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                strict: false,
            },
            TestCase {
                ct_alphabet: vec![vec![4], vec![5], vec![6], vec![7], vec![8]],
                pt_alphabet: vec![1, 2, 3, 4, 5],
                input: vec![0, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 0],
                output: vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
                strict: true,
            },
        ];
        for x in xs {
            let c = HomophonicBuilder::default()
                .pt_alphabet(x.pt_alphabet.to_vec())
                .ct_alphabet(x.ct_alphabet.to_vec())
                .strict(x.strict)
                .build()
                .unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

#[derive(Default, Builder)]
pub struct Homophonic<T: Atom> {
    ct_alphabet: Vec<Vec<T>>,

    pt_alphabet: Vec<T>,
    strict: bool,
}

impl<T: Atom> Cipher<T, T> for Homophonic<T> {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let ct_alphabet = self.ct_alphabet.to_vec();
        let m: std::collections::HashMap<_, Vec<T>> =
            self.pt_alphabet.iter().copied().zip(ct_alphabet).collect();
        xs.iter()
            .filter_map(|x| match m.get(x) {
                Some(v) => Some(v[0]),
                None => {
                    if self.strict {
                        None
                    } else {
                        Some(*x)
                    }
                }
            })
            .collect()
        // let c = SubstitutionCipherBuilder::default()
        //     .pt_alphabet(self.pt_alphabet.to_vec())
        //     .ct_alphabet(self.ct_alphabet.to_vec())
        //     .strict(self.strict)
        //     .build()
        //     .unwrap();
        // c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let ct_alphabet = self.ct_alphabet.to_vec();
        let m: std::collections::HashMap<_, _> = self
            .pt_alphabet
            .iter()
            .copied()
            .zip(ct_alphabet)
            .flat_map(|t| t.1.into_iter().map(move |c| (c, t.0)))
            .collect();
        xs.iter()
            .filter_map(|x| match m.get(x) {
                Some(v) => Some(*v),
                None => {
                    if self.strict {
                        None
                    } else {
                        Some(*x)
                    }
                }
            })
            .collect()
        // let c = SubstitutionCipherBuilder::default()
        //     .pt_alphabet(self.pt_alphabet.to_vec())
        //     .ct_alphabet(self.ct_alphabet.to_vec())
        //     .strict(self.strict)
        //     .build()
        //     .unwrap();
        // c.decipher(xs)
    }
}
