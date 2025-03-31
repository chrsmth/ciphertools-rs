#![allow(dead_code)]
mod alphabet;
mod cipher;
mod cli;
mod language;
mod manager;
mod ngrams;
mod resources;

use crate::cipher::caesar;
use crate::cipher::vigenere;
use crate::cipher::vigenere::VigenereKey;
use crate::cipher::BruteForceIterator;
use crate::manager::Manager;
use std::fmt::Display;
use std::{io::BufRead, num::NonZeroUsize};

use clap::Parser;

use crate::{
  cipher::{Decipher, Encipher},
  language::Language,
};

fn main() {
  let cli = cli::CliOpts::parse();

  let language = Language::english();
  match cli.commands {
    cli::Commands::Vigenere(opts) => {
      let context = vigenere::Vigenere::new(opts.alphabet.into());
      match opts.commands {
        cli::vigenere::VigenereCommands::Encipher(opts) => {
          run_encipher(
            &vigenere::VigenereKey::new(opts.key),
            context,
            &opts.plaintext,
          );
        }
        cli::vigenere::VigenereCommands::Decipher(opts) => {
          run_decipher(
            &vigenere::VigenereKey::new(opts.key),
            context,
            &opts.ciphertext,
          );
        }
        cli::vigenere::VigenereCommands::DictionaryAttack(opts) => {
          let confidence = opts.confidence_algorithm.into_confidence(language);
          run_dictionary_attack(
            context,
            &opts.ciphertext,
            get_vigenere_dictionary_iter(&opts.dictionary_file),
            &mut Manager::new(NonZeroUsize::new(10).unwrap(), confidence),
          )
        }
      }
    }
    cli::Commands::Caesar(opts) => {
      let context = caesar::Caesar::new(opts.alphabet.into());
      match opts.commands {
        cli::caesar::CaesarCommands::Encipher(opts) => {
          run_encipher(
            &caesar::CaesarKey::new(opts.key),
            context,
            &opts.plaintext,
          );
        }
        cli::caesar::CaesarCommands::Decipher(opts) => {
          run_decipher(
            &caesar::CaesarKey::new(opts.key),
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

fn get_vigenere_dictionary_iter(
  dictionary_file: &str,
) -> impl Iterator<Item = VigenereKey> {
  let reader = std::io::BufReader::new(
    std::fs::File::open(dictionary_file).unwrap_or_else(|e| {
      eprintln!("Failed to open file '{}': {}", dictionary_file, e);
      std::process::exit(1);
    }),
  );

  reader.lines().filter_map(|line_result| match line_result {
    Ok(line) => Some(vigenere::VigenereKey::new(line)),
    Err(e) => {
      eprintln!("failed to parse line in dictionary: {}", e);
      None
    }
  })
}
