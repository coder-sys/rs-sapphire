mod transcription {
    pub mod transcription; // Import the module from transcription/transcription.rs
}
mod tokenization {
    pub mod tokenization;
}
mod sentencetokenization {
    pub mod sentencetokenization;
}
mod abstractfunctions {
    pub mod arraymanipulations;
}
mod freqdist {
    pub mod freqdist;
}
use abstractfunctions::arraymanipulations;
use freqdist::freqdist::freq_dist;
use sentencetokenization::sentencetokenization::Setencetokenization;
use tokenization::tokenization::Tokenization;
use transcription::transcription::get_transcript;

const BASE_URL: &str = "http://localhost:8000";

#[tokio::main]
async fn main() {
    let transcript = get_transcript(BASE_URL.to_string()).await;
    let tokenizer = Tokenization {
        cleansed_transcript: transcript.to_string(),
    };
    let tokens = tokenizer.tokenize();
    let sentence_tokenizer = Setencetokenization {
        cleansed_transcript: transcript.to_string(),
    };

    let sentence_tokens = sentence_tokenizer.tokenize_sentences();
    let sentence_tokens = arraymanipulations::purify_array(&sentence_tokens);
    match tokens {
        Ok(v) => {
            let tokens = arraymanipulations::purify_array(&v);
            let freq = freq_dist(&tokens, &transcript.as_str());
            println!("{:#?}", freq.get("applause"))
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
