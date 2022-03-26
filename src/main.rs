use cipher::Cipher;
use clap::Parser;
use derive_builder::Builder;
use itertools::Itertools;
use lfg::LFGBuilder;
use message::Message;
use nalgebra::{dmatrix, matrix};
use ndarray::{arr1, arr2, Array1, Array2};
use std::collections::HashMap;
use std::hash::Hash;
use transposition::{ColumnarTranspositionCipher, ColumnarTranspositionCipherBuilder};

// use masc::*;

// use num_traits::Num;
// use std::collections::HashMap;
// use std::convert::TryFrom;
// use std::fmt;

// fn mapints(hm: &HashMap<u32, u32>, nums: &[u32]) -> Vec<u32> {
//     let out: Vec<u32> = nums
//         .iter()
//         .map(|x| match hm.get(&(x - 1)) {
//             Some(v) => *v,
//             None => 0,
//         })
//         .collect();
//     out
// }

/*
let mut books = HashSet::new();

// Add some books.
books.insert("A Dance With Dragons".to_string());
books.insert("To Kill a Mockingbird".to_string());
books.insert("The Odyssey".to_string());
books.insert("The Great Gatsby".to_string());

// Check for a specific one.
if !books.contains("The Winds of Winter") {
    println!("We have {} books, but The Winds of Winter ain't one.",
             books.len());
}

// Remove a book.
books.remove("The Odyssey");

// Iterate over everything.
for book in &books {
    println!("{}", book);
}*/

// fn makemap(s1: &str, s2: &str) -> HashMap<char, char> {
//     s1.chars().zip(s2.chars()).collect()
// }

// use lcg::LCG;

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
            .to_vec()
            .into_iter()
            .chain(
                LFGBuilder::default()
                    .additive()
                    .modulus(10)
                    .seed(lineC.to_vec())
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

// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     /// PID file
//     #[clap(short, long)]
//     pidfile: Option<String>,
// }

#[derive(Builder)]
struct HillCipher<T>
where
    T: Copy + Hash + Eq,
{
    alphabet: Vec<T>,
    n: usize,
}

impl<T> HillCipher<T>
where
    T: Copy + Hash + Eq,
{
    pub fn encipher(&self, xs: &[T], k: &nalgebra::SMatrix<usize, 3, 3>) -> Vec<T> {
        let h: HashMap<_, usize> = self
            .alphabet
            .iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect();
        let ys1: Vec<usize> = xs.iter().map(|x| h[x]).collect();
        let ys: nalgebra::SMatrix<usize, 3, 1> = nalgebra::SMatrix::from_vec(ys1);

        let out = k * ys;
        out.iter()
            .map(|&i| self.alphabet[i % self.alphabet.len()])
            .collect()
    }

    // pub fn decipher(&self, xs: &[T], k: &nalgebra::Matrix3<usize>) -> Vec<T> {
    //     self.encipher(xs, k.inverse())
    // }
}

fn main() {
    // let args = Args::parse();
    run_app()
}

fn run_app() {
    {
        let k = matrix![6, 24, 1; 13, 16, 10; 20, 17, 15];
        // let msg = dmatrix!(&[0, 2, 19]);

        let ab: Vec<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let x = HillCipherBuilder::default()
            .alphabet(ab)
            .n(3)
            .build()
            .unwrap();
        let m: Vec<_> = "ACT".chars().collect();
        let y = x.encipher(&m, &k);
        println!("y = {:?}", y);

        // let z = x.decipher(&y, &k);
        // println!("z = {:?}", z);
    }
    {
        let msg = "Attack at dawn. By dawn I mean 0500. Not 0915 like you did last time.";
        let d = &[1, 3, 9, 1, 9, 5];
        let kg = &[7, 2, 4, 0, 1];
        let v = VIC {
            phrase: "Twas the night before Christmas".to_string(),
            date: d.to_vec(),
            personal_number: 6,
            keygroup: kg.to_vec(),
        };

        v.keygen();
    }

    {
        // let c = masc::SubstitutionCipherBuilder::with_caesar(3).build().unwrap();
        // let mut msg = Message::new("HELLO WORLD");
        // msg.encipher(&c);
        // println!("enc = {}", msg);
        // msg.decipher(&c);
        // println!("dec = {}", msg);

        // let period = 6;
        // let counter = std::iter::from_fn(move || {
        //     // Increment our count. This is why we started at zero.
        //     count += 1;
        //     // Check to see if we've finished counting or not.
        //     if count < 6 {
        //         Some(count)
        //     } else {
        //         None
        //     }
        // });

        // let period = 6;
        // let z: Vec<_> = (0..)
        //     .map(|i| i % period)
        //     .map(|n| std::cmp::min(n, period - n))
        //     .take(20)
        //     .collect();
        // println!("hey: {:?}", z);
    }
    // {
    //     // let x = masc::caesar(3); // TODO: should be this simple

    //     let c = masc::CipherBuilder::with_keyword("KANGAROO").build();
    //     let out = c.encipher2(&['H', 'E', 'L', 'L', 'O', 'w', 'o', 'r', 'l', 'D', '!']);
    //     let out2 = masc::filterify_lenient(&out);

    //     println!("out = {:?}", out2);
    // }
    {
        // let c = transposition::Columnar::new(&['A', 'N', 'D', 'R', 'E', 'W'], false);
        // let c = transposition::Columnar::new(&['6', '3', '2', '4', '1', '5'], false);
        // let c = transposition::Columnar::new(&['Z', 'E', 'B', 'R', 'A', 'S'], false);
        // let mut msg = Message::new("WEAREDISCOVEREDFLEEATONCE");

        // let mut cb = transposition::ColumnarTranspositionCipherBuilder::default();
        // cb.myszkowski(true);
        //ROFOACDTEDSEEEACWEIVRLENE
        //ROFOACDTEDSEEEACWEIVRLENE
        //ROFOACDTEDSEEEACWEIVRLENE
        //ROFOACDTESEADEECWIREEEVLN
        // Looks like Myszkowski is approximated by repeating the key over and over.
        // cb.add_key(&['T', 'O', 'M', 'A', 'T', 'O']);
        // cb.add_key(&['Z', 'E', 'B', 'R', 'A', 'S']);
        // cb.add_str_key("4321434321434321434321434");
        // cb.add_str_key("TOMATOTOMATOTOMATOTOMATOT", true);
        // cb.add_str_key("ZEBRAS", true);
        // cb.add_rail_fence_key(3);
        // cb.add_str_key("TOMATO", true);
        // cb.add_str_key("STRIPE");
        // cb.add_str_key("AB");
        // cb.add_str_key("CD");
        //WECRLTERSEFACAIVDENEDOEEO AB,CD
        //WECRLTERSEFACAIVDENEDOEEO ADCB
        //
        // cb.add_str_key("EAC");
        // cb.add_str_key("FBD");
        //EENCEWOESLAETRROEVADDCIFE
        //
        // cb.add_str_key("EAC");
        // cb.add_str_key("STRIPEZEBRASSTRIPEZEBRASSTRIPEZEBRAS");
        // cb.add_str_key("TOMATOTOMATOTOMATOTOMATOT");
        // cb.add_str_key("ZEBRAS");
        // cb.add_str_key("WEAREDISCOVEREDFLEEATONCE");
        //EVLNACDTESEAROFODEECWIREE
        //EVLNACDTESEAROFODEECWIREE
        //EVLNACDTESEAROFODEECWIREE
        //EVLNACDTESEAROFODEECWIREE
        //
        //AACCDDEEEEEEEFILNOORRSTVW
        //AACCDDEEEEEEEFILNOORRSTVW
        // Beale cipher--letters in order because of columnar cipher?!
        // cb.add_str_key("ZEBRASZEBRASZEBRASZEBRASZ");
        // cb.add_str_key("1232123212321232123212321");
        // cb.add_str_key("1232").myszkowski(true)
        // cb.add_rail_fence_key(3);
        // cb.add_str_key("ZEBRAS");
        // let c = cb.build();
        // // let c = transposition::Columnar::new(&['T','T','T','T','O','O','O','O','M','M','M','M','A','A','A','A','T','T','T','T','O','O','O','O'], false);
        // // let c = transposition::Columnar::new(&['T','O','M','A','T','O','T','O','M','A','T','O','T','O','M','A','T','O','T','O','M','A','T','O','T'], false);
        // msg.encipher(&c);
        // println!("enc = {}", msg);
        // msg.decipher(&c);
        // println!("dec = {}", msg);
        // println!("m = {:?} -> {:?}", e, d);
    }
    // {
    //     let x = chainadder(10, 30, &[6, 0, 1, 2, 8]);
    //     println!("chainadder out = {:?}", x);
    // }
    // {
    //     let x = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    //     for i in 0..x.len() {
    //         let y = pasc::owrap(x, i);
    //         println!("y = {:?}", y);
    //     }

    //     let m: Vec<_> = "ANDREW".chars().collect();
    //     let z = std_ext::argsort(&m);
    //     println!("m = {:?}", std_ext::argsort(&z));

    //     let x1 = lexikey(&['A', 'N', 'D', 'R', 'E', 'E', 'W'], false);
    //     let x2 = lexikey(&['A', 'N', 'D', 'R', 'E', 'E', 'W'], true);
    //     println!("myout = {:?}, {:?}", x1, x2);
    // }
    // let x = masc::affine(7, 1);
    // x.strict = true;
    // x.caseless = true;
    // for i in x {
    //     println!("1. Hello, world! {}", i);
    // }

    // fn untransform<T>(xs: &[T], indices: &[usize]) -> Vec<T>
    // where
    //     T: Copy,
    // {
    //     let mut zippy: Vec<_> = xs.iter().zip(indices.iter()).collect();
    //     zippy.sort_by_key(|k| k.1);
    //     zippy.iter().map(|(&a, _)| a).collect()
    // }

    // fn inverse(xs: &[usize]) -> Vec<usize> {
    //     let mut ys: Vec<_> = xs.iter().enumerate().collect();
    //     ys.sort_by_key(|k| k.1);
    //     ys.iter().map(|t| t.0).collect()
    // }

    // fn inverse2<T>(xs: &[T]) -> Vec<usize>
    // where
    //     T: Ord,
    // {
    //     let mut ys: Vec<_> = xs.iter().enumerate().collect();
    //     ys.sort_by_key(|k| k.1);
    //     ys.iter().map(|t| t.0).collect()
    // }

    // fn untransform<T>(xs: &[T], indices: &[usize]) -> Vec<T>
    // where
    //     T: Copy,
    // {
    //     let idxs = std_ext::invert(indices);
    //     backpermute(xs, &idxs)
    //     // // let mut zippy: Vec<_> = xs.iter().zip(indices.iter()).collect();
    //     // let mut zippy: Vec<_> = indices.iter().enumerate().collect();
    //     // zippy.sort_by_key(|k| k.1);
    //     // let indices2: Vec<usize> = zippy.iter().map(|(a, _)| *a).collect();
    //     // println!("hello! indices2 = {:?}", indices2);
    //     // Vec::new()
    //     // indices2.iter().map(|&i| xs[i]).collect()
    // }

    // fn untransform2<T>(xs: &mut Vec<T>, indices: &[usize]) {
    //     // let mut zippy: Vec<_> = indices.iter().enumerate().collect();
    //     // zippy.sort_by_key(|k| k.1);
    //     // let mut xs = xs;
    //     // xs.reverse();
    //     // let xslen = xs.len()-1;
    //     // let v2 = indices.to_vec();
    //     // // xs.reverse();
    //     // for (i, &e) in indices.iter().enumerate() {
    //     //     // xs.swap(xslen-i, xslen-e);
    //     //     xs.swap(i, e);
    //     // }

    //     // let mappy:HashMap<&T, &usize> = xs.iter().zip(indices.iter()).collect();
    //     let mapping :Vec<_> = xs.iter().zip(indices.iter()).collect();
    //     xs.sort_by_key(|k| mapping[k]);
    // }

    // PROBLEM STATEMENT: now, given that we have the transformation "key" [2,0,5,1,4,3]
    //                    and, given that repeats may occur...
    //                    how, then, can we change DAWNER back to ANDREW?
    /*
        0 1 2 3 4 5                        0,1,2,3,4,5
        A N D R E W -> to DAWNER, use key [2,0,5,1,4,3]
        D A W N E R -> to ANDREW, use key [1,3,0,5,4,2]
                                           1+2 = 3, 3-3 = 0, 0+5=5, 5-1=4, 4-2=2


        Option 1: Zip DAWNER (i.e., transformed) with key 2,0,5,1,4,3 and sort:

        [(2,D), (0,A), (5,W), (1,N), (4,E), (3,R)] // zipped
        [(0,A), (1,N), (2,D), (3,R), (4,E), (W,5)] // sorted by first element

        Option 2: is there a way to determine 1,3,0,5,4,2 from 2,0,5,1,4,3?
    */

    // {
    //     let mut z = zigzag::Zigzag::new(18u32);
    //     for i in 0..50 {
    //         println!("next = {}", z.next().unwrap());
    //     }
    // }

    // let zz = untransform(&['D', 'A', 'W', 'N', 'E', 'R'], &[2, 0, 5, 1, 4, 3]);
    // println!("zz = {:?}", zz);

    // let mut a1 = vec!['D','A','W','N','R','E'];
    // let a2 = &[2,0,5,1,3,4];
    // untransform2(&mut a1,a2);
    // println!("newa1 = {:?}", a1);
    {
        // let idxs2 = [9,1,4,3,5,2,10,6,8];
        // let idxs2 = [4,2,1,3];
        // let xf = std_ext::invert(&idxs2);
        // let xf2 = std_ext::invert(&xf);
        // let xf3 = std_ext::invert(&xf2);
        // let xf4 = std_ext::invert(&xf3);
        // println!("1 = {:?}", &idxs2);
        // println!("2 = {:?}", &xf);
        // println!("3 = {:?}", &xf2);
        // println!("4 = {:?}", &xf3);
        // println!("5 = {:?}", &xf4);
    }

    // let ab = alphabet::LATIN;
    // let x = affine_transform(ab.chars().collect(), 25, 25);
    // let y: String = x.iter().collect();
    // println!("out = {}", y);

    // let z = backpermute(&[5, 6, 7, 8, 9, 10], &[3, 3, 1, 0, 5, 0, 5]);
    // let n = 40;
    // let z = &[5,6,7,8,9,10][n];
    // println!("out2 = {:?}", z);
    // for i in 0..28 {
    //     let ow = pasc::owrap(ab, i);
    //     println!("row {} = [{}]",i, ow.unwrap());
    // }

    // let conf = masc::Config{
    //     pt_alphabet:alphabet::LATIN.to_string(),
    //     strict:true,
    //     caseless:true,
    // };

    // {
    //     // let m1 = Message::from_str("HELLO WORLD");
    //     let mut msg = Message::new("HELLO, WORLD!");
    //     let c = masc::CipherBuilder::with_keyword("KANGAROO")
    //         .caseless(false)
    //         .strict(false)
    //         .build();
    //     println!("cipher = {}", c);
    //     println!("MSG = {}", msg);
    //     msg.encipher(&c);
    //     println!("ENC = {}", msg);
    //     msg.decipher(&c);
    //     // msg.retain_str("HLO ");
    //     // msg.expunge();
    //     // println!("msg = {}\nct  = {}\npt  = {}", &msg, &enc, &dec);
    //     println!("DEC = {}", msg);
    //     println!("^^^--- that was the caesar");

    //     // println!("doing something");
    //     // let enc2 = c.encipher_test(msg);
    //     // println!("here it is: {:?}", enc2);
    //     // // enc2.expunge();
    //     // println!("here it is: {:?}", enc2);
    //     // enc2.iter().map(|s| print!("{:?}", s));
    // }

    // // msg.encipher(&c1).encipher(&c2).expunge().encipher(&c3).expunge()

    // {
    //     // let mut msg = Message::new("HELLO WORLD");
    //     let mut msg = Message::new("WE ARE DISCOVERED. FLEE AT ONCE!");
    //     // let msg = "HELLO, WORLD!";
    //     // let c = pasc::CipherBuilder::with_vigenere("OCEANOGRAPHYWHAT")
    //     let c = pasc::CipherBuilder::with_beaufort("FORTIFICATION")
    //         .caseless(false)
    //         .strict(false)
    //         .no_autokey()
    //         .build();
    //     println!("MSG = {}", msg);
    //     msg.encipher(&c);
    //     println!("ENC = {}", msg);
    //     msg.decipher(&c);
    //     // println!("msg = {}\nct  = {}\npt  = {}", &msg, &enc, &dec);
    //     println!("DEC = {} (should be same: {})", msg, msg.as_str());
    //     println!("^^^--- that was the della porta");
    // }

    // {
    //     let mut msg = Message::from("THEREAREUPTOTENSUBSTITUTESPERLETTER".to_string());

    //     let c = pasc::SubstitutionCipherBuilder::with_gromark("23452")
    //         .key(String::from("ENIGMA"))
    //         .build()
    //         .unwrap();
    //     // let c = transposition::Columnar::new(&['T','T','T','T','O','O','O','O','M','M','M','M','A','A','A','A','T','T','T','T','O','O','O','O'], false);
    //     // let c = transposition::Columnar::new(&['T','O','M','A','T','O','T','O','M','A','T','O','T','O','M','A','T','O','T','O','M','A','T','O','T'], false);
    //     msg.encipher(&c);
    //     println!("enc = {}", msg);
    //     msg.decipher(&c);
    //     println!("dec = {}", msg);
    // }
    {
        let mut msg = Message::from("Hello world".to_string());

        let c = pasc::SubstitutionCipherBuilder::standard()
            .with_vigenere()
            .str_key("OCEANOGRAPHY")
            // .strict(true)
            // .caseless(true)
            .build()
            .unwrap();
        // let c = transposition::Columnar::new(&['T','T','T','T','O','O','O','O','M','M','M','M','A','A','A','A','T','T','T','T','O','O','O','O'], false);
        // let c = transposition::Columnar::new(&['T','O','M','A','T','O','T','O','M','A','T','O','T','O','M','A','T','O','T','O','M','A','T','O','T'], false);
        msg.encipher(&c);
        println!("enc = {}", msg);
        msg.decipher(&c);
        println!("dec = {}", msg);
    }
}
