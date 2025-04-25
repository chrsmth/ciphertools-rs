use std::cmp::Ordering;
use std::fmt;
use std::sync::Mutex;
use std::{collections::BTreeSet, num::NonZeroUsize};

use cipher::language::GetConfidence;

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

pub struct Scoreboard {
  scoreboard: Mutex<BTreeSet<CandidatePlaintext>>,
  confidence: GetConfidence,
  len: usize,
}

impl Scoreboard {
  pub fn new(len: NonZeroUsize, confidence: GetConfidence) -> Self {
    Scoreboard {
      scoreboard: Mutex::new(BTreeSet::new()),
      confidence,
      len: len.into(),
    }
  }

  pub fn display_scoreboard(&self) {
    self
      .scoreboard
      .lock()
      .unwrap()
      .iter()
      .for_each(|score| println!("{}", score));
  }

  pub fn insert_with_confidence(
    &self,
    text: String,
    key: String,
    confidence: f64,
  ) {
    let candidate = CandidatePlaintext {
      confidence,
      text,
      key,
    };

    let mut scoreboard = self.scoreboard.lock().unwrap();
    if scoreboard.len() < self.len {
      scoreboard.insert(candidate);
    } else {
      let Some(last) = scoreboard.last() else {
        log::error!(
          "Failed to insert candidate: scoreboard should not be empty"
        );
        return;
      };
      if last.confidence > candidate.confidence {
        scoreboard.insert(candidate);
      }
    }

    if scoreboard.len() > self.len {
      scoreboard.pop_last();
    }
  }

  pub fn insert(&self, text: String, key: String) {
    let confidence = self.confidence.run(&text);
    self.insert_with_confidence(text, key, confidence);
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
