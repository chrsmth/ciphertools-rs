use clap::Parser;

use crate::cli::{Alphabet, DictionaryOpts};

#[derive(Parser, Debug)]
pub struct VigenereOpts {
  #[arg(long, value_enum, default_value_t = Alphabet::Latin)]
  pub alphabet: Alphabet,
  #[arg(long, default_value_t = true)]
  pub skip_whitespace: bool,
  #[command(subcommand)]
  pub commands: VigenereCommands,
}

#[derive(Parser, Debug)]
pub enum VigenereCommands {
  Encipher(VigenereEncipherOpts),
  Decipher(VigenereDecipherOpts),
  Dictionary(DictionaryOpts),
}

#[derive(Parser, Debug)]
pub struct VigenereEncipherOpts {
  pub key: String,
  pub plaintext: String,
}

#[derive(Parser, Debug)]
pub struct VigenereDecipherOpts {
  pub key: String,
  pub ciphertext: String,
}
