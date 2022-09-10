use derive_builder::Builder;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
// use std::collections::BTreeMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

macro_rules! maketrans2 {
    ($xs:expr , $ys:expr $(,$zs:expr)?) => {
        {
            $xs.iter().zip($ys.iter()).map(|(&x, &y)| (x, Some(y)))
                $ ( .chain( $zs.iter().map(|&z| (z, None))) )?
                .collect()
        }
    };
}

// macro_rules! maketrans3 {
//     ($xs:expr , $ys:expr $(,$zs:expr)?) => {
//         {
//             $xs.iter().zip($ys.iter()).map(|(x, y)| (x, Some(y)))
//                 $ ( .chain( $zs.iter().map(|z| (z, None))) )?
//                 .collect()
//         }
//     };
// }

pub trait Atom: Hash + Eq + Copy + Default {}
impl<T> Atom for T where T: Hash + Eq + Copy + Default {}

#[derive(Builder, Default)]
pub struct Table<T>
where
    T: Atom,
{
    #[builder(setter(into))]
    src: Vec<T>,
    #[builder(setter(into))]
    dst: Vec<T>,
    #[builder(setter(into))]
    del: Vec<T>,

    #[builder(setter(skip))]
    map: RefCell<HashMap<T, Option<T>>>,
}

impl<T> Table<T>
where
    T: Atom,
{
    // TODO: validate lengths
    // TODO: refcell to save

    // fn translate_one(&self, x: &T) -> Option<T> {
    //     // todo: initialize
    //     let xlator: HashMap<T, Option<T>> = self
    //         .t
    //         .iter()
    //         .zip(self.u.iter())
    //         .map(|(x, y)| (x, Some(y)))
    //         .chain(self.v.iter().map(|z| (z, None)))
    //         .collect();
    //     // maketrans3!(self.t, self.u, self.v);
    //     xlator.get(x).unwrap_or(Some(*x))
    // }

    fn initialize(&self) -> &RefCell<HashMap<T, Option<T>>> {
        if self.map.borrow().is_empty() {
            *self.map.borrow_mut() = maketrans2!(self.src, self.dst, self.del);
        }
        &self.map
    }

    pub fn translate(&self, xs: &[T]) -> Vec<T> {
        let map = self.initialize();
        xs.iter()
            .filter_map(|&x| *map.borrow().get(&x).unwrap_or(&Some(x)))
            .collect()
    }
}
