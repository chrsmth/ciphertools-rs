use crate::{
  alphabet::Alphabet,
  cipher::{BruteForceIterator, Decipher, Encipher},
};

pub struct CaesarKey(char);

#[derive(Default)]
pub struct Caesar {
  alphabet: Alphabet,
}

impl CaesarKey {
  pub fn new(shift: char) -> Self {
    CaesarKey(shift)
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

//TODO maybe theres a nicer way to do this iterator
impl BruteForceIterator for Caesar {
  type BruteForceIter =
    <std::vec::Vec<CaesarKey> as std::iter::IntoIterator>::IntoIter;

  fn brute_force_iter(&self) -> Self::BruteForceIter {
    let a: Vec<CaesarKey> = self.alphabet.iter().map(CaesarKey::new).collect();

    a.into_iter()
  }
}
