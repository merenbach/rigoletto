use cipher::Cipher;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Copy> {
        input: Vec<T>,
        output: Vec<T>,
    }

    #[test]
    fn encipher_works() {
        let xs = &[TestCase {
            input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
        }];
        for x in xs {
            let c = Dummy {};
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[TestCase {
            input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
        }];
        for x in xs {
            let c = Dummy {};
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

pub struct Dummy {}

impl<T: Copy> Cipher<T, T> for Dummy {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        xs.into()
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        xs.into()
    }
}
