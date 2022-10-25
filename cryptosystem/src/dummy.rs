use cipher::Cipher;
use dummy::Dummy;

/// Make a dummy (no-op) cipher.
pub fn make<T: Copy>() -> impl Cipher<T, T> {
    Dummy::default()
}
