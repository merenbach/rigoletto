use serde::Deserialize;
use std_ext;

// fn main() {
//     let u = read_user_from_file("test.json").unwrap();
//     println!("{:#?}", u);
// }

// fn run_masc_test_case<F>(path: &str, cb: F)
// where
//     F: Fn(TestCase) -> SubstitutionCipher,
// {
//     let tables: Vec<MascTestCase> = std_ext::read_data_from_file("testdata/masc/atbash_decipher.json").unwrap();

//     for t in tables {
//         let mut c = cb(&t);
//         c.strict = t.strict;
//         c.caseless = t.caseless;
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_atbash_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/atbash_encipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_atbash()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_atbash_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/atbash_decipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_atbash()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_masc_dummy_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/dummy_encipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::default()
//             .with_dummy()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_masc_dummy_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/dummy_decipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::default()
//             .with_dummy()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_rot13_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         caseless: bool,
//         input: String,
//         output: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/rot13_encipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_rot13()
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_rot13_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         caseless: bool,
//         input: String,
//         output: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/rot13_decipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_rot13()
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_caesar_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         shift: usize,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/caesar_encipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_caesar(t.shift)
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_caesar_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         shift: usize,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/caesar_decipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_caesar(t.shift)
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_keyword_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         keyword: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/keyword_encipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_keyword_str(&t.keyword)
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_keyword_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         keyword: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/keyword_decipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_keyword_str(&t.keyword)
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_affine_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         slope: usize,
//         intercept: usize,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/affine_encipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_affine(t.slope, t.intercept)
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_affine_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         slope: usize,
//         intercept: usize,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/affine_decipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_affine(t.slope, t.intercept)
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_decimation_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         multiplier: usize,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/decimation_encipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_decimation(t.multiplier)
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_decimation_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         multiplier: usize,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/masc/decimation_decipher.json").unwrap();

//     for t in tables {
//         let c = masc::SubstitutionCipherBuilder::standard()
//             .with_decimation(t.multiplier)
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_vigenere_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         autokey: u8,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/vigenere_encipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_vigenere()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .autoclave(match t.autokey {
//                 1 => pasc::AutoclaveKind::Text,
//                 2 => pasc::AutoclaveKind::Key,
//                 _ => pasc::AutoclaveKind::None,
//             })
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_vigenere_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         autokey: u8,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/vigenere_decipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_vigenere()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .autoclave(match t.autokey {
//                 1 => pasc::AutoclaveKind::Text,
//                 2 => pasc::AutoclaveKind::Key,
//                 _ => pasc::AutoclaveKind::None,
//             })
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_beaufort_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/beaufort_encipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_beaufort()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_beaufort_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/beaufort_decipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_beaufort()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_gromark_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         primer: String,
//         input: String,
//         output: String,
//         keyword: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/gromark_encipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_gromark(&t.keyword, &t.primer)
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_gromark_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         primer: String,
//         input: String,
//         output: String,
//         keyword: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/gromark_decipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_gromark(&t.keyword, &t.primer)
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_gronsfeld_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/gronsfeld_encipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_gronsfeld()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_gronsfeld_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/gronsfeld_decipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_gronsfeld()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_dellaporta_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/dellaporta_encipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_della_porta()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_dellaporta_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/dellaporta_decipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_della_porta()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_trithemius_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/trithemius_encipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_trithemius()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_trithemius_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/trithemius_decipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_trithemius()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_variantbeaufort_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/variantbeaufort_encipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_variant_beaufort()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_variantbeaufort_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/variantbeaufort_decipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_variant_beaufort()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_pasc_dummy_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/dummy_encipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_dummy()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_pasc_dummy_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         alphabet: String,
//         caseless: bool,
//         input: String,
//         output: String,
//         key: String,
//         strict: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/pasc/dummy_decipher.json").unwrap();

//     for t in tables {
//         let c = pasc::SubstitutionCipherBuilder::default()
//             .with_dummy()
//             .pt_alphabet(t.alphabet.chars().collect())
//             .caseless(t.caseless)
//             .strict(t.strict)
//             .str_key(&t.key)
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn test_railfence_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         input: String,
//         output: String,
//         rows: usize,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/transposition/railfence_encipher.json").unwrap();

//     for t in tables {
//         let c = transposition::rail_fence(t.rows);
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn test_railfence_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         input: String,
//         output: String,
//         rows: usize,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/transposition/railfence_decipher.json").unwrap();

//     for t in tables {
//         let c = transposition::rail_fence(t.rows);
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn scytale_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         input: String,
//         output: String,
//         turns: usize,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/transposition/scytale_encipher.json").unwrap();

//     for t in tables {
//         let c = transposition::scytale(t.turns);
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn scytale_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         input: String,
//         output: String,
//         turns: usize,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/transposition/scytale_decipher.json").unwrap();

//     for t in tables {
//         let c = transposition::scytale(t.turns);
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }

// #[test]
// fn columnar_transposition_encipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         input: String,
//         output: String,
//         nulls: String,
//         key: String,
//         myszkowski: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/transposition/columnar_transposition_encipher.json")
//             .unwrap();

//     for t in tables {
//         let c = transposition::ColumnarTranspositionCipherBuilder::with_str_key(&t.key)
//             .myszkowski(t.myszkowski)
//             .nulls(t.nulls.chars().collect())
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.encipher(&c).to_string());
//     }
// }

// #[test]
// fn columnar_transposition_decipher() {
//     #[derive(Deserialize)]
//     struct TestCase {
//         input: String,
//         output: String,
//         nulls: String,
//         key: String,
//         myszkowski: bool,
//     }

//     let tables: Vec<TestCase> =
//         std_ext::read_data_from_file("testdata/transposition/columnar_transposition_decipher.json")
//             .unwrap();

//     for t in tables {
//         let c = transposition::ColumnarTranspositionCipherBuilder::with_str_key(&t.key)
//             .myszkowski(t.myszkowski)
//             .nulls(t.nulls.chars().collect())
//             .build()
//             .unwrap();
//         let mut msg = message::Message::from(t.input);
//         assert_eq!(t.output, msg.decipher(&c).to_string());
//     }
// }
