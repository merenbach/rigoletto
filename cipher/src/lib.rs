#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/// A Cipher implements a generic cipher.
pub trait Cipher<T, U>
where
    T: Copy,
    U: Copy,
{
    /// Encipher a sequence.
    fn encipher(&self, xs: &[T]) -> Vec<U>;

    /// Decipher a sequence.
    fn decipher(&self, xs: &[U]) -> Vec<T>;
}
