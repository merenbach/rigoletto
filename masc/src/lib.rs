pub mod tableau;
pub mod transform;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn test_affine_transform() {
//     //     assert_eq!(2 + 2, 4);
//     // }

//     // #[test]
//     // fn test_keyword_transform() {
//     //     assert_eq!(2 + 2, 4);
//     // }
// }

// Four total possible combinations yield three possible outcomes:
// 1. HashMap entry v found for key k, strict enabled: return Some(v)
// 2. HashMap entry v found for key k, strict disabled: return Some(v)
// 3. HashMap entry v not found for key k, strict enabled: return None
// 4. HashMap entry v not found for key k, strict disabled: return Some(k)
/*
// Get a transcoded rune (optionally ignoring case) and a boolean indicating success.
// Get (-1) instead if strict mode is enabled.
// Get the original rune back instead if strict mode is disabled.
*/

// /// Modulus performs a Euclidean remainder operation.
// /// // TODO: add test
// fn modulus<T: Integer + Copy>(x: T, y: T) -> T {
//     return ((x % y) + y) % y;
// }

// fn build_lcg<T>(seed: T, multiplier: T, increment: T, modulus: T, count: usize) -> Vec<T>
// where
//     T: Integer + Unsigned + Copy,
// {
//     use std::iter::successors;

//     successors(Some(seed), |n| multiplier * seed + increment % modulus)
//         .skip(1)
//         .take(count);
// }

// fn sentinel_subtract(x: usize) -> impl Fn(usize) -> usize {
//     return move |i: usize| -> usize { return i - x };
// }

// enum ModNum {
//     Natural(usize),
//     SubtractFrom(usize),
// }
