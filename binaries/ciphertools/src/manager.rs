use std::cmp::Ordering;
use std::fmt;
use std::{collections::BTreeSet, num::NonZeroUsize};

use cipher::language::Confidence;

#[derive(Debug)]
struct CandidatePlaintext {
  confidence: f64,
  text: String,
  key: String,
}

impl PartialEq for CandidatePlaintext {
  fn eq(&self, other: &Self) -> bool {
    self.confidence == other.confidence
  }
}

impl Eq for CandidatePlaintext {}

impl PartialOrd for CandidatePlaintext {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for CandidatePlaintext {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .confidence
      .partial_cmp(&other.confidence)
      .unwrap_or(Ordering::Equal)
  }
}

pub struct Manager {
  scoreboard: BTreeSet<CandidatePlaintext>,
  confidence: Confidence,
  len: usize,
}

impl Manager {
  pub fn new(len: NonZeroUsize, confidence: Confidence) -> Self {
    Manager {
      scoreboard: BTreeSet::new(),
      confidence,
      len: len.into(),
    }
  }

  pub fn display_scoreboard(&self) {
    self
      .scoreboard
      .iter()
      .for_each(|score| println!("{}", score));
  }

  pub fn insert(&mut self, text: String, key: String) {
    let confidence = self.confidence.run(&text);
    let candidate = CandidatePlaintext {
      confidence,
      text,
      key,
    };

    if self.scoreboard.len() < self.len {
      self.scoreboard.insert(candidate);
    } else {
      let Some(last) = self.scoreboard.last() else {
        log::error!(
          "Failed to insert candidate: scoreboard should not be empty"
        );
        return;
      };
      if last.confidence > candidate.confidence {
        self.scoreboard.insert(candidate);
      }
    }

    if self.scoreboard.len() > self.len {
      self.scoreboard.pop_last();
    }
  }
}

impl fmt::Display for CandidatePlaintext {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Confidence: {:.4}, Key: {}, Text: {}",
      self.confidence, self.key, self.text
    )
  }
}
