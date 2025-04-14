use crate::alphabet::Alphabet;
use crate::substitution::Substitution;
use ahash::AHashMap;
use std::fmt;

struct Quipqiup {
  table: AHashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct InvalidCharError(char);

#[derive(Debug)]
pub struct QuipqiupNewError {
  c: char,
  word: String,
}

impl fmt::Display for QuipqiupNewError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let c = self.c;
    let word = &self.word;
    write!(f, "Invalid character '{c}' in {word}")
  }
}

impl Quipqiup {
  fn new(
    words: Vec<String>,
    alphabet: &Alphabet,
  ) -> Result<Self, QuipqiupNewError> {
    let mut table: AHashMap<String, Vec<String>> = AHashMap::new();
    for word in words {
      let normalized =
        Self::try_normalize_word(&word, alphabet).map_err(|err| {
          QuipqiupNewError {
            c: err.0,
            word: word.clone(),
          }
        })?;

      table.entry(normalized).or_default().push(word);
    }

    Ok(Quipqiup { table })
  }

  fn try_normalize_word(
    word: &str,
    alphabet: &Alphabet,
  ) -> Result<String, InvalidCharError> {
    let mut normalization: AHashMap<char, char> = AHashMap::new();
    let mut alphabet_iter = alphabet.iter();
    let mut normalized = String::new();

    for c in word.chars() {
      if !alphabet.contains(c) {
        return Err(InvalidCharError(c));
      }

      normalized.push(match normalization.get(&c) {
        Some(a) => *a,
        None => {
          let normalized = alphabet_iter.next().unwrap();
          normalization.insert(c, normalized);
          normalized
        }
      });
    }

    Ok(normalized)
  }
}

fn _substitution_quipqiup(
  _cipher_context: Substitution,
  _quipqiup: Quipqiup,
  _ciphertext: &str,
) {
}
