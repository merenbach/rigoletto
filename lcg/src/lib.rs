use derive_builder::Builder;
use num::integer::gcd;
use num::{CheckedAdd, CheckedMul, Integer, Unsigned};
use std::iter::{repeat, successors, Sum};
use std_ext::coprime;

#[cfg(test)]
mod tests {
    use super::{hull_dobell, regular, LCGBuilder};
    use num::{Integer, Unsigned};
    use std::iter::Sum;

    #[test]
    fn lcg_works() {
        #[derive(Debug)]
        struct TestCase<T>
        where
            T: Integer + Unsigned + Sum + Copy,
        {
            expected: Vec<T>,
            m: T,
            a: T,
            c: T,
            x0: T,
        }

        let rows: &[TestCase<u32>] = &[
            TestCase {
                expected: vec![13, 41, 21, 17, 29, 57, 37, 33, 45, 9, 53, 49, 61, 25, 5, 1],
                m: 64,
                a: 13,
                c: 0,
                x0: 1,
            },
            // TestCase {
            //     expected: vec![26, 18, 42, 34, 58, 50, 10, 2],
            //     m: 64,
            //     a: 13,
            //     c: 0,
            //     x0: 2,
            // },
            TestCase {
                expected: vec![39, 59, 63, 51, 23, 43, 47, 35, 7, 27, 31, 19, 55, 11, 15, 3],
                m: 64,
                a: 13,
                c: 0,
                x0: 3,
            },
            // TestCase {
            //     expected: vec![52, 36, 20, 4],
            //     m: 64,
            //     a: 13,
            //     c: 0,
            //     x0: 4,
            // },
        ];

        for r in rows {
            println!("row = {:?}", r);
            let out: Vec<_> = LCGBuilder::default()
                .modulus(r.m)
                .multiplier(r.a)
                .increment(r.c)
                .seed(r.x0)
                .build()
                .unwrap()
                .take(r.expected.len())
                .collect();
            assert_eq!(&r.expected, &out);
        }
    }

    #[test]
    fn regular_works() {
        let rows = &[
            (false, 1, 2),
            (false, 132, 168),
            (false, 168, 132),
            (false, 7, 98),
            (false, 98, 168),
            (true, 0, 1),
            (true, 0, 3),
            (true, 1, 1),
            (true, 168, 98),
            (true, 18, 12),
            (true, 24, 6),
            (true, 49, 7),
            (true, 6, 24),
            (true, 7, 49),
            (true, 70, 1),
            (true, 98, 7),
        ];

        for (expected, x, y) in rows {
            assert_eq!(*expected, regular(*x, *y));
        }

        // Check base-60 regular numbers for comparison.
        let xs = &[
            1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 25, 27, 30, 32, 36, 40, 45, 48, 50,
            54, 60,
        ];
        let ys: Vec<_> = (1..=60).filter(|&i| regular(60, i)).collect();
        assert_eq!(xs, &ys.as_ref());
    }

    #[test]
    fn hull_dobell_works() {
        let rows = &[
            // (true, 0, 1, 0, 3),
            (true, 1, 1, 0, 3),
            (true, 2, 1, 0, 3),
            (true, 26, 1, 7, 3),
            (true, 26, 1, 3, 3),
            (true, 26, 1, 1, 3),
            (false, 26, 1, 13, 3),
            (false, 26, 2, 7, 3),
            (false, 28, 3, 7, 3),
        ];

        for (expected, m, a, c, x_0) in rows {
            println!("testing: ({}, {}, {}, {})", m, a, c, x_0);
            if *expected {
                assert_eq!(Ok(()), hull_dobell::<u32>(*m, *a, *c, *x_0));
            } else {
                assert_ne!(Ok(()), hull_dobell::<u32>(*m, *a, *c, *x_0));
            }
        }
    }
}

// Hull-Dobell theorem to test for maximal LCG period.
// When `c` != 0, this test passing means that the cycle length equals `a`.
fn hull_dobell<T>(m: T, a: T, c: T, x_0: T) -> Result<(), String>
where
    T: Copy + Integer + Unsigned + Sum,
{
    let four: T = repeat(num::one()).take(4).sum();
    let a_minus_one = a - num::one();

    if c.is_zero() {
        if !coprime(m, x_0) {
            Err("m and x_0 must be relatively prime".to_string())
        } else {
            Ok(())
        }
    } else if !coprime(m, c) {
        Err("m and c must be relatively prime".to_string())
    } else if !regular(a_minus_one, m) {
        Err("a-1 must be divisible by all prime factors of m".to_string())
    } else if m.is_multiple_of(&four) && !a_minus_one.is_multiple_of(&four) {
        Err("a-1 must be divisible by 4 because m is divisible by 4".to_string())
    } else {
        Ok(())
    }
}

/// Regular tests if all prime factors of `n` also divide `base`.
/// Regular numbers "are those whose prime factors divide the base." (Source: https://www.britannica.com/science/regular-number)
/// A common place to see these used is the Babylonian numeral system, wherein the base is 60.
/// Note that the order of the parameters is important, as `base` may have additional prime factors.
fn regular<T: Integer + Copy>(base: T, n: T) -> bool {
    // [TODO] just return true if either a or b is zero?
    if n.is_zero() {
        panic!("Parameter `n` must be nonzero.");
    }

    successors(Some(n), |&x| Some(x / gcd(base, x)))
        .skip_while(|&x| !gcd(base, x).is_one())
        .take(1)
        .next()
        .unwrap()
        .is_one()
}

impl<T> LCGBuilder<T>
where
    T: Copy + Integer + Unsigned + Sum,
{
    // Validate LCG for compliance with the Hull-Dobell theorem.
    // The error parameter, if set, will contain the first-found failing constraint.
    fn validate(&self) -> Result<(), String> {
        let (modulus, multiplier, increment, seed) = (
            self.modulus.unwrap(),
            self.multiplier.unwrap(),
            self.increment.unwrap(),
            self.seed.unwrap(),
        );

        if modulus <= num::zero() {
            Err("modulus must be greater than 0".to_string())
        } else if multiplier <= num::zero() || multiplier >= modulus {
            Err("multiplier must be greater than 0 and less than modulus".to_string())
        } else if increment < num::zero() || seed >= modulus {
            Err("increment must be greater than or equal to 0 and less than modulus".to_string())
        } else if seed < num::zero() || seed >= modulus {
            Err("seed must be greater than or equal to 0 and less than modulus".to_string())
        } else {
            hull_dobell(modulus, multiplier, increment, seed)
        }
    }
}

#[derive(Default, Builder, Clone)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct LCG<T>
where
    T: Copy + Integer + Unsigned + Sum,
{
    modulus: T,    // m
    multiplier: T, // a
    increment: T,  // c
    seed: T,       // x_0

    #[builder(setter(skip))]
    cursor: Option<T>,
}

impl<T> LCG<T>
where
    T: Copy + Integer + Unsigned + Sum,
{
    /// Mixed LCGs have a non-zero increment.
    /// A non-mixed LCG may be called a Lehmer RNG, a Park-Miller RNG,
    /// a multiplicative LCG (MLCG), or a multiplicative congruential generator (MCG).
    pub fn mixed(&self) -> bool {
        !self.increment.is_zero()
    }
}

impl<T> Iterator for LCG<T>
where
    T: Copy + Integer + Unsigned + Sum + CheckedMul + CheckedAdd,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cursor = self.cursor.unwrap_or(self.seed);
        self.cursor = Some(
            self.multiplier
                .checked_mul(&cursor)?
                .checked_add(&self.increment)?
                .mod_floor(&self.modulus),
        );
        self.cursor
    }
}
