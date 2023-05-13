use regex::Regex;

pub struct Tokenization{
    pub cleansed_transcript:String
}
impl Tokenization{
    pub fn tokenize<'a>(&'a self) -> Result<Vec<&str>, &'static str> {
        // Define the tokenization pattern using a regular expression
        let pattern = Regex::new(r"\w+|[^\w\s]+").unwrap();
    
        // Tokenize the input text
        let tokens: Vec<&str> = pattern.find_iter(&self.cleansed_transcript).map(|m| m.as_str()).collect();
    
        // Check if the vector is empty
        if tokens.is_empty() {
            return Err("Empty input");
        }
    
        Ok(tokens)
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