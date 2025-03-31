use ahash::AHashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ngrams(AHashMap<String, f64>);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RankedNgrams {
  ngrams: AHashMap<String, f64>,
  ranks: Vec<(String, f64)>,
}

impl FromIterator<(String, f64)> for Ngrams {
  fn from_iter<I: IntoIterator<Item = (String, f64)>>(iter: I) -> Self {
    Ngrams(AHashMap::from_iter(iter))
  }
}

impl FromIterator<(String, u64)> for Ngrams {
  fn from_iter<I: IntoIterator<Item = (String, u64)>>(iter: I) -> Self {
    let (ngram, count): (Vec<_>, Vec<_>) = iter.into_iter().unzip();
    let sum: u64 = count.iter().sum();

    let iter = ngram
      .into_iter()
      .zip(count.into_iter().map(|c| (c as f64) / (sum as f64)));

    Ngrams::from_iter(iter)
  }
}

impl FromIterator<(String, f64)> for RankedNgrams {
  fn from_iter<I: IntoIterator<Item = (String, f64)>>(iter: I) -> Self {
    Ngrams::from_iter(iter).into()
  }
}

impl FromIterator<(String, u64)> for RankedNgrams {
  fn from_iter<I: IntoIterator<Item = (String, u64)>>(iter: I) -> Self {
    Ngrams::from_iter(iter).into()
  }
}

impl From<RankedNgrams> for Ngrams {
  fn from(ranked_ngrams: RankedNgrams) -> Self {
    Ngrams(ranked_ngrams.ngrams)
  }
}

impl From<Ngrams> for RankedNgrams {
  fn from(ngrams: Ngrams) -> Self {
    let mut ngrams_vec: Vec<_> = ngrams.iter().collect();
    ngrams_vec.sort_by(|a, b| {
      a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)
    });
    let ranked = ngrams_vec
      .into_iter()
      .map(|(ngram, freq)| (ngram.clone(), *freq))
      .collect();

    RankedNgrams {
      ngrams: ngrams.0,
      ranks: ranked,
    }
  }
}

impl Ngrams {
  pub fn get(&self, ngram: &str) -> f64 {
    self.0.get(ngram).copied().unwrap_or(0.0)
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn ngrams(&self) -> impl Iterator<Item = &String> {
    self.0.keys()
  }

  pub fn iter(&self) -> impl Iterator<Item = (&String, &f64)> {
    self.0.iter()
  }

  pub fn from_text(text: &str, ngrams: usize) -> Self {
    //TODO collecting into a vec is not efficient. Could try a ring buffer
    let sample: Vec<_> = text.chars().collect();
    let windows = sample.windows(ngrams).map(|x| x.iter().collect::<String>());

    let (ngrams, sum) =
      windows
        .into_iter()
        .fold((AHashMap::new(), 0), |(mut ngrams, sum), s| {
          *ngrams.entry(s.to_string()).or_insert(0) += 1;
          (ngrams, sum + 1)
        });

    let ngrams: AHashMap<String, f64> = ngrams
      .into_iter()
      .map(|(ngram, count)| (ngram, (count as f64) / (sum as f64)))
      .collect();

    Ngrams(ngrams)
  }
}

impl RankedNgrams {
  pub fn get(&self, ngram: &str) -> f64 {
    self.ngrams.get(ngram).copied().unwrap_or(0.0)
  }

  pub fn is_empty(&self) -> bool {
    self.ranks.is_empty()
  }

  pub fn ngrams(&self) -> impl Iterator<Item = &String> {
    self.ranks.iter().map(|x| &x.0)
  }

  pub fn iter(&self) -> impl Iterator<Item = &(String, f64)> {
    self.ranks.iter()
  }

  pub fn first(&self) -> Option<&(String, f64)> {
    self.ranks.first()
  }

  pub fn last(&self) -> Option<&(String, f64)> {
    self.ranks.last()
  }

  pub fn from_text(text: &str, ngrams: usize) -> Self {
    Ngrams::from_text(text, ngrams).into()
  }
}
