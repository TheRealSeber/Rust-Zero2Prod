use serde::Deserialize;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct SubscriberName(String);

impl TryFrom<String> for SubscriberName {
    type Error = String;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        let is_empty_or_whitespace = name.trim().is_empty();
        let is_too_long = name.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let has_forbidden_characters = name.chars().any(|c| forbidden_characters.contains(&c));
        match is_empty_or_whitespace || is_too_long || has_forbidden_characters {
            true => Err(format!("{} is not a not valid name!", name)),
            false => Ok(Self(name)),
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SubscriberName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(SubscriberName::try_from(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::try_from(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::try_from(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::try_from(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::try_from(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::try_from(name));
    }
}
