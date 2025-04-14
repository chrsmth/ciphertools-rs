use crate::{
  alphabet::Alphabet,
  cipher::{Decipher, Encipher},
};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter::once;
use stutter_zip::StutterZipIterator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutokeyKey(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Autokey {
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
  pub fn new(alphabet: Alphabet, skip_whitespace: bool) -> Self {
    Autokey {
      alphabet,
      skip_whitespace,
    }
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
    let key_iter = key.0.chars().chain(
      plaintext
        .chars()
        .filter(|p| !(p.is_whitespace() && self.skip_whitespace)),
    );

    plaintext
      .chars()
      .stutter_zip(key_iter, |p| p.is_whitespace() && self.skip_whitespace)
      .map(|(p, k)| k.map(|k| self.alphabet.add(p, k)).unwrap_or(p))
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
    let mut key_iter = key.0.chars();
    let mut autokey = Vec::new();
    let mut plaintext = String::new();

    let c = loop {
      let Some(c) = ciphertext_iter.next() else {
        return plaintext;
      };

      if c.is_whitespace() && self.skip_whitespace {
        plaintext.push(c);
        continue;
      }

      match key_iter.next() {
        Some(k) => {
          let p = self.alphabet.sub(c, k);
          autokey.push(p);
          plaintext.push(p)
        }
        None => {
          break c;
        }
      }
    };

    once(c)
      .chain(ciphertext_iter)
      .stutter_zip(0.., |c| c.is_whitespace() && self.skip_whitespace)
      .for_each(|(c, i)| {
        i.map(|i| {
          let p = self.alphabet.sub(c, autokey[i]);
          autokey.push(p);
          plaintext.push(p);
        })
        .unwrap_or_else(|| {
          plaintext.push(c);
        })
      });

    plaintext
  }
}

#[cfg(test)]
mod tests;
