use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub struct Tokenization {
    pub cleansed_transcript: String,
}

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"\w+|[^\w\s]+").unwrap();
    static ref STOPWORDS: HashSet<&'static str> = [
        "!", ";", ":", ",", "/", "]", "[", "[]", "i", "me", "you", "he", "him", "she", "her", "it",
        "we", "us", "they", "them", "myself", "yourself", "himself", "herself", "itself", "ourselves",
        "themselves", "who", "whom", "whose", "which", "what", "whatever", "whoever", "whomever",
        "anyone", "anybody", "anything", "someone", "somebody", "something", "everyone", "everybody",
        "everything", "no one", "nobody", "nothing", "dog", "cat", "car", "house", "book", "person",
        "city", "tree", "food", "water", "sun", "moon", "flower", "child", "friend", "job", "money",
        "time", "music", "movie", "computer", "phone", "camera", "shoe", "clothes", "room", "bed",
        "chair", "table", "knife", "fork", "spoon", "plate", "cup", "glass", "bag", "hat", "shirt",
        "pants", "skirt", "dress", "socks", "shoes", "watch", "jewelry", "guitar", "piano", "violin",
        "drum", "basketball", "football", "soccer", "baseball", "tennis", "golf", "a", "b", "c", "d",
        "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w",
        "x", "y", "z",
    ]
    .iter()
    .cloned()
    .collect();
}

impl Tokenization {
    pub fn tokenize(&self) -> Result<Vec<&str>, &'static str> {
        let filtered_tokens: Vec<&str> = PATTERN
            .find_iter(&self.cleansed_transcript)
            .filter_map(|m| {
                let token = m.as_str();
                if !STOPWORDS.contains(&token.to_lowercase().as_str()) {
                    Some(token)
                } else {
                    None
                }
            })
            .collect();

        if filtered_tokens.is_empty() {
            Err("Empty input")
        } else {
            Ok(filtered_tokens)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization() {
        let tokenization = Tokenization {
            cleansed_transcript: "hello, world!".to_string(),
        };

        let expected_tokens = vec!["hello", "world"];
        println!("actual is {:#?}",tokenization.tokenize().unwrap());
        assert_eq!(tokenization.tokenize().unwrap(), expected_tokens);
    }

    #[test]
    fn test_tokenization_empty_input() {
        let tokenization = Tokenization {
            cleansed_transcript: "".to_string(),
        };

        assert_eq!(tokenization.tokenize().unwrap_err(), "Empty input");
    }
}
