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
}

#[derive(Builder, Default)]
#[builder(build_fn(validate = "Self::validate"))]
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
    // TODO: consider not using builder so we don't have to have mishigas caching

    /// Ensure that the mapping is initialized, then return it.
    fn ensure(&self) -> &RefCell<HashMap<T, Option<T>>> {
        if self.map.borrow().is_empty() {
            *self.map.borrow_mut() = self
                .src
                .iter()
                .zip(self.dst.iter())
                .map(|(&x, &y)| (x, Some(y)))
                .chain(self.del.iter().map(|&z| (z, None)))
                .collect()
        }
        &self.map
    }

    /// Translate one element.
    pub fn translate_one(&self, x: &T) -> Option<T> {
        let map = self.ensure();
        *map.borrow().get(x).unwrap_or(&Some(*x))
    }

    /// Translate a sequence of elements.
    pub fn translate(&self, xs: &[T]) -> Vec<T> {
        xs.iter().filter_map(|x| self.translate_one(x)).collect()
    }
}
