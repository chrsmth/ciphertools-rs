use super::*;
use crate::alphabet::Alphabet;
use crate::cipher_test;
use crate::tests::*;
use once_cell::sync::Lazy;

pub static CONTEXT_INCLUDE_WHITESPACE: Lazy<Autokey> =
  Lazy::new(|| Autokey::new(Alphabet::latin(), false));
pub static CONTEXT_SKIP_WHITESPACE: Lazy<Autokey> =
  Lazy::new(|| Autokey::new(Alphabet::latin(), true));

pub static KEY_A: Lazy<AutokeyKey> =
  Lazy::new(|| AutokeyKey::new("key".to_string()));

#[test]
fn key_try_new() {
  assert!(AutokeyKey::try_new(
    "hello".to_string(),
    &CONTEXT_INCLUDE_WHITESPACE
  )
  .is_ok());
  assert!(matches!(
    AutokeyKey::try_new("gr8ness".to_string(), &CONTEXT_INCLUDE_WHITESPACE),
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
