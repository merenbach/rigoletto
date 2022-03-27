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

// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     /// PID file
//     #[clap(short, long)]
//     pidfile: Option<String>,
// }

fn main() {
    // let args = Args::parse();
    run_app()
}

fn run_app() {
    // {
    //     let k = matrix![6, 24, 1; 13, 16, 10; 20, 17, 15];
    //     // let msg = dmatrix!(&[0, 2, 19]);

    //     let ab: Vec<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    //     let x = HillCipherBuilder::default()
    //         .alphabet(ab)
    //         .n(3)
    //         .build()
    //         .unwrap();
    //     let m: Vec<_> = "ACT".chars().collect();
    //     let y = x.encipher(&m, &k);
    //     println!("y = {:?}", y);

    //     // let z = x.decipher(&y, &k);
    //     // println!("z = {:?}", z);
    // }
    // {
    //     let msg = "Attack at dawn. By dawn I mean 0500. Not 0915 like you did last time.";
    //     let d = &[1, 3, 9, 1, 9, 5];
    //     let kg = &[7, 2, 4, 0, 1];
    //     let v = VIC {
    //         phrase: "Twas the night before Christmas".to_string(),
    //         date: d.to_vec(),
    //         personal_number: 6,
    //         keygroup: kg.to_vec(),
    //     };

    //     v.keygen();
    // }

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
