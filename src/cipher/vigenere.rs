use crate::{
  alphabet::Alphabet,
  cipher::{Decipher, Encipher},
};

pub struct VigenereKey(String);

#[derive(Default)]
pub struct Vigenere {
  alphabet: Alphabet,
}

impl VigenereKey {
  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn new(key: String) -> Self {
    VigenereKey(key)
  }
}

impl Vigenere {
  pub fn new(alphabet: Alphabet) -> Self {
    Vigenere { alphabet }
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
      .zip(key.0.chars().cycle())
      .map(|(c, k)| self.alphabet.add(c, k))
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
      .zip(key.0.chars().cycle())
      .map(|(c, k)| self.alphabet.sub(c, k))
      .collect()
  }
}
