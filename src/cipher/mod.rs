pub mod vigenere;
pub mod caesar;

pub trait Cipher {
	type Key;

	fn encipher(plaintext: String, k: Self::Key) -> String;
	fn decipher(ciphertext: String, k: Self::Key) -> String;

	fn parse(key: &str) -> Option<Self::Key>;
}
