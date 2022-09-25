// mod scrapbook;
use translation::{Table, TableBuilder};

fn main() {
    run_app()
}

pub trait Atom: std::hash::Hash + Eq + Copy + Default {}
impl<T> Atom for T where T: std::hash::Hash + Eq + Copy + Default {}

struct MyMascCipher<T>
where
    T: Atom,
{
    pt_alphabet: Vec<T>,
    ct_alphabet: Vec<T>,
    pt2ct: Table<T>,
    ct2pt: Table<T>,
    strict: bool,
}

impl<T> MyMascCipher<T>
where
    T: Atom,
{
    fn new(pt_alphabet: &[T], ct_alphabet: &[T], strict: bool) -> Self {
        let pt2ct = TableBuilder::default()
            .src(pt_alphabet)
            .dst(ct_alphabet)
            .build()
            .unwrap();
        let ct2pt = TableBuilder::default()
            .src(ct_alphabet)
            .dst(pt_alphabet)
            .build()
            .unwrap();

        Self {
            pt_alphabet: pt_alphabet.to_owned(),
            ct_alphabet: ct_alphabet.to_owned(),
            pt2ct,
            ct2pt,
            strict,
        }
    }

    fn encipher(&self, xs: &[T]) -> Vec<T> {
        self.pt2ct
            .translate(xs, |x| if self.strict { None } else { Some(x) })
    }

    fn decipher(&self, xs: &[T]) -> Vec<T> {
        self.ct2pt
            .translate(xs, |x| if self.strict { None } else { Some(x) })
    }
}

fn run_app() {
    // println!("Hello, world!");
    // let a: Vec<_> = "abcde".chars().collect();
    // let b: Vec<_> = "fghij".chars().collect();
    // let c: Vec<_> = "abq".chars().collect();
    // let m: Vec<_> = "andrew hello".chars().collect();
    // let t = TableBuilder::default()
    //     .src(a)
    //     .dst(b)
    //     .del(c)
    //     .build()
    //     .unwrap();
    // let out = t.translate(&m);
    // println!("==> {:?}", out);

    let pt_alphabet: Vec<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let ct_alphabet: Vec<_> = "DEFGHIJKLMNOPQRSTUVWXYZABC".chars().collect();

    let x = MyMascCipher::new(&pt_alphabet, &ct_alphabet, true);
    let msg: Vec<_> = "HELLO, WORLD!".chars().collect();
    let y = x.encipher(&msg);
    let z = x.decipher(&y);
    println!("{:?} --> {:?}", y, z);
}
