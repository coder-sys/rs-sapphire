mod transcription {
    pub mod transcription;  // Import the module from transcription/transcription.rs
}
mod tokenization{
    pub mod tokenization;
}
mod sentencetokenization{
    pub mod sentencetokenization;
}
mod abstractfunctions{
    pub mod arraymanipulations;
}
mod freqdist{
    pub mod freqdist;
}
mod tfidf{
    pub mod tfidf;
}
use tfidf::tfidf::tfidf;
use abstractfunctions::arraymanipulations;
use sentencetokenization::sentencetokenization::Setencetokenization;
use tokenization::tokenization::Tokenization;
use transcription::transcription::get_transcript;
use freqdist::freqdist::freq_dist;
#[tokio::main]
async fn main() {

    let transcript = get_transcript().await;
    let  tokenizer = Tokenization{cleansed_transcript:transcript.to_string()};
    let tokens = tokenizer.tokenize();
    let sentence_tokenizer = Setencetokenization{cleansed_transcript:transcript.to_string()};

    let sentence_tokens = sentence_tokenizer.tokenize_sentences();
    let sentence_tokens = arraymanipulations::purify_array(&sentence_tokens);
    match tokens {
            Ok(v) => {
                let tokens = arraymanipulations::purify_array(&v);
                let freq = freq_dist(&tokens,&transcript.as_str()); 
                let tokens: Vec<String> = tokens.iter().map(|s| s.to_string()).collect();
                let slice_string: &[String] = &tokens;
                let sentence_tokens: Vec<String> = sentence_tokens.iter().map(|s| s.to_string()).collect();
                let sentence_tokens: &[String] = &sentence_tokens;
                let tfdf_matrix = tfidf(&tokens,&sentence_tokens);  
                println!("{:#?}",tfdf_matrix)             
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
}
