use super::Substitution;
use crate::alphabet::Alphabet;
use ahash::AHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuipqiupContext {
  table: AHashMap<String, Vec<String>>,
}

impl QuipqiupContext {
  pub fn new(words: Vec<String>) -> Self {
    let mut table: AHashMap<String, Vec<String>> = AHashMap::new();
    for word in words {
      let normalized = Self::normalize_word(&word);
      table.entry(normalized).or_default().push(word);
    }

    QuipqiupContext { table }
  }

  fn normalize_word(word: &str) -> String {
    let mut normalization: AHashMap<char, char> = AHashMap::new();
    let latin = Alphabet::latin();
    let mut alphabet_iter = latin.iter();
    let mut normalized = String::new();

    for c in word.chars() {
      normalized.push(match normalization.get(&c) {
        Some(a) => *a,
        None => {
          let normalized = alphabet_iter.next().unwrap();
          normalization.insert(c, normalized);
          normalized
        }
      });
    }

    normalized
  }
}

impl Substitution {
  pub fn quipqiup(&self, _quipqiup_context: QuipqiupContext, ciphertext: &str) {
    let _words: Vec<_> = ciphertext.split_whitespace().collect();
    //TODO
  }
}
