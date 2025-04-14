use super::*;
use crate::alphabet::Alphabet;
use crate::cipher_test;
use crate::tests::*;
use once_cell::sync::Lazy;

pub static CONTEXT: Lazy<Substitution> =
  Lazy::new(|| Substitution::new(Alphabet::latin()));

pub static KEY_A: Lazy<SubstitutionEncipherKey> = Lazy::new(|| {
  SubstitutionEncipherKey::new(
    "zebrascdfghijklmnopqtuvwxy".to_string(),
    &Alphabet::latin(),
  )
});

#[test]
fn key_try_new() {
  assert!(matches!(
    SubstitutionEncipherKey::try_new(
      "abcdefghijklmnopqrstuvwxy".to_string(),
      &CONTEXT
    ),
    Err(ParseError::LengthTooShort),
  ));

  assert!(matches!(
    SubstitutionEncipherKey::try_new(
      "abcdefghijklmnopqrstuvwxyza".to_string(),
      &CONTEXT
    ),
    Err(ParseError::LengthTooLong),
  ));

  assert!(matches!(
    SubstitutionEncipherKey::try_new(
      "abcdefghijklmnopqrs7uvwxyz".to_string(),
      &CONTEXT
    ),
    Err(ParseError::InvalidChar('7')),
  ));

  assert!(matches!(
    SubstitutionEncipherKey::try_new(
      "abcdefghijklmnopqrstuvwxya".to_string(),
      &CONTEXT
    ),
    Err(ParseError::DuplicateChar('a')),
  ));

  assert!(
    SubstitutionEncipherKey::try_new(
      "abcdefghijklmnopqrstuvwxyz".to_string(),
      &CONTEXT
    )
    .is_ok()
  );
}

cipher_test!(CONTEXT, *QUICK_BROWN_FOX, KEY_A, include_whitespace);
