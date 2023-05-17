use std::collections::HashMap;

pub fn freq_dist<'a, T: AsRef<str>>(words: &'a [T], text: &'a str) -> HashMap<&'a str, usize> {
    let mut freq_map: HashMap<&str, usize> = HashMap::new();

    for word in words {
        let word_ref = word.as_ref();
        *freq_map.entry(word_ref).or_insert(0) += count_matches(text, word_ref);
    }

    freq_map
}

fn count_matches(text: &str, word: &str) -> usize {
    let mut count = 0;
    let mut start = 0;

    while let Some(index) = text[start..].find(word) {
        count += 1;
        start += index + word.len();
    }

    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq_dist() {
        let words = ["apple", "banana", "cherry"];
        let text = "apple banana cherry banana apple";
        let freq_map = freq_dist(&words, text);
        
        assert_eq!(freq_map.get("apple"), Some(&2));
        assert_eq!(freq_map.get("banana"), Some(&2));
        assert_eq!(freq_map.get("cherry"), Some(&1));
    }
}

