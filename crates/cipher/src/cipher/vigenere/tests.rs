use super::*;
use crate::alphabet::Alphabet;
use crate::cipher_test;
use crate::tests::*;
use once_cell::sync::Lazy;

pub static CONTEXT_INCLUDE_WHITESPACE: Lazy<Vigenere> =
  Lazy::new(|| Vigenere::new(Alphabet::latin(), false));
pub static CONTEXT_SKIP_WHITESPACE: Lazy<Vigenere> =
  Lazy::new(|| Vigenere::new(Alphabet::latin(), true));

pub static KEY_A: Lazy<VigenereKey> =
  Lazy::new(|| VigenereKey::new("key".to_string()));

#[test]
fn key_try_new() {
  assert!(VigenereKey::try_new(
    "hello".to_string(),
    &CONTEXT_INCLUDE_WHITESPACE
  )
  .is_ok());
  assert!(matches!(
    VigenereKey::try_new("gr8ness".to_string(), &CONTEXT_INCLUDE_WHITESPACE),
    Err(ParseError::InvalidChar('8')),
  ));
}

cipher_test!(
  CONTEXT_INCLUDE_WHITESPACE,
  *QUICK_BROWN_FOX,
  KEY_A,
  include_whitespace
);

cipher_test!(
  CONTEXT_SKIP_WHITESPACE,
  *QUICK_BROWN_FOX,
  KEY_A,
  skip_whitespace
);
