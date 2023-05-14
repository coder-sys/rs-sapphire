use regex::Regex;

pub struct Tokenization{
    pub cleansed_transcript:String
}
impl Tokenization{
    pub fn tokenize(&self) -> Result<Vec<&str>, &'static str> {
        // Define the tokenization pattern using a regular expression
        let pattern = Regex::new(r"\w+|[^\w\s]+").unwrap();
    
        // Tokenize the input text
        let tokens: Vec<&str> = pattern.find_iter(&self.cleansed_transcript).map(|m| m.as_str()).collect();
    
        // Define the set of stopwords
        let stopwords = ["the", "a", "an", "and", "or"];
    
        // Filter out any tokens that match the stopwords
        let filtered_tokens: Vec<&str> = tokens.into_iter()
            .filter(|&token| !stopwords.contains(&token.to_lowercase().as_str()))
            .collect();
    
        // Check if the vector is empty after removing stopwords
        if filtered_tokens.is_empty() {
            return Err("Empty input");
        }
    
        Ok(filtered_tokens)
    }
    
}
//match tokens {
//    Ok(v) => {
//        println!("Tokens: {:?}", v);
//    }
//    Err(e) => {
 //       println!("Error: {}", e);
//    }
//}