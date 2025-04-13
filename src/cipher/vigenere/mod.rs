use crate::deferred_zip::StutterZipIterator;
use crate::{
  alphabet::Alphabet,
  cipher::{Decipher, Encipher},
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VigenereKey(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vigenere {
  alphabet: Alphabet,
  skip_whitespace: bool,
}

#[derive(Debug)]
pub enum ParseError {
  InvalidChar(char),
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ParseError::InvalidChar(c) => write!(f, "Invalid character: '{c}'"),
    }
  }
}

impl VigenereKey {
  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn try_new(key: String, context: &Vigenere) -> Result<Self, ParseError> {
    if let Some(k) = key.chars().find(|&k| !context.alphabet.contains(k)) {
      return Err(ParseError::InvalidChar(k));
    }

    Ok(VigenereKey::new(key))
  }

  pub fn new(key: String) -> Self {
    VigenereKey(key)
  }
}

impl TryFrom<(&str, &Vigenere)> for VigenereKey {
  type Error = ParseError;

  fn try_from((key, context): (&str, &Vigenere)) -> Result<Self, Self::Error> {
    Self::try_new(key.to_string(), context)
  }
}

impl Vigenere {
  pub fn new(alphabet: Alphabet, skip_whitespace: bool) -> Self {
    Vigenere {
      alphabet,
      skip_whitespace,
    }
  }
}

impl std::fmt::Display for VigenereKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Encipher for Vigenere {
  type Key = VigenereKey;

  fn encipher(&self, plaintext: &str, key: &Self::Key) -> String {
    if key.is_empty() {
      return plaintext.to_string();
    }

    plaintext
      .chars()
      .stutter_zip(key.0.chars().cycle(), |p| {
        p.is_whitespace() && self.skip_whitespace
      })
      .map(|(p, k)| k.map(|k| self.alphabet.add(p, k)).unwrap_or(p))
      .collect()
  }
}

impl Decipher for Vigenere {
  type Key = VigenereKey;

  fn decipher(&self, ciphertext: &str, key: &Self::Key) -> String {
    if key.is_empty() {
      return ciphertext.to_string();
    }

    ciphertext
      .chars()
      .stutter_zip(key.0.chars().cycle(), |c| {
        c.is_whitespace() && self.skip_whitespace
      })
      .map(|(c, k)| k.map(|k| self.alphabet.sub(c, k)).unwrap_or(c))
      .collect()
  }
}

#[cfg(test)]
mod tests;
