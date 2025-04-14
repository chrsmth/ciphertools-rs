use clap::Parser;

use crate::cli::Alphabet;

#[derive(Parser, Debug)]
pub struct SubstitutionOpts {
  #[arg(long, value_enum, default_value_t = Alphabet::Latin)]
  pub alphabet: Alphabet,
  #[command(subcommand)]
  pub commands: SubstitutionCommands,
}

#[derive(Parser, Debug)]
pub enum SubstitutionCommands {
  Encipher(SubstitutionEncipherOpts),
  Decipher(SubstitutionDecipherOpts),
}

#[derive(Parser, Debug)]
pub struct SubstitutionEncipherOpts {
  pub key: String,
  pub plaintext: String,
}

#[derive(Parser, Debug)]
pub struct SubstitutionDecipherOpts {
  pub key: String,
  pub ciphertext: String,
}
