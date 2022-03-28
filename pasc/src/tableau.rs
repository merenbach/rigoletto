use masc::tableau::{Atom, Tableau};
use std::collections::HashMap;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// struct prototable {
//     pt_alphabet: Vec<char>,
//     ct_alphabet: Vec<char>,

//     pt_map: HashMap<char, usize>,
//     ct_map: HashMap<char, usize>,
// }

/*impl prototable {
    fn new(pt_alphabet: &str, ct_alphabet: &str) -> Self {
        Self {
            pt_alphabet: pt_alphabet.chars().collect(),
            ct_alphabet: ct_alphabet.chars().collect(),
            pt_map: pt_alphabet
                .chars()
                .enumerate()
                .map(|(i, c)| (c, i))
                .collect(),
            ct_map: ct_alphabet
                .chars()
                .enumerate()
                .map(|(i, c)| (c, i))
                .collect(),
        }
    }

    fn encipher(&self, c: char, offsets: &[usize]) -> Option<&char> {
        let offset_sum: usize = offsets.iter().sum();
        let ct_len = self.ct_map.len();
        match self.pt_map.get(&c) {
            Some(&i) => self.ct_alphabet.get((i + offset_sum) % ct_len),
            None => None,
        }
    }

    fn decipher(&self, c: char, offsets: &[usize]) -> Option<&char> {
        let offset_sum: usize = offsets.iter().sum();
        let pt_len = self.pt_map.len();
        match self.ct_map.get(&c) {
            Some(&i) => self.pt_alphabet.get((i + pt_len - offset_sum) % pt_len),
            None => None,
        }
    }
}*/

#[derive(Default, Clone)]
pub struct ReciprocalTable<K: Atom, T: Atom, U: Atom>(HashMap<K, Tableau<T, U>>);

impl<K: Atom, T: Atom, U: Atom> ReciprocalTable<K, T, U> {
    pub fn new(xs: &[T], ys: &[Vec<U>], zs: &[K]) -> Self {
        Self(
            zs.iter()
                .copied()
                .enumerate()
                .map(|(i, z)| (z, Tableau::new(xs, &ys[i])))
                .collect(),
        )
    }

    pub fn keyset(&self) -> Vec<K> {
        self.0.keys().copied().collect()
    }

    // Encode an element.
    pub fn encode(&self, x: &T, k: &K) -> Option<U> {
        self.0.get(&k)?.encode(x)
    }

    // Decode an element.
    pub fn decode(&self, x: &U, k: &K) -> Option<T> {
        self.0.get(&k)?.decode(x)
    }

    // /// Encipher a string.
    // fn encipher(&self, xs: &[T]) -> Vec<U> {
    //     self.initialize();

    //     let mut mutakey = self.make_key(xs.len());

    //     let tr = self..borrow();

    //     xs.iter()
    //         // can use .scan(0, |cursor, &c| if we're not going to return None
    //         .filter_map(|&c| {
    //             let k = mutakey.front().unwrap();
    //             let raw_table = match self.lookup {
    //                 Some(f) => (f)(k, &tr),
    //                 _ => tr.get(k),
    //             };
    //             let table = raw_table.unwrap();
    //             match table.encipher_one(&c) {
    //                 Some(o) => {
    //                     let elem = mutakey.pop_front().unwrap();
    //                     match self.autoclave {
    //                         AutoclaveKind::None => mutakey.push_back(elem),
    //                         AutoclaveKind::Key => mutakey.push_back(o),
    //                         AutoclaveKind::Text => mutakey.push_back(c),
    //                     };
    //                     Some(o)
    //                 }
    //                 None => {
    //                     if !self.strict {
    //                         Some(c)
    //                     } else {
    //                         None
    //                     }
    //                 }
    //             }
    //         }) // .filter_map for caseless
    //         .collect()
    // }

    // /// Decipher a string.
    // fn decipher(&self, xs: &[U]) -> Vec<T> {
    //     self.initialize();

    //     let mut mutakey = self.make_key(xs.len());

    //     let tr = self.tabula_recta.borrow();

    //     xs.iter()
    //         // can use .scan(0, |cursor, &c| if we're not going to return None
    //         .filter_map(|&c| {
    //             let k = mutakey.front().unwrap();
    //             let raw_table = match self.lookup {
    //                 Some(f) => (f)(k, &tr),
    //                 _ => tr.get(k),
    //             };
    //             let table = raw_table.unwrap();
    //             match table.decipher_one(&c) {
    //                 Some(o) => {
    //                     let elem = mutakey.pop_front().unwrap();
    //                     match self.autoclave {
    //                         AutoclaveKind::None => mutakey.push_back(elem),
    //                         AutoclaveKind::Key => mutakey.push_back(c),
    //                         AutoclaveKind::Text => mutakey.push_back(o),
    //                     };
    //                     Some(o)
    //                 }
    //                 None => {
    //                     if !self.strict {
    //                         Some(c)
    //                     } else {
    //                         None
    //                     }
    //                 }
    //             }
    //         })
    //         .collect()
    // }
}
