use ahash::{AHashMap, AHashSet};

use serde::{Deserialize, Serialize};

use crate::ngrams::{Ngrams, RankedNgrams};
use crate::resources;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(from = "LanguageSerializer", into = "LanguageSerializer")]
pub struct Language {
  ranked_ngrams: AHashMap<usize, RankedNgrams>,
  index_of_coincidence: f64,
}

pub struct Confidence(Box<dyn Fn(&str) -> f64>);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguageSerializer {
  ngrams: AHashMap<usize, Ngrams>,
  index_of_coincidence: f64,
}

impl From<Language> for LanguageSerializer {
  fn from(language: Language) -> Self {
    LanguageSerializer {
      ngrams: language
        .ranked_ngrams
        .into_iter()
        .map(|(n, ranked_ngrams)| (n, ranked_ngrams.into()))
        .collect(),
      index_of_coincidence: language.index_of_coincidence,
    }
  }
}

impl From<LanguageSerializer> for Language {
  fn from(language_serializer: LanguageSerializer) -> Self {
    Language {
      ranked_ngrams: language_serializer
        .ngrams
        .into_iter()
        .map(|(n, ngrams)| (n, ngrams.into()))
        .collect(),
      index_of_coincidence: language_serializer.index_of_coincidence,
    }
  }
}

impl Confidence {
  pub fn new(confidence: Box<dyn Fn(&str) -> f64>) -> Self {
    Confidence(confidence)
  }

  pub fn run(&self, text: &str) -> f64 {
    (self.0)(text)
  }
}

impl Language {
  pub fn new(
    ranked_ngrams: AHashMap<usize, RankedNgrams>,
    index_of_coincidence: f64,
  ) -> Self {
    Language {
      ranked_ngrams,
      index_of_coincidence,
    }
  }

  pub fn english() -> Self {
    serde_json::from_str(resources::ENGLISH)
      .expect("Failed to parse resources::ENGLISH")
  }

  pub fn text_confidence_chi2_unigram(&self, text: &str) -> f64 {
    self.text_confidence_chi2_ngram(text, 1)
  }
  pub fn text_confidence_chi2_bigram(&self, text: &str) -> f64 {
    self.text_confidence_chi2_ngram(text, 2)
  }
  pub fn text_confidence_chi2_trigram(&self, text: &str) -> f64 {
    self.text_confidence_chi2_ngram(text, 3)
  }

  fn text_confidence_chi2_ngram(&self, text: &str, n: usize) -> f64 {
    let ranked_ngrams = self
      .ranked_ngrams
      .get(&n)
      .expect("Language doesn't support {n}grams");

    if ranked_ngrams.is_empty() {
      return f64::MAX;
    }

    let text_trigrams = Ngrams::from_text(text, n);

    let least_common_tri = match ranked_ngrams.last() {
      Some(tri) => &tri.0,
      None => {
        return f64::MAX;
      }
    };

    let mut unpresent_sum = 0.0;
    let trigrams: AHashSet<_> = text_trigrams
      .ngrams()
      .chain(ranked_ngrams.ngrams())
      .filter(|x| *x != least_common_tri)
      .collect();

    let mut chi2_sum = 0.0;
    for tri in trigrams
      .into_iter()
      .filter(|x| x.chars().all(|y| y.is_alphabetic()))
    {
      let actual = text_trigrams.get(tri);
      let expected = ranked_ngrams.get(tri);
      if expected > 0.0 {
        chi2_sum += f64::powi(actual - expected, 2) / expected
      } else {
        unpresent_sum += actual;
      }
    }

    let actual = text_trigrams.get(least_common_tri) + unpresent_sum;
    let expected = ranked_ngrams.get(least_common_tri);
    let unpresent_chi = f64::powi(actual - expected, 2) / expected;
    chi2_sum += unpresent_chi;

    chi2_sum
  }
}
