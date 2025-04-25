mod cli;
mod scoreboard;
mod threads;

use crate::cli::CliOpts;
use crate::scoreboard::Scoreboard;
use cipher::cipher::{
  Decipher, Encipher, IntoDecipherKey, KeysIterator, autokey, caesar,
  substitution, vigenere,
};
use cipher::language::{GetConfidence, Language};
use clap::Parser;
use crossbeam::channel::Sender;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::fmt::Display;
use std::fs::File;
use std::sync::Arc;
use std::{
  io::{BufRead, BufReader},
  num::NonZeroUsize,
};
use threads::{CandidateCollectorMsg, spawn_candidate_collector};

struct CiphertoolsContext {
  get_confidence: GetConfidence,
  pool: ThreadPool,
}

fn main() {
  let cli = cli::CliOpts::parse();
  run(cli).unwrap_or_else(|e| {
    eprintln!("{e}");
    std::process::exit(1);
  });
}

fn run(opts: CliOpts) -> Result<(), String> {
  let language = Language::english();
  let pool = ThreadPoolBuilder::new()
    .num_threads(opts.jobs)
    .build()
    .unwrap_or_else(|e| {
      eprintln!("Failed to build thread pool: {e}");
      std::process::exit(1);
    });

  let ciphertools_context = CiphertoolsContext {
    get_confidence: opts
      .confidence_algorithm
      .into_get_confidence(language.clone()),
    pool,
  };

  match opts.commands {
    cli::Commands::Autokey(opts) => {
      let context =
        autokey::Autokey::new(opts.alphabet.into(), opts.skip_whitespace);
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
          let (tx, candidate_collector_handle) =
            spawn_candidate_collector(ciphertools_context);
          let dictionary_iter =
            get_dictionary_iter(opts.dictionary_file, context.clone())?;
          run_dictionary_attack(context, opts.ciphertext, dictionary_iter, tx);

          let _ = candidate_collector_handle.join();
        }
      }
    }
    cli::Commands::Vigenere(opts) => {
      let context =
        vigenere::Vigenere::new(opts.alphabet.into(), opts.skip_whitespace);
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
          let (tx, candidate_collector_handle) =
            spawn_candidate_collector(ciphertools_context);
          let dictionary_iter =
            get_dictionary_iter(opts.dictionary_file, context.clone())?;
          run_dictionary_attack(context, opts.ciphertext, dictionary_iter, tx);

          let _ = candidate_collector_handle.join();
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
          .into_decipher_key();
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
          run_brute_force(
            context,
            &opts.ciphertext,
            &mut Scoreboard::new(
              NonZeroUsize::new(10).unwrap(),
              ciphertools_context.get_confidence,
            ),
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

fn run_brute_force<D>(context: D, ciphertext: &str, scoreboard: &mut Scoreboard)
where
  D: KeysIterator + Decipher,
  D::Key: Display,
{
  for key in context.keys_iter() {
    scoreboard.insert(context.decipher(ciphertext, &key), format!("{}", key));
  }

  scoreboard.display_scoreboard();
}

fn run_dictionary_attack<D>(
  cipher: D,
  ciphertext: String,
  dictionary: impl Iterator<Item = D::Key>,
  tx: Sender<CandidateCollectorMsg>,
) where
  D: Decipher + std::marker::Sync + std::marker::Send,
  D::Key: Display + Send + Sync,
{
  let cipher = Arc::new(cipher);
  let ciphertext = Arc::new(ciphertext);
  dictionary.for_each(|key| {
    let text = cipher.decipher(&ciphertext, &key);
    let key = format!("{}", key);
    let _ = tx.send(CandidateCollectorMsg::CandidatePlaintext { text, key });
  });
}

fn get_dictionary_iter<K, C>(
  dictionary_file: String,
  context: C,
) -> Result<impl Iterator<Item = K>, String>
where
  for<'a> K: TryFrom<(&'a str, &'a C)>,
  for<'a> <K as TryFrom<(&'a str, &'a C)>>::Error: std::fmt::Display,
{
  let file = File::open(&dictionary_file)
    .map_err(|e| format!("Failed to open file '{}': {}", dictionary_file, e))?;
  let reader = BufReader::new(file);

  let iter = reader.lines().filter_map(move |line_res| match line_res {
    Ok(line) => match K::try_from((&line, &context)) {
      Ok(key) => Some(key),
      Err(e) => {
        eprintln!("Failed to parse line in dictionary: {e}");
        None
      }
    },
    Err(e) => {
      eprintln!("Failed to parse line in dictionary: {e}");
      None
    }
  });

  Ok(iter)
}
