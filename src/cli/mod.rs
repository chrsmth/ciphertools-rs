pub mod autokey;
pub mod caesar;
pub mod substitution;
pub mod vigenere;

use crate::cli::autokey::AutokeyOpts;
use crate::cli::vigenere::VigenereOpts;
use crate::cli::{caesar::CaesarOpts, substitution::SubstitutionOpts};

use clap::{Parser, ValueEnum};

use crate::{
  alphabet,
  language::{self, Language},
};

#[derive(Parser, Debug)]
pub struct CliOpts {
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
  #[clap(long, hide = true, value_enum, default_value_t = Confidence::Chi2Trigrams)]
  pub confidence_algorithm: Confidence,
}

#[derive(Parser, Debug)]
pub struct BruteForceOpts {
  pub ciphertext: String,
  #[clap(long, hide = true, value_enum, default_value_t = Confidence::Chi2Trigrams)]
  pub confidence_algorithm: Confidence,
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
  pub fn into_confidence(self, language: Language) -> language::Confidence {
    match self {
      Confidence::Chi2Unigrams => {
        language::Confidence::new(Box::new(move |text: &str| {
          language.text_confidence_chi2_unigram(text)
        }))
      }
      Confidence::Chi2Bigrams => {
        language::Confidence::new(Box::new(move |text: &str| {
          language.text_confidence_chi2_bigram(text)
        }))
      }
      Confidence::Chi2Trigrams => {
        language::Confidence::new(Box::new(move |text: &str| {
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
