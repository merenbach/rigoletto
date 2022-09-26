use derive_builder::Builder;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod tests {
    use super::make_translation_table;
    use std::collections::HashMap;

    #[test]
    fn make_translation_table_works() {
        let rows = &[
            // (vec![], vec![], vec![], vec![]),
            (
                vec![('A', Some('D')), ('B', Some('E')), ('C', Some('F'))],
                vec!['A', 'B', 'C'],
                vec!['D', 'E', 'F'],
                vec![],
            ),
            (
                vec![
                    ('A', Some('D')),
                    ('B', Some('E')),
                    ('C', Some('F')),
                    ('G', None),
                ],
                vec!['A', 'B', 'C'],
                vec!['D', 'E', 'F'],
                vec!['G'],
            ),
            (
                vec![('A', None), ('B', Some('E')), ('C', Some('F')), ('E', None)],
                vec!['A', 'B', 'C'],
                vec!['D', 'E', 'F'],
                vec!['A', 'E'],
            ),
        ];
        for row in rows {
            println!("{:?}", row);
            let expect: HashMap<_, Option<_>> = row.0.iter().copied().collect();
            assert_eq!(expect, make_translation_table(&row.1, &row.2, &row.3));
        }
    }
}

/// Make a translation table that supports deletion.
/// This is inspired by Python's `str.maketrans()`.
/// Elements passed in `del` will override those in `src` if there is overlap.
/// TODO: consider removing U for symmetry.
fn make_translation_table<T, U>(src: &[T], dst: &[U], del: &[T]) -> HashMap<T, Option<U>>
where
    T: Copy + Eq + Hash,
    U: Copy,
{
    src.iter()
        .zip(dst.iter())
        .map(|(&x, &y)| (x, Some(y)))
        .chain(del.iter().map(|&z| (z, None)))
        .collect()
}

/// Translate a sequence using a hashmap, leaving non-translateable elements unchanged .
/// This is inspired by Python's `str.translate()`.
/// TODO: use unwrap_or_else instead
fn translate<T>(xs: &[T], m: &HashMap<T, Option<T>>, fallback: impl Fn(T) -> Option<T>) -> Vec<T>
where
    T: Copy + Eq + Hash,
{
    xs.iter()
        .filter_map(|&x| *m.get(&x).unwrap_or(&fallback(x)))
        .collect()
}


// /// Translate an element based on a given translation table.
// /// This is inspired by Python's `str.translate()`.
// fn translate_one<T, U>(m: &HashMap<T, Option<U>>, x: &T, default: Option<U>) -> Option<U>
// where
//     T: Copy + Eq + Hash,
//     U: Copy,
// {
//     m.get(x).copied().unwrap_or(default)
// }

// /// A Cipher implements a generic cipher.
// pub trait Translate<T, U, A = Self>
// where
//     T: Copy + Eq + Hash,
//     U: Copy,
// {
//     /// Translate a sequence.
//     fn translate(&self, xs: HashMap<T, U>, strict: bool) -> Vec<Option<U>>
//     where
//         I: Iterator<Item = A>,
//     {
//         self.iter()
//             .map(|x| xs.contains_key(x).unwrap_or(*x))
//             .collect()
//     }
// }

pub trait Atom: Hash + Eq + Copy + Default {}
impl<T> Atom for T where T: Hash + Eq + Copy + Default {}

impl<T> TableBuilder<T>
where
    T: Atom,
{
    fn validate(&self) -> Result<(), String> {
        if self.src.as_ref().unwrap().len() != self.dst.as_ref().unwrap().len() {
            Err("source and destination mappings must be the same length".to_string())
        } else {
            Ok(())
        }
    }

    // fn source_str(s:&str) {
    //     self.source = s.chars().co
    // }
}

#[derive(Builder, Default, Clone)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Table<T>
where
    T: Atom,
{
    #[builder(setter(into))]
    src: Vec<T>,
    #[builder(setter(into))]
    dst: Vec<T>,
    #[builder(setter(into), default)]
    del: Vec<T>,

    #[builder(setter(skip))]
    map: RefCell<HashMap<T, Option<T>>>,
}

impl<T> Table<T>
where
    T: Atom,
{
    pub fn is_empty(&self) -> bool {
        self.map.borrow().is_empty()
    }

    // TODO: consider not using builder so we don't have to have mishigas caching

    // translation::Table::new("ABCDE", "defgh", "")
    // translation::Table::default().src("ABCDE").dst("defgh").del("!*").build().unwrap()

    /// Ensure that the mapping is initialized, then return it.
    fn ensure(&self) -> &RefCell<HashMap<T, Option<T>>> {
        if self.map.borrow().is_empty() {
            *self.map.borrow_mut() = make_translation_table(&self.src, &self.dst, &self.del);
        }
        &self.map
    }

    /// Translate one element.
    // fn translate_one_default(&self, x: &T, default: Option<T>) -> Option<T> {
    //     let map = self.ensure();
    //     *map.borrow().get(x).unwrap_or(&default)
    // }

    // /// Translate one element.
    // pub fn translate_one(&self, x: &T, fallback: impl Fn(T) -> Option<T>) -> Option<T> {
    //     self.translate_one_default(x, fallback)
    // }

    /// Translate one element.
    /// TODO: use unwrap_or_else instead
    pub fn translate_one(&self, x: &T, fallback: impl Fn(T) -> Option<T>) -> Option<T> {
        let map = self.ensure();
        *map.borrow().get(x).unwrap_or(&fallback(*x))
    }

    /// Translate a sequence of elements.
    pub fn translate(&self, xs: &[T], fallback: impl Fn(T) -> Option<T>) -> Vec<T> {
        let map = self.ensure();
        translate(xs, &(*map.borrow()), fallback)
    }
}
