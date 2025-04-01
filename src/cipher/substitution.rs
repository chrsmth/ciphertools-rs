use std::fmt;

use crate::alphabet::Alphabet;
use crate::cipher::{Decipher, Encipher};
use ahash::AHashMap;

struct SubstitutionKey(AHashMap<char, char>);
pub struct SubstitutionDecipherKey(AHashMap<char, char>);
pub struct SubstitutionEncipherKey(AHashMap<char, char>);

#[derive(Default)]
pub struct Substitution {
  alphabet: Alphabet,
}

impl Substitution {
  pub fn new(alphabet: Alphabet) -> Self {
    Substitution { alphabet }
  }

  pub fn alphabet(&self) -> &Alphabet {
    &self.alphabet
  }
}

pub enum ParseError {
  LengthTooShort,
  LengthTooLong,
  InvalidChar(char),
  DuplicateChar(char),
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ParseError::LengthTooShort => write!(f, "Key is shorter than alphabet"),
      ParseError::LengthTooLong => write!(f, "Key is longer than alphabet"),
      ParseError::InvalidChar(c) => write!(f, "Invalid character: '{c}'"),
      ParseError::DuplicateChar(c) => write!(f, "Duplicate character: '{c}'"),
    }
  }
}

impl SubstitutionEncipherKey {
  //TODO Retrun result with error explanation
  pub fn try_new(key: String, alphabet: &Alphabet) -> Result<Self, ParseError> {
    SubstitutionKey::try_new(key, alphabet).map(SubstitutionEncipherKey::from)
  }

  pub fn new(key: String, alphabet: &Alphabet) -> Self {
    SubstitutionKey::new(key, alphabet).into()
  }
}

impl SubstitutionDecipherKey {
  pub fn try_new(key: String, alphabet: &Alphabet) -> Result<Self, ParseError> {
    SubstitutionKey::try_new(key, alphabet).map(SubstitutionDecipherKey::from)
  }

  pub fn new(key: String, alphabet: &Alphabet) -> Self {
    SubstitutionKey::new(key, alphabet).into()
  }
}

impl From<SubstitutionKey> for SubstitutionEncipherKey {
  fn from(value: SubstitutionKey) -> Self {
    SubstitutionEncipherKey(value.0)
  }
}

impl From<SubstitutionKey> for SubstitutionDecipherKey {
  fn from(value: SubstitutionKey) -> Self {
    SubstitutionDecipherKey(value.0)
  }
}

impl SubstitutionKey {
  pub fn new(key: String, alphabet: &Alphabet) -> Self {
    SubstitutionKey(key.chars().zip(alphabet.iter()).fold(
      AHashMap::new(),
      |mut acc, (k, v)| {
        acc.insert(k, v);
        acc
      },
    ))
  }

  pub fn try_new(key: String, alphabet: &Alphabet) -> Result<Self, ParseError> {
    match key.len().cmp(&alphabet.len()) {
      std::cmp::Ordering::Less => return Err(ParseError::LengthTooShort),
      std::cmp::Ordering::Greater => return Err(ParseError::LengthTooLong),
      _ => (),
    }

    if let Some(k) = key.chars().find(|&k| !alphabet.contains(k)) {
      return Err(ParseError::InvalidChar(k));
    }

    key
      .chars()
      .zip(alphabet.iter())
      .try_fold(AHashMap::new(), |mut acc, (k, v)| {
        acc
          .insert(k, v) // try_insert() would be more readable, but it is a nightly only API
          .map_or_else(|| Ok(acc), |_| Err(ParseError::DuplicateChar(k)))
      })
      .map(SubstitutionKey)
  }
}

impl Encipher for Substitution {
  type Key = SubstitutionEncipherKey;

  fn encipher(&self, plaintext: &str, key: &Self::Key) -> String {
    plaintext
      .chars()
      .map(|c| key.0.get(&c).copied().unwrap_or(c))
      .collect()
  }
}

impl Decipher for Substitution {
  type Key = SubstitutionDecipherKey;

  fn decipher(&self, ciphertext: &str, key: &Self::Key) -> String {
    ciphertext
      .chars()
      .map(|c| key.0.get(&c).copied().unwrap_or(c))
      .collect()
  }
}
