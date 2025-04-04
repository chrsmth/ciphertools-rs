use crate::alphabet::Alphabet;
use crate::cipher::{Decipher, Encipher};
use ahash::AHashMap;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone)]
struct SubstitutionKey(AHashMap<char, char>);

#[derive(Debug, Clone)]
pub struct SubstitutionDecipherKey(AHashMap<char, char>);

#[derive(Debug, Clone)]
pub struct SubstitutionEncipherKey(AHashMap<char, char>);

#[derive(Debug, Clone)]
pub struct Substitution {
  alphabet: Alphabet,
}

#[derive(Debug)]
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

impl Substitution {
  pub fn new(alphabet: Alphabet) -> Self {
    Substitution { alphabet }
  }

  pub fn alphabet(&self) -> &Alphabet {
    &self.alphabet
  }
}

impl SubstitutionEncipherKey {
  fn try_new(key: String, context: &Substitution) -> Result<Self, ParseError> {
    SubstitutionKey::try_new(key, context).map(SubstitutionEncipherKey::from)
  }

  pub fn new(key: String, alphabet: &Alphabet) -> Self {
    SubstitutionKey::new(key, alphabet).into()
  }

  pub fn inverse(self) -> SubstitutionDecipherKey {
    SubstitutionKey(self.0).inverse().into()
  }
}

impl TryFrom<(&str, &Substitution)> for SubstitutionEncipherKey {
  type Error = ParseError;

  fn try_from(
    (key, context): (&str, &Substitution),
  ) -> Result<Self, Self::Error> {
    Self::try_new(key.to_string(), context)
  }
}

impl From<SubstitutionKey> for SubstitutionEncipherKey {
  fn from(value: SubstitutionKey) -> Self {
    SubstitutionEncipherKey(value.0)
  }
}

impl SubstitutionDecipherKey {
  fn try_new(key: String, context: &Substitution) -> Result<Self, ParseError> {
    SubstitutionKey::try_new(key, context).map(SubstitutionDecipherKey::from)
  }

  pub fn new(key: String, alphabet: &Alphabet) -> Self {
    SubstitutionKey::new(key, alphabet).into()
  }

  pub fn inverse(self) -> SubstitutionEncipherKey {
    SubstitutionKey(self.0).inverse().into()
  }
}

impl TryFrom<(&str, &Substitution)> for SubstitutionDecipherKey {
  type Error = ParseError;

  fn try_from(
    (key, context): (&str, &Substitution),
  ) -> Result<Self, Self::Error> {
    Self::try_new(key.to_string(), context)
  }
}

impl From<SubstitutionKey> for SubstitutionDecipherKey {
  fn from(value: SubstitutionKey) -> Self {
    SubstitutionDecipherKey(value.0)
  }
}

impl SubstitutionKey {
  fn try_new(key: String, context: &Substitution) -> Result<Self, ParseError> {
    match key.len().cmp(&context.alphabet.len()) {
      std::cmp::Ordering::Less => return Err(ParseError::LengthTooShort),
      std::cmp::Ordering::Greater => return Err(ParseError::LengthTooLong),
      _ => (),
    }

    if let Some(k) = key.chars().find(|&k| !context.alphabet.contains(k)) {
      return Err(ParseError::InvalidChar(k));
    }

    key
      .chars()
      .zip(context.alphabet.iter())
      .try_fold(AHashMap::new(), |mut acc, (k, v)| {
        acc
          .insert(k, v) // try_insert() would be more readable, but it is a nightly only API
          .map_or_else(|| Ok(acc), |_| Err(ParseError::DuplicateChar(k)))
      })
      .map(SubstitutionKey)
  }

  fn new(key: String, alphabet: &Alphabet) -> Self {
    SubstitutionKey(key.chars().zip(alphabet.iter()).fold(
      AHashMap::new(),
      |mut acc, (k, v)| {
        acc.insert(k, v);
        acc
      },
    ))
  }

  fn inverse(self) -> Self {
    SubstitutionKey(AHashMap::from_iter(self.0.iter().map(|(k, v)| (*v, *k))))
  }
}

impl TryFrom<(&str, &Substitution)> for SubstitutionKey {
  type Error = ParseError;

  fn try_from(
    (key, context): (&str, &Substitution),
  ) -> Result<Self, Self::Error> {
    Self::try_new(key.to_string(), context)
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
