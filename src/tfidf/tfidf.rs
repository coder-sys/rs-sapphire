use std::collections::{HashMap, HashSet};

pub fn tfidf(tokens: &[String], sentences: &[String]) -> HashMap<String, f64> {
    let mut term_freq: HashMap<&str, Vec<usize>> = HashMap::new();
    let mut doc_freq: HashMap<&str, usize> = HashMap::new();
    let mut tf_idf_scores: HashMap<String, f64> = HashMap::new();

    let num_docs = sentences.len();
    let tokens_set: HashSet<&str> = tokens.iter().map(|s| s.as_str()).collect();

    // Compute term frequency for each token in each document
    for token in &tokens_set {
        let mut token_freqs = vec![0; num_docs];

        for (i, sentence) in sentences.iter().enumerate() {
            let freq = sentence.split_whitespace().filter(|&w| w == *token).count();
            token_freqs[i] = freq;
        }

        term_freq.insert(token, token_freqs);
    }

    // Compute document frequency and tf-idf scores
    for (token, freqs) in &term_freq {
        let doc_freq_count = freqs.iter().filter(|&f| *f > 0).count() + 1;
        doc_freq.insert(token, doc_freq_count);

        for (i, &freq) in freqs.iter().enumerate() {
            let tf = (freq + 1) as f64 / (tokens.len() + tokens_set.len()) as f64;
            let idf = (num_docs + 1) as f64 / (doc_freq_count as f64 + 1.0).ln();
            let score = tf * idf;
            let token_str = (*token).to_string();
            tf_idf_scores.entry(token_str.clone()).or_insert(0.0);
            *tf_idf_scores.get_mut(&token_str).unwrap() += score;
        }
    }

    tf_idf_scores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tfidf() {
        let tokens = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
        ];
        let sentences = vec![
            String::from("apple banana apple"),
            String::from("banana cherry"),
            String::from("cherry apple"),
        ];

        let tfidf_scores = tfidf(&tokens, &sentences);

        // Verify tf-idf scores for specific tokens
        println!("{:#?}",tfidf_scores.get("cherry"));
        assert_eq!(tfidf_scores.get("apple"), Some(&2.8853900817779268));
        assert_eq!(tfidf_scores.get("banana"), Some(&2.404491734814939));
        assert_eq!(tfidf_scores.get("cherry"), Some(&2.404491734814939));
    }
}
