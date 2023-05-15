use regex::Regex;

pub struct Tokenization {
    pub cleansed_transcript: String,
}
impl Tokenization{
    pub fn tokenize(&self) -> Result<Vec<&str>, &'static str> {
        // Define the tokenization pattern using a regular expression
        let pattern = Regex::new(r"\w+|[^\w\s]+").unwrap();

        // Tokenize the input text
        let tokens: Vec<&str> = pattern.find_iter(&self.cleansed_transcript).map(|m| m.as_str()).collect();
    
        // Define the set of stopwords
        let stopwords = ["/","]","[","[]","i", "me", "you", "he", "him", "she", "her", "it", "we", "us", "they", "them", "myself", "yourself", "himself", "herself", "itself", "ourselves", "themselves", "who", "whom", "whose", "which", "what", "whatever", "whoever", "whomever", "anyone", "anybody", "anything", "someone", "somebody", "something", "everyone", "everybody", "everything", "no one", "nobody", "nothing","dog", "cat", "car", "house", "book", "person", "city", "tree", "food", "water", "sun", "moon", "flower", "child", "friend", "job", "money", "time", "music", "movie", "computer", "phone", "camera", "shoe", "clothes", "room", "bed", "chair", "table", "knife", "fork", "spoon", "plate", "cup", "glass", "bag", "hat", "shirt", "pants", "skirt", "dress", "socks", "shoes", "watch", "jewelry", "guitar", "piano", "violin", "drum", "basketball", "football", "soccer", "baseball", "tennis", "golf","a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    
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
