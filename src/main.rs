#![allow(dead_code)]
mod alphabet;
mod cipher;
mod cli;
mod language;
mod manager;
mod ngrams;
mod resources;

use crate::cipher::autokey;
use crate::cipher::caesar;
use crate::cipher::substitution;
use crate::cipher::vigenere;
use crate::cipher::BruteForceIterator;
use crate::cli::CliOpts;
use crate::manager::Manager;
use std::fmt::Display;
use std::fs::File;
use std::{
  io::{BufRead, BufReader},
  num::NonZeroUsize,
};

use clap::Parser;

use crate::{
  cipher::{Decipher, Encipher},
  language::Language,
};

fn main() {
  let cli = cli::CliOpts::parse();
  run(cli).unwrap_or_else(|e| {
    eprintln!("{e}");
    std::process::exit(1);
  });
}

fn run(cli: CliOpts) -> Result<(), String> {
  let language = Language::english();
  match cli.commands {
    cli::Commands::Autokey(opts) => {
      let context = autokey::Autokey::new(opts.alphabet.into());
      match opts.commands {
        cli::autokey::AutokeyCommands::Encipher(opts) => {
          run_encipher(
            &autokey::AutokeyKey::try_new(opts.key, &context)
              .map_err(|e| format!("Failed to parse key: {e}"))?,
            context,
            &opts.plaintext,
          );
        }
        cli::autokey::AutokeyCommands::Decipher(opts) => {
          run_decipher(
            &autokey::AutokeyKey::try_new(opts.key, &context)
              .map_err(|e| format!("Failed to parse key: {e}"))?,
            context,
            &opts.ciphertext,
          );
        }
        cli::autokey::AutokeyCommands::Dictionary(opts) => {
          let confidence = opts.confidence_algorithm.into_confidence(language);
          let dictionary_iter =
            get_dictionary_iter(&opts.dictionary_file, context.clone())?;
          run_dictionary_attack(
            context,
            &opts.ciphertext,
            dictionary_iter,
            &mut Manager::new(NonZeroUsize::new(10).unwrap(), confidence),
          )
        }
      }
    }
    cli::Commands::Vigenere(opts) => {
      let context = vigenere::Vigenere::new(opts.alphabet.into());
      match opts.commands {
        cli::vigenere::VigenereCommands::Encipher(opts) => {
          run_encipher(
            &vigenere::VigenereKey::try_new(opts.key, &context)
              .map_err(|e| format!("Failed to parse key: {e}"))?,
            context,
            &opts.plaintext,
          );
        }
        cli::vigenere::VigenereCommands::Decipher(opts) => {
          run_decipher(
            &vigenere::VigenereKey::try_new(opts.key, &context)
              .map_err(|e| format!("Failed to parse key: {e}"))?,
            context,
            &opts.ciphertext,
          );
        }
        cli::vigenere::VigenereCommands::Dictionary(opts) => {
          let confidence = opts.confidence_algorithm.into_confidence(language);
          let dictionary_iter =
            get_dictionary_iter(&opts.dictionary_file, context.clone())?;
          run_dictionary_attack(
            context,
            &opts.ciphertext,
            dictionary_iter,
            &mut Manager::new(NonZeroUsize::new(10).unwrap(), confidence),
          )
        }
      }
    }
    cli::Commands::Substitution(opts) => {
      let context = substitution::Substitution::new(opts.alphabet.into());
      match opts.commands {
        cli::substitution::SubstitutionCommands::Encipher(opts) => {
          let key = substitution::SubstitutionEncipherKey::try_from((
            opts.key.as_str(),
            &context,
          ))
          .map_err(|e| format!("Failed to parse key: {e}"))?;
          run_encipher(&key, context, &opts.plaintext);
        }
        cli::substitution::SubstitutionCommands::Decipher(opts) => {
          let key = substitution::SubstitutionEncipherKey::try_from((
            opts.key.as_str(),
            &context,
          ))
          .map_err(|e| format!("Failed to parse key: {e}"))?
          .inverse();
          run_decipher(&key, context, &opts.ciphertext);
        }
      }
    }
    cli::Commands::Caesar(opts) => {
      let context = caesar::Caesar::new(opts.alphabet.into());
      match opts.commands {
        cli::caesar::CaesarCommands::Encipher(opts) => {
          run_encipher(
            &caesar::CaesarKey::try_new(opts.key, &context)
              .map_err(|e| format!("Failed to parse key: {e}"))?,
            context,
            &opts.plaintext,
          );
        }
        cli::caesar::CaesarCommands::Decipher(opts) => {
          run_decipher(
            &caesar::CaesarKey::try_new(opts.key, &context)
              .map_err(|e| format!("Failed to parse key: {e}"))?,
            context,
            &opts.ciphertext,
          );
        }
        cli::caesar::CaesarCommands::BruteForce(opts) => {
          let confidence = opts.confidence_algorithm.into_confidence(language);
          run_brute_force(
            context,
            &opts.ciphertext,
            &mut Manager::new(NonZeroUsize::new(10).unwrap(), confidence),
          );
        }
      }
    }
  }
  Ok(())
}
fn run_encipher<E: Encipher>(key: &E::Key, context: E, plaintext: &str) {
  let result = context.encipher(plaintext, key);
  println!("{}", result);
}

fn run_decipher<D: Decipher>(key: &D::Key, context: D, ciphertext: &str) {
  let result = context.decipher(ciphertext, key);
  println!("{}", result);
}

fn run_brute_force<D>(context: D, ciphertext: &str, manager: &mut Manager)
where
  D: BruteForceIterator + Decipher,
  D::Key: Display,
{
  for key in context.brute_force_iter() {
    manager.insert(context.decipher(ciphertext, &key), format!("{}", key));
  }

  manager.display_scoreboard();
}

fn run_dictionary_attack<D>(
  cipher: D,
  ciphertext: &str,
  dictionary: impl Iterator<Item = D::Key>,
  manager: &mut Manager,
) where
  D: Decipher,
  D::Key: Display,
{
  for key in dictionary {
    manager.insert(cipher.decipher(ciphertext, &key), format!("{}", key));
  }

  manager.display_scoreboard();
}

fn get_dictionary_iter<K, C>(
  dictionary_file: &str,
  context: C,
) -> Result<impl Iterator<Item = K>, String>
where
  for<'a> K: TryFrom<(&'a str, &'a C)>,
  for<'a> <K as TryFrom<(&'a str, &'a C)>>::Error: std::fmt::Display,
{
  let file = File::open(dictionary_file)
    .map_err(|e| format!("Failed to open file '{}': {}", dictionary_file, e))?;
  let reader = BufReader::new(file);

  let iter = reader.lines().filter_map(move |line_res| match line_res {
    Ok(line) => match K::try_from((&line, &context)) {
      Ok(key) => Some(key),
      Err(e) => {
        eprintln!("failed to parse line in dictionary: {e}");
        None
      }
    },
    Err(e) => {
      eprintln!("failed to parse line in dictionary: {e}");
      None
    }
  });

  Ok(iter)
}
