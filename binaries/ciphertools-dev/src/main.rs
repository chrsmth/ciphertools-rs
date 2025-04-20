#![allow(dead_code)]
use crate::ngrams::RankedNgrams;
use ahash::AHashMap;
use cipher::language::Language;
use cipher::ngrams;
use cipher::quipqiup::QuipqiupContext;
use clap::Parser;
use csv::Reader;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct CliOpts {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Parser)]
enum Commands {
  BuildLanguage(BuildLanguage),
  TextConfidence(TextConfidence),
  BuildQuipqiupContext(BuildQuipqiupContext),
}

#[derive(Parser)]
struct BuildLanguage {
  pub word_ngrams: String,
  pub unigrams_filepath: String,
  pub bigrams_filepath: String,
  pub trigrams_filepath: String,
  pub index_of_coincidence: f64,
}

#[derive(Parser)]
struct TextConfidence {
  pub language_file: String,
  pub text: String,
}

#[derive(Parser)]
struct BuildQuipqiupContext {
  pub words_file: String,
}

fn main() {
  let cli_opts = CliOpts::parse();

  run(cli_opts);
}

fn run(cli_opts: CliOpts) {
  match cli_opts.commands {
    Commands::BuildLanguage(opts) => run_build_language(opts),
    Commands::TextConfidence(opts) => run_text_confidence(opts),
    Commands::BuildQuipqiupContext(opts) => run_build_quipqiup_context(opts),
  }
}

fn run_build_language(opts: BuildLanguage) {
  let words = parse_ngrams_csv(&opts.word_ngrams);
  let unigrams = parse_ngrams_csv(&opts.unigrams_filepath);
  let bigrams = parse_ngrams_csv(&opts.bigrams_filepath);
  let trigrams = parse_ngrams_csv(&opts.trigrams_filepath);

  let ranked_ngrams =
    AHashMap::from([(1, unigrams), (2, bigrams), (3, trigrams)]);
  let language = Language::new(words, ranked_ngrams, opts.index_of_coincidence);

  let language_json = serde_json::to_string_pretty(&language).unwrap();
  println!("{}", language_json);
}

fn parse_ngrams_csv(file_path: &str) -> RankedNgrams {
  let file = match File::open(file_path) {
    Ok(contents) => contents,
    Err(e) => panic!("Failed to read {file_path}: {e}"),
  };

  let mut rdr = Reader::from_reader(file);

  let ngrams_iter = rdr.records().map(|result| {
    let record = result.expect("Failed to parse csv");
    assert!(record.len() == 2, "Unexpected csv record: {:?}", record);

    let name: String = record[0].to_string();
    let age: u64 = record[1].parse().expect("Failed to parse occurrences");
    (name, age)
  });

  RankedNgrams::from_iter(ngrams_iter)
}

fn run_text_confidence(opts: TextConfidence) {
  let language_json = std::fs::read_to_string(opts.language_file)
    .expect("failed to read language");
  let language: Language =
    serde_json::from_str(&language_json).expect("failed to parse language");

  println!(
    "confidence: {}",
    language.text_confidence_chi2_trigram(&opts.text)
  );
}

fn run_build_quipqiup_context(opts: BuildQuipqiupContext) {
  let file =
    File::open(opts.words_file).expect("Failed to open: {opts.words_file}");
  let lines: Vec<String> = BufReader::new(file)
    .lines()
    .map(|line_res| line_res.expect("failed to parse line"))
    .collect();

  let quipqiup_context = QuipqiupContext::new(lines);
  let quipqiup_json = serde_json::to_string_pretty(&quipqiup_context).unwrap();
  println!("{}", quipqiup_json);
}
