pub mod caesar;
pub mod vigenere;

pub trait Encipher {
  type Key;

  fn encipher(&self, plaintext: &str, key: &Self::Key) -> String;
}

pub trait Decipher {
  type Key;

  fn decipher(&self, ciphertext: &str, key: &Self::Key) -> String;
}

//TODO better name for this
pub trait BruteForceIterator: Decipher {
  type BruteForceIter: Iterator<Item = Self::Key>;

  fn brute_force_iter(&self) -> Self::BruteForceIter;
}
