extern crate serde;
extern crate serde_json;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};

mod cipher;
mod pallet;

use cipher::vigenere::*;
use cipher::Cipher;

macro_rules! key_arg {
	() => (
		Arg::with_name("key")
			.short("k")
			.value_name("KEY")
			.required(true)
		)
}

macro_rules! decipher_subcommand {
	() => (
		SubCommand::with_name("decipher")
		.about("Decipher ciphertext")
		.arg(Arg::with_name("ciphertext")
			 .short("c")
			 .value_name("CIPHERTEXT")
			 .required(true))
		.arg(key_arg!())
	)
}

macro_rules! encipher_subcommand {
	() => (
		SubCommand::with_name("encipher")
		.about("Encipher plaintext")
		.arg(Arg::with_name("plaintext")
			 .short("p")
			 .value_name("PLAINTEXT")
			 .required(true))
		.arg(key_arg!())
	)
}

macro_rules! encipher {
	($matches:ident, $Cipher:ident) => (
		if let Some(matches) = $matches.subcommand_matches("encipher") {
			let plaintext = String::from(matches.value_of("plaintext").unwrap());
			let key = $Cipher::parse(matches.value_of("key").unwrap());

			match key {
				Some(key) => println!("{:}", $Cipher::encipher(plaintext, key)),
				_ => println!("Parse key failed"),
			}
		}
	)
}

macro_rules! decipher {
	($matches:ident, $Cipher:ident) => (
		if let Some(matches) = $matches.subcommand_matches("decipher") {
			let ciphertext = String::from(matches.value_of("ciphertext").unwrap());
			let key = $Cipher::parse(matches.value_of("key").unwrap());

			match key {
				Some(key) => println!("{:}", $Cipher::encipher(ciphertext, key)),
				_ => println!("Parse key failed"),
			}
		}
	)
}

fn main() {
	let matches = App::new("Cipher Tools")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(SubCommand::with_name("vigenere")
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(encipher_subcommand!())
			.subcommand(decipher_subcommand!()))
		.get_matches();

	if let Some(matches) = matches.subcommand_matches("vigenere") {
		encipher!(matches, Vigenere);
		decipher!(matches, Vigenere);
	}
}