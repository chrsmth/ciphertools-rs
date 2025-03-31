use clap::Parser;

use crate::cli::{Alphabet, BruteForceOpts};

#[derive(Parser, Debug)]
pub struct CaesarOpts {
  #[arg(long, value_enum, default_value_t = Alphabet::Latin)]
  pub alphabet: Alphabet,
  #[command(subcommand)]
  pub commands: CaesarCommands,
}

#[derive(Parser, Debug)]
pub enum CaesarCommands {
  Encipher(CaesarEncipherOpts),
  Decipher(CaesarDecipherOpts),
  BruteForce(BruteForceOpts),
}

#[derive(Parser, Debug)]
pub struct CaesarEncipherOpts {
  pub key: char,
  pub plaintext: String,
}

#[derive(Parser, Debug)]
pub struct CaesarDecipherOpts {
  pub key: char,
  pub ciphertext: String,
}
