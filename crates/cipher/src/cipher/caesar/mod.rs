use crate::{
  alphabet::Alphabet,
  cipher::{Decipher, Encipher, KeysIterator},
};
use cipher_derive::{IntoDecipherKey, IntoEncipherKey};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
  Debug, Clone, Serialize, Deserialize, IntoDecipherKey, IntoEncipherKey,
)]
pub struct CaesarKey(char);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Caesar {
  alphabet: Alphabet,
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

impl CaesarKey {
  pub fn try_new(key: char, context: &Caesar) -> Result<Self, ParseError> {
    if !context.alphabet.contains(key) {
      return Err(ParseError::InvalidChar(key));
    }

    Ok(Self::new(key))
  }

  pub fn new(key: char) -> Self {
    CaesarKey(key)
  }
}

impl Caesar {
  pub fn new(alphabet: Alphabet) -> Self {
    Caesar { alphabet }
  }
}

impl std::fmt::Display for CaesarKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Encipher for Caesar {
  type Key = CaesarKey;

  fn encipher(&self, plaintext: &str, key: &Self::Key) -> String {
    plaintext
      .chars()
      .map(|c| self.alphabet.add(c, key.0))
      .collect()
  }
}

impl Decipher for Caesar {
  type Key = CaesarKey;

  fn decipher(&self, ciphertext: &str, key: &Self::Key) -> String {
    ciphertext
      .chars()
      .map(|c| self.alphabet.sub(c, key.0))
      .collect()
  }
}

impl KeysIterator for Caesar {
  type KeysIter =
    <std::vec::Vec<CaesarKey> as std::iter::IntoIterator>::IntoIter;

  fn keys_iter(&self) -> Self::KeysIter {
    let a: Vec<CaesarKey> = self.alphabet.iter().map(CaesarKey::new).collect();

    a.into_iter()
  }
}

#[cfg(test)]
mod tests;
