use clap::Parser;

use crate::cli::{Alphabet, DictionaryOpts};

#[derive(Parser, Debug)]
pub struct AutokeyOpts {
  #[arg(long, value_enum, default_value_t = Alphabet::Latin)]
  pub alphabet: Alphabet,
  #[arg(long, default_value_t = true)]
  pub skip_whitespace: bool,
  #[command(subcommand)]
  pub commands: AutokeyCommands,
}

#[derive(Parser, Debug)]
pub enum AutokeyCommands {
  Encipher(AutokeyEncipherOpts),
  Decipher(AutokeyDecipherOpts),
  Dictionary(DictionaryOpts),
}

#[derive(Parser, Debug)]
pub struct AutokeyEncipherOpts {
  pub key: String,
  pub plaintext: String,
}

#[derive(Parser, Debug)]
pub struct AutokeyDecipherOpts {
  pub key: String,
  pub ciphertext: String,
}
