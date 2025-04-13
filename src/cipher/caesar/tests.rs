use super::*;
use crate::alphabet::Alphabet;
use crate::cipher_test;
use crate::tests::*;
use once_cell::sync::Lazy;

pub static CONTEXT: Lazy<Caesar> = Lazy::new(|| Caesar::new(Alphabet::latin()));

pub static KEY_A: Lazy<CaesarKey> = Lazy::new(|| CaesarKey::new('a'));
pub static KEY_B: Lazy<CaesarKey> = Lazy::new(|| CaesarKey::new('b'));

#[test]
fn key_try_new() {
  let valid_keys = LATIN_LOWERCASE.iter();
  let invalid_keys = RUSSIAN_LOWERCASE
    .iter()
    .chain(GREEK_LOWERCASE)
    .chain(DIGITS)
    .chain(SYMBOLS);

  valid_keys.for_each(|&c| assert!(CaesarKey::try_new(c, &CONTEXT).is_ok()));
  invalid_keys.for_each(|&c| assert!(CaesarKey::try_new(c, &CONTEXT).is_err()));
}

cipher_test!(CONTEXT, *QUICK_BROWN_FOX, KEY_A, latin_a_spaces);
cipher_test!(CONTEXT, *QUICKBROWNFOX, KEY_A, latin_a_no_spaces);
cipher_test!(CONTEXT, *QUICK_BROWN_FOX, KEY_B, latin_b_spaces);
cipher_test!(CONTEXT, *QUICKBROWNFOX, KEY_B, latin_b_no_spaces);
