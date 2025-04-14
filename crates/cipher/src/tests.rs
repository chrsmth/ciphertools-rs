use once_cell::sync::Lazy;

#[macro_export]
macro_rules! cipher_test {
  ($context:expr, $plaintext:expr, $encipher_key:expr, $name:ident) => {
    #[test]
    #[allow(non_snake_case)]
    fn $name() {
      use insta::assert_snapshot;
      let ciphertext = $context.encipher(&$plaintext, &$encipher_key);
      let plaintext = $context.decipher(&ciphertext, &$encipher_key);

      assert_eq!(plaintext, $plaintext);
      assert_snapshot!(ciphertext);
    }
  };
}

macro_rules! lazy_read_to_string {
  ($path:literal) => {
    Lazy::new(|| {
      std::fs::read_to_string($path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", $path, e))
    })
  };
}

pub static LATIN_LOWERCASE: &[char; 26] = &[
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
  'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
pub static RUSSIAN_LOWERCASE: &[char; 33] = &[
  'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н',
  'о', 'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь',
  'э', 'ю', 'я',
];
pub static GREEK_LOWERCASE: &[char; 24] = &[
  'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο',
  'π', 'ρ', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω',
];
pub static DIGITS: &[char; 10] =
  &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub static SYMBOLS: &[char; 32] = &[
  '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '-', '+', '=', '{',
  '}', '[', ']', ':', ';', '<', '>', '?', ',', '.', '/', '\\', '|', '`', '~',
  '\"', '\'',
];

pub static QUICK_BROWN_FOX: Lazy<String> =
  lazy_read_to_string!("./test-texts/plaintexts/the_quick_brown_fox.txt");

pub static QUICKBROWNFOX: Lazy<String> =
  lazy_read_to_string!("./test-texts/plaintexts/thequickbrownfox.txt");
