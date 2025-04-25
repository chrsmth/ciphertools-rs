pub mod autokey;
pub mod caesar;
pub mod substitution;
pub mod vigenere;

use crate::cli::autokey::AutokeyOpts;
use crate::cli::vigenere::VigenereOpts;
use crate::cli::{caesar::CaesarOpts, substitution::SubstitutionOpts};
use cipher::alphabet;
use cipher::language::{self, Language};
use clap::{Parser, ValueEnum};
use num_cpus;
use std::sync::Arc;

#[derive(Parser, Debug)]
pub struct CliOpts {
  #[arg(short = 'j', default_value_t = num_cpus::get())]
  pub jobs: usize,
  #[clap(long, hide = true, value_enum, default_value_t = Confidence::Chi2Trigrams)]
  pub confidence_algorithm: Confidence,
  #[command(subcommand)]
  pub commands: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
  Autokey(AutokeyOpts),
  Vigenere(VigenereOpts),
  Substitution(SubstitutionOpts),
  Caesar(CaesarOpts),
}

#[derive(Parser, Debug)]
pub struct DictionaryOpts {
  pub ciphertext: String,
  pub dictionary_file: String,
}

#[derive(Parser, Debug)]
pub struct BruteForceOpts {
  pub ciphertext: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(ValueEnum, Debug, Clone)]
pub enum Confidence {
  Chi2Unigrams,
  Chi2Bigrams,
  Chi2Trigrams,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Alphabet {
  Russian,
  Greek,
  Latin,
}

impl Confidence {
  pub fn into_get_confidence(
    self,
    language: Language,
  ) -> language::GetConfidence {
    match self {
      Confidence::Chi2Unigrams => {
        language::GetConfidence::new(Arc::new(move |text: &str| {
          language.text_confidence_chi2_unigram(text)
        }))
      }
      Confidence::Chi2Bigrams => {
        language::GetConfidence::new(Arc::new(move |text: &str| {
          language.text_confidence_chi2_bigram(text)
        }))
      }
      Confidence::Chi2Trigrams => {
        language::GetConfidence::new(Arc::new(move |text: &str| {
          language.text_confidence_chi2_bigram(text)
        }))
      }
    }
  }
}

impl From<Alphabet> for alphabet::Alphabet {
  fn from(val: Alphabet) -> Self {
    match val {
      Alphabet::Russian => alphabet::Alphabet::russian(),
      Alphabet::Greek => alphabet::Alphabet::greek(),
      Alphabet::Latin => alphabet::Alphabet::latin(),
    }
  }
}
