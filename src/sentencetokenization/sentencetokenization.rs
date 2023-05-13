use regex::Regex;
pub struct Setencetokenization{
    pub cleansed_transcript:String
}

impl Setencetokenization{

    pub fn tokenize_sentences(&self) -> Vec<&str> {
        let re = Regex::new(r"[.!?]").unwrap();
        let sentences: Vec<&str> = re.split(&self.cleansed_transcript).collect();
    
        sentences
    }
}