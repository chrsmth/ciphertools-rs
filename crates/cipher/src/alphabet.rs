use ahash::AHashMap;
use serde::{Deserialize, Serialize};

static LATIN: &[char; 26] = &[
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
  'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static RUSSIAN: &[char; 33] = &[
  'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н',
  'о', 'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь',
  'э', 'ю', 'я',
];

static GREEK: &[char; 24] = &[
  'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο',
  'π', 'ρ', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω',
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "AlphabetDeserializer", into = "AlphabetSerializer")]
pub struct Alphabet {
  chars: Vec<char>,
  indexes: AHashMap<char, usize>,
}

#[derive(Debug, Clone, Serialize)]
enum AlphabetSerializer {
  Chars(String),
}

#[derive(Debug, Clone, Deserialize)]
enum AlphabetDeserializer {
  Chars(String),
  Latin,
  Russian,
  Greek,
}

impl From<AlphabetDeserializer> for Alphabet {
  fn from(value: AlphabetDeserializer) -> Self {
    match value {
      AlphabetDeserializer::Chars(cs) => Alphabet::from_iter(cs.chars()),
      AlphabetDeserializer::Latin => Alphabet::latin(),
      AlphabetDeserializer::Russian => Alphabet::russian(),
      AlphabetDeserializer::Greek => Alphabet::greek(),
    }
  }
}

impl From<Alphabet> for AlphabetSerializer {
  fn from(value: Alphabet) -> Self {
    AlphabetSerializer::Chars(value.iter().collect())
  }
}

impl Alphabet {
  pub fn russian() -> Alphabet {
    Alphabet::from_iter(RUSSIAN.iter().cloned())
  }

  pub fn greek() -> Alphabet {
    Alphabet::from_iter(GREEK.iter().cloned())
  }

  pub fn latin() -> Alphabet {
    Alphabet::from_iter(LATIN.iter().cloned())
  }

  pub fn iter(&self) -> impl Iterator<Item = char> + use<'_> {
    self.chars.iter().copied()
  }
}

impl Default for Alphabet {
  fn default() -> Self {
    Alphabet::latin()
  }
}

impl FromIterator<char> for Alphabet {
  fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Alphabet {
    let chars: Vec<_> = iter.into_iter().collect();
    let indexes = AHashMap::from_iter(
      chars.iter().cloned().enumerate().map(|(i, c)| (c, i)),
    );

    Alphabet { chars, indexes }
  }
}

impl Alphabet {
  pub fn get(&self, i: usize) -> Option<char> {
    self.chars.get(i).map(|x| x.to_owned())
  }

  pub fn get_index(&self, c: char) -> Option<usize> {
    self.indexes.get(&c).map(|x| x.to_owned())
  }

  #[allow(clippy::len_without_is_empty)]
  pub fn len(&self) -> usize {
    self.chars.len()
  }

  pub fn contains(&self, c: char) -> bool {
    self.indexes.contains_key(&c)
  }

  pub fn all(&self, s: String) -> bool {
    s.chars().all(|c| self.contains(c))
  }

  pub fn add(&self, a: char, b: char) -> char {
    self
      .get_index(a)
      .zip(self.get_index(b))
      .and_then(|(ai, bi)| self.get((ai + bi) % self.len()))
      .unwrap_or(a)
  }

  pub fn sub(&self, a: char, b: char) -> char {
    self
      .get_index(a)
      .zip(self.get_index(b))
      .and_then(|(ai, bi)| self.get((self.len() + ai - bi) % self.len()))
      .unwrap_or(a)
  }
}
