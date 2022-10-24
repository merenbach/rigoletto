use cipher::Cipher;
use transposition::transform;
use transposition::{Atom, ColumnarTranspositionCipherBuilder};

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        rails: usize,

        input: Vec<T>,
        output: Vec<T>,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                rails: 1,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                rails: 2,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 2, 4, 5, 3, 1, 1, 3, 5, 4, 2, 0],
            },
            TestCase {
                rails: 3,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 4, 3, 1, 3, 5, 4, 2, 0, 2, 5, 1],
            },
        ];
        for x in xs {
            let c = make(x.rails);
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                rails: 1,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                rails: 2,
                input: vec![0, 2, 4, 5, 3, 1, 1, 3, 5, 4, 2, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                rails: 3,
                input: vec![0, 4, 3, 1, 3, 5, 4, 2, 0, 2, 5, 1],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
        ];
        for x in xs {
            let c = make(x.rails);
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

/// Make a transposition cipher.
pub fn make<T: Atom>(rails: usize) -> impl Cipher<T, T> {
    let ys = match rails {
        1 => vec![0],
        _ => {
            let period = 2 * (rails - 1);
            transform::zigzag(period)
        }
    };

    // Prepare a rail fence cipher.
    // N.b.: The rail fence cipher is a special case of a columnar transposition cipher
    //       with Myszkowski transposition and a key equal to a zigzag sequence
    //       that converts the row count into the appropriate period.
    ColumnarTranspositionCipherBuilder::default()
        .key(ys)
        .myszkowski(true)
        .build()
        .unwrap()
}
