use cipher::Cipher;
use derive_builder::Builder;
use transposition::Atom;
use transposition::ColumnarTranspositionCipherBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T: Atom> {
        turns: usize,

        input: Vec<T>,
        output: Vec<T>,
    }

    #[test]
    fn encipher_works() {
        let xs = &[
            TestCase {
                turns: 1,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                turns: 2,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 2, 4, 5, 3, 1, 1, 3, 5, 4, 2, 0],
            },
            TestCase {
                turns: 3,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 3, 5, 2, 1, 4, 4, 1, 2, 5, 3, 0],
            },
        ];
        for x in xs {
            let c = ScytaleBuilder::default().turns(x.turns).build().unwrap();
            assert_eq!(x.output, c.encipher(&x.input));
        }
    }

    #[test]
    fn decipher_works() {
        let xs = &[
            TestCase {
                turns: 1,
                input: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                turns: 2,
                input: vec![0, 2, 4, 5, 3, 1, 1, 3, 5, 4, 2, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
            TestCase {
                turns: 3,
                input: vec![0, 3, 5, 2, 1, 4, 4, 1, 2, 5, 3, 0],
                output: vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0],
            },
        ];
        for x in xs {
            let c = ScytaleBuilder::default().turns(x.turns).build().unwrap();
            assert_eq!(x.output, c.decipher(&x.input));
        }
    }
}

#[derive(Default, Builder)]
pub struct Scytale {
    turns: usize,
}

impl<T: Atom> Cipher<T, T> for Scytale {
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<T> {
        let c = ColumnarTranspositionCipherBuilder::with_scytale(self.turns)
            .myszkowski(true)
            .build()
            .unwrap();
        c.encipher(xs)
    }

    /// Decipher a sequence.
    fn decipher(&self, xs: &[T]) -> Vec<T> {
        let c = ColumnarTranspositionCipherBuilder::with_scytale(self.turns)
            .myszkowski(true)
            .build()
            .unwrap();
        c.decipher(xs)
    }
}

struct VIC {
    phrase: String,
    date: Vec<u32>,
    personal_number: u32,
    keygroup: Vec<u32>,
}

impl VIC {
    // pub fn to_num(&self, xs: &[u32]) -> Option<u32> {
    //     xs.to_vec().into_iter().reduce(|a, b| 10 * a + b)
    // }

    fn last_two_nonequal<T: Eq + Hash + Copy>(&self, xs: &[T]) -> Vec<T> {
        let ys: Vec<_> = xs.iter().rev().unique().take(2).copied().collect();
        ys.into_iter().rev().collect()
    }

    fn resequence(&self, xs: &[u32], ys: &[u32]) -> Vec<u32> {
        let m: HashMap<_, _> = (0u32..=9).map(|i| (i + 1) % 10).zip(xs.iter()).collect();
        ys.iter().map(|i| m[i]).copied().collect()
    }

    fn resequence2(&self, xs: &[u32], ys: &[u32]) -> Vec<u32> {
        let m: HashMap<_, _> = (0u32..=9).map(|i| i).zip(xs.iter()).collect();
        ys.iter().map(|i| m[i]).copied().collect()
    }

    pub fn keygen(&self) {
        let lineA = &self.keygroup;
        let lineB = &self.date[0..5];
        let lineC: Vec<_> = lineA
            .iter()
            .zip(lineB.iter())
            .map(|(a, b)| if a > b { a - b } else { 10 + a - b })
            .collect();
        let lineD0: Vec<_> = self
            .phrase
            .to_ascii_uppercase()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        let lineD = &lineD0[0..20];
        let lineD2 = &lineD[0..10];
        let lineD3 = &lineD[10..20];
        let lineE1: Vec<_> =
            transposition::transform::lexical_order(&std_ext::argsort(&std_ext::argsort(lineD2)))
                .iter()
                .map(|i| ((i + 1) % 10) as u32)
                .collect();
        let lineE2: Vec<u32> =
            transposition::transform::lexical_order(&std_ext::argsort(&std_ext::argsort(lineD3)))
                .iter()
                .map(|i| ((i + 1) % 10) as u32)
                .collect();
        let lineF1: Vec<_> = lineC
            .into_iter()
            .chain(
                LFGBuilder::default()
                    .additive()
                    .modulus(10)
                    .seed(lineC)
                    .taps(vec![1, 2])
                    .build()
                    .unwrap()
                    .take(5),
            )
            .collect();
        let lineF2: Vec<_> = (0u32..=9).map(|i| (i + 1) % 10).collect();
        // primer.iter().copied().chain(g).take(count).collect()

        // let lineE2Encoder: std::collections::HashMap<_, _> =
        //     lineF2.iter().zip(lineE2.iter()).collect();

        let lineG: Vec<_> = lineE1
            .iter()
            .zip(lineF1.iter())
            .map(|(a, b)| if a + b < 10 { a + b } else { a + b - 10 })
            .collect();
        // let lineH: Vec<_> = lineG.iter().map(|i| lineE2Encoder[i]).collect();
        let lineH: Vec<_> = self.resequence(&lineE2, &lineG);
        let lineJ: Vec<_> = std_ext::argsort(&std_ext::argsort(&lineH))
            .iter()
            .map(|i| (i + 1) % 10)
            .collect();
        let lineKLMNPseed: Vec<_> = lineH.iter().map(|&i| i as u32).collect();
        let lineKLMNP: Vec<_> = LFGBuilder::default()
            .additive()
            .modulus(10)
            .seed(lineKLMNPseed)
            .taps(vec![1, 2])
            .build()
            .unwrap()
            .take(50)
            .collect();
        let digits = self.last_two_nonequal(&lineKLMNP);

        println!("{:?} - {:?} = {:?}", lineA, lineB, lineC);
        println!("lineD = {:?}", lineD);
        println!("lineD = {:?} {:?}", lineD2, lineD3);
        println!("lineE = {:?} {:?}", lineE1, lineE2);
        println!("lineF = {:?} {:?}", lineF1, lineF2);
        println!("lineG = {:?}", lineG);
        println!("lineH = {:?}", lineH);
        println!("lineJ = {:?}", lineJ);
        println!("lineKLMNP = {:?}", lineKLMNP);
        println!("last_two_nonequal = {:?}", digits);
        let lens: Vec<usize> = digits
            .iter()
            .copied()
            .map(|i| (i + self.personal_number) as usize)
            .collect();

        // let revisedLineJ: Vec<u32> = lineJ.iter().copied().map(|i| i as u32).collect();
        let revisedLineJ: Vec<_> = std_ext::argsort(&std_ext::argsort(&lineH));
        println!("revised line j = {:?}", revisedLineJ);
        let tc = transposition::ColumnarTranspositionCipherBuilder::with_generic_key(&revisedLineJ)
            // .myszkowski(true)
            .build()
            .unwrap();
        let preQRout = tc.encipher(&lineKLMNP);
        let lineQ = &preQRout[0..lens[0]];
        let lineR = &preQRout[lens[0]..lens[0] + lens[1]];

        println!("lineQ = {:?}", lineQ);
        println!("lineR = {:?}", lineR);

        let lineP: Vec<_> = lineKLMNP.iter().rev().take(10).rev().collect();
        let lineS: Vec<_> = std_ext::argsort(&std_ext::argsort(&lineP))
            .iter()
            .map(|&i| if i == 0 { 9 } else { i - 1 })
            .collect();
        println!("lineS = {:?}", lineS);

        let mnemonic: Vec<_> = "AT ONE SIR".chars().collect();
        let alphabet: Vec<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ./".chars().collect();
        let stradalpha = masc::transform::keyword(&alphabet, &mnemonic);
        println!("strad alphabet = {:?}", stradalpha);
    }
}
