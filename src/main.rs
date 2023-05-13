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
use abstractfunctions::arraymanipulations;
use sentencetokenization::sentencetokenization::Setencetokenization;
use tokenization::tokenization::Tokenization;
use transcription::transcription::get_transcript;

#[tokio::main]
async fn main() {

    let transcript = get_transcript().await;
    let  tokenizer = Tokenization{cleansed_transcript:transcript.to_string()};
    let tokens = tokenizer.tokenize();
    let sentence_tokenizer = Setencetokenization{cleansed_transcript:transcript.to_string()};
    let sentence_tokens = sentence_tokenizer.tokenize_sentences();
    println!("{:?}",sentence_tokens);
    match tokens {
            Ok(v) => {
                println!("Token is: {:?}", v);
                
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
}
