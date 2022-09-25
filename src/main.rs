// mod scrapbook;
use translation::TableBuilder;

fn main() {
    run_app()
}

fn run_app() {
    println!("Hello, world!");
    let a: Vec<_> = "abcde".chars().collect();
    let b: Vec<_> = "fghij".chars().collect();
    let c: Vec<_> = "abq".chars().collect();
    let m: Vec<_> = "andrew hello".chars().collect();
    let t = TableBuilder::default()
        .src(a)
        .dst(b)
        .del(c)
        .build()
        .unwrap();
    let out = t.translate(&m);
    println!("==> {:?}", out);
}
