pub mod transform;

use derive_builder::Builder;
use std::cmp;
use std::hash::Hash;
use std_ext::argsort;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// pub fn dsu<T, F, K>(xs: &[T], f: F) -> Vec<usize>
// where
//     T: Ord + Copy,
//     F: Fn(&T) -> K,
//     K: Ord,
// {
//     let mut ys: Vec<_> = xs.iter().enumerate().collect();
//     ys.sort_by_key(|k| f(k.1));
//     ys.iter().map(|t| t.0).collect()
// }

// /// Decorate-sort-undecorate (Schwartzian transform) based on index.
// pub fn dsu<T, F, K>(xs: &[T], f: F) -> Vec<T>
// where
//     T: Ord + Copy,
//     F: Fn(usize) -> K,
//     K: Ord,
// {
//     let mut ys: Vec<_> = xs.iter().enumerate().collect();
//     ys.sort_by_key(|k| f(k.0));
//     ys.iter().map(|t| t.1).copied().collect()
// }

// Zigzag sequence for use in the rail fence cipher.
fn zigzag(period: usize) -> Vec<usize> {
    match period {
        0 => vec![0],
        _ => (0..period)
            .map(|i| i % period)
            .map(|n| cmp::min(n, period - n))
            .collect(),
    }
}

pub trait Atom: Copy + Default {}
impl<T> Atom for T where T: Copy + Default {}

impl<T: Atom> ColumnarTranspositionCipherBuilder<T> {
    /// Add a key.
    pub fn with_generic_key(v: &[impl Ord + Hash]) -> Self {
        // This lexical ordering transformation is technically needed only to support Myszkowski transposition.
        // We don't know the message length at this point, so we want to avoid doing any argsorts until later,
        // since argsort will convert duplicate values into consecutive values.
        let data = transform::lexical_order(&v);
        Self {
            key: Some(data),
            ..Default::default()
        }
    }

    /// Add a string-based key.
    pub fn with_str_key(v: &str) -> Self {
        let xs: Vec<_> = v.chars().collect();
        Self::with_generic_key(&xs)
    }

    /// Prepare a rail fence cipher.
    /// N.b.: The rail fence cipher is a special case of a columnar transposition cipher
    ///       with Myszkowski transposition and a key equal to a zigzag sequence
    ///       that converts the row count into the appropriate period.
    pub fn with_rail_fence(rows: usize) -> Self {
        let period = 2 * (rows - 1);
        Self::with_generic_key(&zigzag(period))
    }

    /// Prepare a scytale cipher.
    /// N.b.: The scytale cipher is a special case of a columnar transposition cipher
    ///       with a key equal to an ascending consecutive integer sequence
    ///       as long as the number of turns.
    ///       A sequence with all the same digit may also work, but may depend on a stable sort.
    pub fn with_scytale(turns: usize) -> Self {
        let seq: Vec<_> = (0..turns).collect();
        Self::with_generic_key(&seq)
    }
}

/// A Columnar transposition cipher.
/// TODO: should keys be split out into use of multiple ciphers?
#[derive(Default, Builder, Clone)]
#[builder(default)]
pub struct ColumnarTranspositionCipher<T: Atom> {
    #[builder(private)]
    key: Vec<usize>,

    nulls: Vec<T>,

    #[doc = r"`myszkowski` determines whether to perform Myszkowski transposition."]
    myszkowski: bool,
}

impl<T: Atom> ColumnarTranspositionCipher<T> {
    // TODO: add more tests to ensure that Myszkowski argument maybe always has an effect
    /// Generate transposition cipher indices based on a columnar key.
    fn process(&self, count: usize) -> Vec<usize> {
        let v: Vec<_> = if self.myszkowski {
            self.key.iter().cycle().take(count).copied().collect()
        } else {
            self.key.to_vec()
        };

        let u = argsort(&argsort(&v));
        if self.myszkowski {
            u
        } else {
            u.iter().cycle().take(count).copied().collect()
        }
    }

    /// Encipher a message.
    pub fn encipher(&self, xs: &[T]) -> Vec<T> {
        let ys: Vec<_> = xs.iter().chain(self.nulls.iter()).copied().collect();
        let indices = self.process(ys.len());
        transform::transpose(&ys, &indices)
    }

    /// Decipher a message.
    pub fn decipher(&self, xs: &[T]) -> Vec<T> {
        let indices = self.process(xs.len());
        // TODO: this doesn't verify that the nulls are again present at the end
        transform::transpose(&xs, &argsort(&indices))
            .into_iter()
            .take(xs.len() - self.nulls.len())
            .collect()
    }
}

// enum Corner {
//     TopLeft,
//     TopRight,
//     BottomLeft,
//     BottomRight,
// }

// impl Corner {
//     pub fn run(&self, cellcount: usize, cols: usize) -> usize {
//         match &self {
//             TopLeft => 0,
//             TopRight => cols - 1,
//             BottomLeft => cellcount - cols - 1,
//             BottomRight => cellcount - 1,
//         }
//     }
// }

// impl Default for Corner {
//     fn default() -> Self {
//         Self::TopLeft
//     }
// }

// #[derive(Default)]
// pub struct SpiralTranspositionCipherBuilder {
//     start_corner: Corner,
//     clockwise: bool,
//     cols: usize,
// }

// impl SpiralTranspositionCipherBuilder {
//     /// Build a cipher.
//     pub fn build(&self) -> SpiralTranspositionCipher {
//         SpiralTranspositionCipher {
//             start_corner: self.start_corner,
//             clockwise: self.clockwise,
//             cols: self.cols,
//         }
//     }

//     fn corner(&mut self, corner: Corner) -> &mut Self {
//         self.start_corner = corner;
//         self
//     }

//     pub fn columns(&mut self, v: usize) -> &mut Self {
//         self.cols = v;
//         self
//     }

//     pub fn start_top_left(&mut self) -> &mut Self {
//         self.corner(Corner::TopLeft)
//     }

//     pub fn start_top_right(&mut self) -> &mut Self {
//         self.corner(Corner::TopRight)
//     }

//     pub fn start_bottom_left(&mut self) -> &mut Self {
//         self.corner(Corner::BottomLeft)
//     }

//     pub fn start_bottom_right(&mut self) -> &mut Self {
//         self.corner(Corner::BottomRight)
//     }

//     pub fn clockwise(&mut self, b: bool) -> &mut Self {
//         self.clockwise = b;
//         self
//     }
// }

// /// A spiral transposition cipher.
// /// TODO: can we or should we merge with columnar somehow? should columnar be split to have only one key? is there any point in supporting multiple keys???
// pub struct SpiralTranspositionCipher {
//     start_corner: Corner,
//     clockwise: bool,
//     cols: usize,
// }

// impl Cipher for SpiralTranspositionCipher {
//     /// Encipher a message.
//     fn encipher(&self, xs: &[char]) -> Vec<char> {
//         let mut indices: Vec<_> = Vec::new();
//         let mut rows = xs.len() / self.cols;
//         if xs.len() % self.cols != 0 {
//             rows += 1;
//         }
//         let (mut col_l, mut col_r, mut row_l, mut row_r) = (0, self.cols - 1, 0, rows - 1);

//         let mut cur_row = self.start_corner.run_row(xs.len(), self.cols);
//         let mut cur_col = self.start_corner.run_col(xs.len(), self.cols);
//         while col_l < col_r && row_l < row_r {
//             if cur_col == self.cols - 1 {
//                 if cur_row ==
//             }

//             let square = cur_col + cur_row*self.cols;
//             indices.push(square);
//         }

//         transpose_transform(&xs, &indices, false)
//     }

//     /// Decipher a message.
//     fn decipher(&self, xs: &[char]) -> Vec<char> {
//         Vec::new()
//     }
// }

// pub fn rail_fence<T: Atom>(rows: usize) -> ColumnarTranspositionCipher<T> {
//     ColumnarTranspositionCipherBuilder::with_rail_fence(rows)
//         .myszkowski(true)
//         .build()
//         .unwrap()
// }

// pub fn scytale<T: Atom>(turns: usize) -> ColumnarTranspositionCipher<T> {
//     ColumnarTranspositionCipherBuilder::with_scytale(turns)
//         .myszkowski(false)
//         .build()
//         .unwrap()
// }
