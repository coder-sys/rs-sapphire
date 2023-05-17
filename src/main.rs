mod transcription {
    pub mod transcription; // Import the module from transcription/transcription.rs
}
mod tokenization {
    pub mod tokenization;
}

mod abstractfunctions {
    pub mod arraymanipulations;
}
mod freqdist {
    pub mod freqdist;
}
mod tfidf{
    pub mod tfidf;
}
mod scoreevaluation{
    pub mod scoreevaluation;
}

use std::collections::HashMap;
use tfidf::tfidf::tfidf;
use abstractfunctions::arraymanipulations;
use freqdist::freqdist::freq_dist;
use tokenization::tokenization::Tokenization;
use transcription::transcription::get_transcript;
use transcription::transcription::get_sent_tokens;
use scoreevaluation::scoreevaluation::evaluatescore;

const BASE_URL: &str = "http://localhost:5000";
const VIDEO_ID: &str = "8jLOx1hD3_o";
#[tokio::main]
async fn main() {
    let transcript = get_transcript(BASE_URL.to_string(),VIDEO_ID.to_string()).await;
    let tokenizer = Tokenization {
        cleansed_transcript: transcript.to_string(),
    };
    let tokens = tokenizer.tokenize();
    

    let sentence_tokens = get_sent_tokens(BASE_URL.to_string(),VIDEO_ID.to_string()).await;
    let sentence_tokens: Vec<&str> = sentence_tokens.iter().map(|s| s.as_str()).collect();

    let sentence_tokens = arraymanipulations::purify_array(&sentence_tokens);
    match tokens {
            Ok(v) => {
                let tokens = arraymanipulations::purify_array(&v);
                let freq = freq_dist(&tokens,&transcript.as_str()); 
                let tokens: Vec<String> = tokens.iter().map(|s| s.to_string()).collect();
                let slice_string: &[String] = &tokens;
                let sentence_tokens: Vec<String> = sentence_tokens.iter().map(|s| s.to_string()).collect();
                let sentence_tokens: &[String] = &sentence_tokens;
                let tfidf_matrix = tfidf(&tokens,&sentence_tokens);  
                let tfidf_matrix: HashMap<String, f64> = tfidf_matrix
                .into_iter()
                .filter(|(_, v)| !v.is_nan())
                .collect();
        let tfidf_matrix = tfidf_matrix
        .into_iter()
        .filter(|(k, _)| {
            k.parse::<i64>().is_err() && k.parse::<f64>().is_err()
        })
        .collect::<HashMap<String, f64>>();
        let score:f64 = evaluatescore(tfidf_matrix,freq);
        println!("{}",score)
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
}
