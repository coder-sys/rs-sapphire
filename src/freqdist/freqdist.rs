use std::collections::HashMap;

pub fn freq_dist<'a, T: AsRef<str>>(words: &'a [T], text: &'a str) -> HashMap<&'a str, usize> {
    let mut freq_map: HashMap<&str, usize> = HashMap::new();
    for word in words {
        *freq_map.entry(word.as_ref()).or_insert(0) += text.matches(word.as_ref()).count();
    }
    freq_map
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

