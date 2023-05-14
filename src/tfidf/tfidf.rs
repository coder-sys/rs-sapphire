use std::collections::{HashMap, HashSet};

pub fn tfidf(tokens: &[String], sentences: &[String]) -> HashMap<String, f64> {
    let mut term_freq: HashMap<&str, usize> = HashMap::new();
    let mut doc_freq: HashMap<&str, usize> = HashMap::new();
    let mut tf_idf_scores: HashMap<String, f64> = HashMap::new();

    let num_docs = sentences.len();
    let tokens_set: HashSet<&str> = tokens.iter().map(|s| s.as_str()).collect();

    for token in &tokens_set {
        let mut token_freq = 0;

        for sentence in sentences {
            if sentence.contains(token) {
                token_freq += 1;
            }
        }

        term_freq.insert(token, token_freq);
    }

    for (token, freq) in &term_freq {
        let mut doc_freq_count = 0;

        for sentence in sentences {
            if sentence.contains(token) {
                doc_freq_count += 1;
            }
        }

        doc_freq.insert(token, doc_freq_count);
        let tf = *freq as f64 / tokens.len() as f64;
        let idf = (num_docs as f64 / doc_freq_count as f64).ln();
        tf_idf_scores.insert((*token).to_string(), tf * idf);
    }

    tf_idf_scores
}