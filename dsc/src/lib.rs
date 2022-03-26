use cipher::Cipher;
use derive_builder::Builder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left_shift_works() {
        let inp: Vec<_> = "HXUCZVAMDSLKPEFJRIGTWOBNYQ".chars().collect();
        let out: Vec<_> = "PFJRIGTWOBNYQEHXUCZVAMDSLK".chars().collect();
        assert_eq!(out, left_shift(&inp, 0, 13, 12));
    }

    #[test]
    fn right_shift_works() {
        let inp: Vec<_> = "PTLNBQDEOYSFAVZKGJRIHWXUMC".chars().collect();
        let out: Vec<_> = "VZGJRIHWXUMCPKTLNBQDEOYSFA".chars().collect();
        assert_eq!(out, right_shift(&inp, 0, 13, 12));
    }

    #[test]
    fn encipher_works() {
        // http://www.chaocipher.com/ActualChaocipher/Chaocipher-Revealed-Algorithm.pdf
        let msg: Vec<_> = "WELLDONEISBETTERTHANWELLSAID".chars().collect();
        let out: Vec<_> = "OAHQHCNYNXTSZJRRHJBYHQKSOUJY".chars().collect();

        let cipher = ChaocipherBuilder::default()
            .left("HXUCZVAMDSLKPEFJRIGTWOBNYQ".chars().collect())
            .right("PTLNBQDEOYSFAVZKGJRIHWXUMC".chars().collect())
            .build()
            .unwrap();

        assert_eq!(out, cipher.encipher(&msg));
    }

    #[test]
    fn decipher_works() {
        // http://www.chaocipher.com/ActualChaocipher/Chaocipher-Revealed-Algorithm.pdf
        let msg: Vec<_> = "OAHQHCNYNXTSZJRRHJBYHQKSOUJY".chars().collect();
        let out: Vec<_> = "WELLDONEISBETTERTHANWELLSAID".chars().collect();

        let cipher = ChaocipherBuilder::default()
            .left("HXUCZVAMDSLKPEFJRIGTWOBNYQ".chars().collect())
            .right("PTLNBQDEOYSFAVZKGJRIHWXUMC".chars().collect())
            .build()
            .unwrap();

        assert_eq!(out, cipher.decipher(&msg));
    }
}


// TODO: can use rotate, permute, and bringToZenith like in http://www.chaocipher.com/ActualChaocipher/Chaocipher-Revealed-Algorithm.pdf
/*

> "It is perfectly logical to alternate between locating the plaintext letter in the right or left alphabet based on
some prearranged pattern. As will be shown in a future paper, Byrne used this alternating alphabet method
for deriving the starting alphabets."

--- this means we might have an alternative encryption mode available.

*/


fn my_shift<T>(
    xs: &[T],
    offset: usize,
    zenith: usize,
    nadir: usize,
    element_idx_to_move: usize,
) -> Vec<T>
where
    T: Copy,
{
    let mut ys = xs.to_vec();

    ys.rotate_left(element_idx_to_move + offset - 1); // TODO: what if zenith is later in the alphabet?
    let z = ys.remove(zenith + offset);
    ys.insert(nadir, z);
    ys
}

fn left_shift<T>(xs: &[T], zenith: usize, nadir: usize, element_idx_to_move: usize) -> Vec<T>
where
    T: Copy,
{
    my_shift(xs, 1, zenith, nadir, element_idx_to_move)
}

fn right_shift<T>(xs: &[T], zenith: usize, nadir: usize, element_idx_to_move: usize) -> Vec<T>
where
    T: Copy,
{
    my_shift(&xs, 2, zenith, nadir, element_idx_to_move)
}

// Generic Chaocipher alphabet permutation (i.e., Nick Pelling's "twizzling")
fn permute<T>(xs: &[T], offset: usize, zenith: usize, nadir: usize) -> Vec<T>
where
    T: Copy,
{
    let mut ys = xs.to_vec();
    let z = ys.remove(zenith + offset);
    ys.insert(nadir, z);
    ys
}

#[derive(Builder, Default)]
#[builder(default)]
struct Chaocipher<T>
where
    T: Copy + Default,
{
    #[builder(default = "0")]
    zenith: usize,
    #[builder(default = "13")]
    nadir: usize,

    strict: bool,

    right: Vec<T>, // for finding plaintext letters on encipherment
    left: Vec<T>,  // for finding ciphertext letters on decipherment
}

impl<T> Cipher<T> for Chaocipher<T>
where
    T: Copy + Default + Eq,
{
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let mut left = self.left.to_vec();
        let mut right = self.right.to_vec();
        let mut output = vec![];

        for x in xs {
            let idx = right.iter().position(|y| y == x);
            if let Some(idx_of_c_in_right) = idx {
                let new_x = left[idx_of_c_in_right];
                left.rotate_left(idx_of_c_in_right);

                right.rotate_left(idx_of_c_in_right);

                left = permute(&left, 1, self.zenith, self.nadir);
                right.rotate_left(1);
                right = permute(&right, 2, self.zenith, self.nadir);

                output.push(new_x);
            } else if !self.strict {
                output.push(*x)
            }
        }

        output
    }

    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let mut left = self.left.to_vec();
        let mut right = self.right.to_vec();
        let mut output = vec![];

        for x in xs {
            let idx = left.iter().position(|y| y == x);
            if let Some(idx_of_c_in_left) = idx {
                let new_x = right[idx_of_c_in_left];
                right.rotate_left(idx_of_c_in_left);

                left.rotate_left(idx_of_c_in_left);

                left = permute(&left, 1, self.zenith, self.nadir);
                right.rotate_left(1);
                right = permute(&right, 2, self.zenith, self.nadir);

                output.push(new_x);
            } else if !self.strict {
                output.push(*x);
            }
        }

        output
    }
}
