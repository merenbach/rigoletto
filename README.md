# Rigoletto

Rust-based classical cryptography framework

## TODO

- Major cipher refactor:
  1. Get everything into Cipher package.
  2. Improve algorithms. Maybe allow only chars for now, but provide underlying ciphers that accept more (e.g., simple substitution cipher, tabula recta, etc.).
- Improve transposition nulls and allow cycling of nulls (i.e., provide only "X")
- LCG tests ought to be expanded
- Figure out ergonomics of None vs Dummy cipherkind and default encipherment modes to return (dummy? use specified CT? vigenere?)
- Custom traits could remove duplication, especially in MASC and PASC.
- Restore pt_alphabet setting in ROT13? Or remove entirely? or make only as a shortcut function?
- Can we use no_std?
- Use setter each on transposition to restore multi-key support: <https://github.com/colin-kiegel/rust-derive-builder/issues/213>
- Determine if we can/ought to avoid empty kinds (e.g., Dummy or None) for ciphers.
- Standardize on triple slash for doc purposes where useful.
- Should transposition cipher allow multiple keys again?
- Add custom alphabets to cipher integration tests
- Add doc tags like: ```#[doc = r"`myszkowski` decides whether to use Myszkowski transposition."]```
- Improve Gromark key length determination--can we use an iterator?
- Add tests for "simple" and "dummy" cases of substitution ciphers (that is, without named cipher flavors, where ct_alphabet is or isn't overridden, respectively).
- Refactor integration tests and add tests for transforms directly.
- Extreme use of clone() is fairly inefficient. Find another way.
- Close edge cases in PASC re: custom alphabets and different ciphers.
- Remove no-op concept from MASC and PASC and instead don't allow conflicting options, if possible; otherwise, send up a warning or error.
- For refactoring into named cipher packages: allow changing of alphabet after cipher creation, if at all possible, so that ciphers may be created with simple functions and not builders.
- Create text transformation functions for reuse elsewhere (keyword transform, etc.)
- Add homophonic cipher (more than one possible encryption per letter, not to be confused with homomorphic ciphers, which allow manipulation to pass through to cipehrtext)
- Add unit tests and try to break out supporting functionality to minimize count and variety of these for full ciphers
- Convenience iteration over alphabets
- Translation tables maybe should accept alphabets? or not?
- Should translation table require an alphabet? Or can we find some other way to avoid converting back and forth from string repeatedly? (But we also want to allow repeats in destination.)
- Return `Option<String>` from tabula recta in case key is totally unusable -- or create Key wrapper to handle parsing automatically
- Rename Cipher to something like Cipherable, less of a noun and more of an adjective-cum-noun
- See if we can avoid traits in some places (e.g., Copy) by borrowing more

Nice-to-have:

- Expand all tests, e.g., Hull-Dobell testing.

## Implementation notes

- Affine cipher: y = mx+b
  - Atbash: affine cipher with slope=(alphabet length - 1), intercept = (alphabet length - 1)
  - Caesar: affine cipher with slope=1, intercept=(shift)
    - Rot13: Caesar cipher with shift=13
  - Decimation: affine cipher with slope=(multiplier), intercept=0
- Columnar transposition cipher: key is integers; when a string sequence is provided, convert to lexical ordering; Myszkowski allows repetition of values in ordering.
  - Rail fence: columnar transposition with key=(zigzag sequence with period 2*(rows-1)) and Myszkowski transposition
  - Scytale: columnar transposition with key=(ascending integer sequence with length equal to number of turns)
