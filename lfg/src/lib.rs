use derive_builder::Builder;
use num::{Integer, Unsigned};
use std::iter::{Product, Sum};

#[cfg(test)]
mod tests {
    use super::LFGBuilder;

    #[test]
    fn alfg_works() {
        struct TestCase {
            modulus: u32,
            seed: Vec<u32>,
            taps: Vec<usize>,
            expect: Vec<u32>,
        }

        let rows = &[
            TestCase {
                modulus: 1000,
                seed: vec![1, 1],
                taps: vec![1, 2],
                expect: vec![
                    2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 597, 584, 181, 765,
                    946, 711,
                ],
            },
            TestCase {
                modulus: 10,
                seed: vec![4, 8, 9, 4, 6, 0, 8],
                taps: vec![1, 2],
                expect: vec![2, 7, 3, 0, 6, 8, 0, 9, 0, 3, 6, 4, 8, 9, 9, 3, 9, 0, 2, 7],
            },
            TestCase {
                modulus: 10,
                seed: vec![4, 8, 9, 4, 6, 0, 8],
                taps: vec![3, 7],
                expect: vec![7, 1, 7, 7, 5, 2, 3, 0, 7, 2, 4, 7, 7, 4, 6, 0, 7, 4, 8, 4],
            },
        ];

        for r in rows {
            let g = LFGBuilder::default()
                .additive()
                .modulus(r.modulus)
                .seed(r.seed.to_owned())
                .taps(r.taps.to_owned())
                .build()
                .unwrap();
            let out: Vec<_> = g.take(r.expect.len()).collect();
            assert_eq!(&r.expect, &out);
        }
    }

    #[test]
    fn mlfg_works() {
        struct TestCase {
            modulus: u32,
            seed: Vec<u32>,
            taps: Vec<usize>,
            expect: Vec<u32>,
        }
        let rows = &[
            TestCase {
                modulus: 10,
                seed: vec![1, 1],
                taps: vec![1, 2],
                expect: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            },
            TestCase {
                modulus: 10,
                seed: vec![7, 7, 7, 9, 3, 1, 1],
                taps: vec![1, 2],
                expect: vec![9, 9, 3, 7, 3, 1, 9, 1, 7, 1, 1, 3, 9, 9, 7, 7, 1, 3, 7, 1],
            },
            TestCase {
                modulus: 10,
                seed: vec![7, 7, 7, 9, 3, 1, 1],
                taps: vec![3, 7],
                expect: vec![7, 3, 9, 9, 9, 3, 9, 1, 9, 1, 3, 7, 7, 3, 3, 9, 3, 1, 3, 9],
            },
        ];

        for r in rows {
            let g = LFGBuilder::default()
                .multiplicative()
                .modulus(r.modulus)
                .seed(r.seed.to_owned())
                .taps(r.taps.to_owned())
                .build()
                .unwrap();
            let out: Vec<_> = g.take(r.expect.len()).collect();
            assert_eq!(&r.expect, &out);
        }
    }
}

/// An LFGKind denotes a type of LFG and implements a strategy pattern to supply differentiated logic.
#[derive(Copy, Clone)]
enum LFGKind {
    Additive,
    Multiplicative,
}

impl LFGKind {
    fn run<T>(&self, xs: &[T]) -> T
    where
        T: Copy + Unsigned + Integer + Sum + Product,
    {
        match self {
            Self::Additive => xs.iter().cloned().sum(),
            Self::Multiplicative => xs.iter().cloned().product(),
        }
    }
}

impl<T> LFGBuilder<T>
where
    T: Copy + Unsigned + Integer + Product + Sum,
{
    /// Build a new LFG.
    // pub fn build(&self) -> Result<LFG<T>, String> {
    //     if let Err(s) = self.kind.check(&self.seed) {
    //         Err(format!("problem: {}", s))
    //     } else {
    //         Ok(LFG {
    //             kind: self.kind,
    //             lag_table: self.seed.to_owned(),
    //             modulus: self.modulus,
    //             taps: self.taps.to_owned(),
    //         })
    //     }
    // }

    fn validate(&self) -> Result<(), String> {
        match self.kind {
            Some(Some(LFGKind::Additive)) => {
                if self.seed.as_ref().unwrap().iter().all(|x| x.is_even()) {
                    Err("an ALFG requires at least one odd seed value".to_string())
                } else {
                    Ok(())
                }
            }
            Some(Some(LFGKind::Multiplicative)) => {
                if self.seed.as_ref().unwrap().iter().any(|x| x.is_even()) {
                    Err("an MLFG requires only odd seed values".to_string())
                } else {
                    Ok(())
                }
            }
            _ => Err("an LFG must be either additive or multiplicative".to_string()),
        }
    }

    /// Additive LFG.
    pub fn additive(&mut self) -> &mut Self {
        self.kind(Some(LFGKind::Additive));
        self
    }

    /// Multiplicative LFG.
    pub fn multiplicative(&mut self) -> &mut Self {
        self.kind(Some(LFGKind::Multiplicative));
        self
    }
}

/// A LFG is a lagged Fibonacci generator.
#[derive(Default, Builder, Clone)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct LFG<T>
where
    T: Unsigned + Integer + Copy + Product + Sum,
{
    modulus: T,

    #[builder(setter(into))]
    taps: Vec<usize>,

    #[builder(setter(into))]
    seed: Vec<T>,

    #[builder(private)]
    kind: Option<LFGKind>,
}

impl<T> LFG<T>
where
    T: Copy + Unsigned + Integer + Sum + Product,
{
    /// Lag table size.
    pub fn lag(&self) -> usize {
        self.seed.len()
    }
}

impl<T> Iterator for LFG<T>
where
    T: Copy + Unsigned + Integer + Sum + Product,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let tapped: Vec<_> = self.taps.iter().map(|t| self.seed[t - 1]).collect();
        let result = self.kind.unwrap().run(&tapped) % self.modulus;
        self.seed.push(result);
        self.seed.remove(0);
        Some(result)
    }
}
