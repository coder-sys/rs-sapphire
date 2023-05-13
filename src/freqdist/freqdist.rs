use std::collections::HashMap;

pub fn freq_dist<'a,T: AsRef<str>>(words: &'a [T], text: &'a str) -> HashMap<&'a str, usize> {
    let mut freq_map: HashMap<&str, usize> = HashMap::new();
    for word in words {
        *freq_map.entry(word.as_ref()).or_insert(0) += text.matches(word.as_ref()).count();
    }
    freq_map
}
