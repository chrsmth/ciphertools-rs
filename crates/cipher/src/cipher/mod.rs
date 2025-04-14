pub mod autokey;
pub mod caesar;
pub mod substitution;
pub mod vigenere;

pub trait Encipher {
  type Key;

  fn encipher(&self, plaintext: &str, key: &Self::Key) -> String;
}

pub trait Decipher {
  type Key;

  fn decipher(&self, ciphertext: &str, key: &Self::Key) -> String;
}

pub trait PartialDecipher {
  type PartialKey;

  fn partial_decipher(
    &self,
    ciphertext: &str,
    key: &Self::PartialKey,
  ) -> String;
}

pub trait KeysIterator: Decipher {
  type KeysIter: Iterator<Item = Self::Key>;

  fn brute_force_iter(&self) -> Self::KeysIter;
}

pub trait IntoDecipherKey {
  type DecipherKey;

  fn into_decipher_key(self) -> Self::DecipherKey;
}

pub trait IntoEncipherKey {
  type EncipherKey;

  fn into_encipher_key(self) -> Self::EncipherKey;
}
