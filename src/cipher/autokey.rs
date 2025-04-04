use crate::{
  alphabet::Alphabet,
  cipher::{Decipher, Encipher},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct AutokeyKey(String); //TODO try_new that errors on invalid chars

#[derive(Debug, Clone)]
pub struct Autokey {
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

impl AutokeyKey {
  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn try_new(key: String, context: &Autokey) -> Result<Self, ParseError> {
    if let Some(k) = key.chars().find(|&k| !context.alphabet.contains(k)) {
      return Err(ParseError::InvalidChar(k));
    }

    Ok(AutokeyKey::new(key))
  }

  pub fn new(key: String) -> Self {
    AutokeyKey(key)
  }
}

impl Autokey {
  pub fn new(alphabet: Alphabet) -> Self {
    Autokey { alphabet }
  }
}

impl TryFrom<(&str, &Autokey)> for AutokeyKey {
  type Error = ParseError;

  fn try_from((key, context): (&str, &Autokey)) -> Result<Self, Self::Error> {
    Self::try_new(key.to_string(), context)
  }
}

impl std::fmt::Display for AutokeyKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Encipher for Autokey {
  type Key = AutokeyKey;

  fn encipher(&self, plaintext: &str, key: &Self::Key) -> String {
    if key.is_empty() {
      return plaintext.to_string();
    }

    plaintext
      .chars()
      .zip(key.0.chars().chain(plaintext.chars()))
      .map(|(c, k)| self.alphabet.add(c, k))
      .collect()
  }
}

impl Decipher for Autokey {
  type Key = AutokeyKey;

  fn decipher(&self, ciphertext: &str, key: &Self::Key) -> String {
    if key.is_empty() {
      return ciphertext.to_string();
    }

    let mut ciphertext_iter = ciphertext.chars();
    let mut autokey = Vec::new();

    for k in key.0.chars() {
      if let Some(c) = ciphertext_iter.next() {
        autokey.push(self.alphabet.sub(c, k));
      } else {
        return autokey.into_iter().collect();
      }
    }

    for (i, c) in ciphertext_iter.enumerate() {
      autokey.push(self.alphabet.sub(c, autokey[i]));
    }

    autokey.iter().collect()
  }
}
