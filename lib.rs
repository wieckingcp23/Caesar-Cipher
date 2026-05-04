use std::error::Error;
use std::fmt::{Display, Formatter};

pub trait Cipher {
    fn encrypt(&self, plaintext: &str) -> String;
    fn decrypt(&self, ciphertext: &str) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaesarCipher {
    shift: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CipherError {
    InvalidKey(u8),
}

impl Display for CipherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidKey(key) => write!(f, "invalid key {key}; expected a value from 1 to 26"),
        }
    }
}

impl Error for CipherError {}

impl CaesarCipher {
    pub fn new(key: u8) -> Result<Self, CipherError> {
        if !(1..=26).contains(&key) {
            return Err(CipherError::InvalidKey(key));
        }

        Ok(Self { shift: key % 26 })
    }

    fn shift_text(&self, input: &str, shift: i16) -> String {
        input
            .chars()
            .map(|ch| match ch {
                'a'..='z' => shift_letter(ch, shift, b'a'),
                'A'..='Z' => shift_letter(ch, shift, b'A'),
                _ => ch,
            })
            .collect()
    }
}

impl Cipher for CaesarCipher {
    fn encrypt(&self, plaintext: &str) -> String {
        self.shift_text(plaintext, i16::from(self.shift))
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        self.shift_text(ciphertext, -i16::from(self.shift))
    }
}

fn shift_letter(ch: char, shift: i16, base: u8) -> char {
    let alpha_index = ch as u8 - base;
    let shifted_index = (i16::from(alpha_index) + shift).rem_euclid(26) as u8;

    char::from(base + shifted_index)
}

#[cfg(test)]
mod tests {
    use super::{CaesarCipher, Cipher, CipherError};

    #[test]
    fn encrypts_text_with_spaces() {
        let cipher = CaesarCipher::new(3).expect("key should be valid");

        assert_eq!(cipher.encrypt("Hello World"), "Khoor Zruog");
    }

    #[test]
    fn preserves_case_and_punctuation() {
        let cipher = CaesarCipher::new(5).expect("key should be valid");

        assert_eq!(cipher.encrypt("Rust 2026!"), "Wzxy 2026!");
    }

    #[test]
    fn decrypts_back_to_original_text() {
        let cipher = CaesarCipher::new(3).expect("key should be valid");

        assert_eq!(cipher.decrypt("Khoor Zruog"), "Hello World");
    }

    #[test]
    fn wraps_at_end_of_alphabet() {
        let cipher = CaesarCipher::new(1).expect("key should be valid");

        assert_eq!(cipher.encrypt("Zebra"), "Afcsb");
    }

    #[test]
    fn treats_twenty_six_as_full_rotation() {
        let cipher = CaesarCipher::new(26).expect("key should be valid");

        assert_eq!(cipher.encrypt("Rotate Me"), "Rotate Me");
    }

    #[test]
    fn rejects_keys_outside_allowed_range() {
        assert_eq!(CaesarCipher::new(0), Err(CipherError::InvalidKey(0)));
        assert_eq!(CaesarCipher::new(27), Err(CipherError::InvalidKey(27)));
    }
}
