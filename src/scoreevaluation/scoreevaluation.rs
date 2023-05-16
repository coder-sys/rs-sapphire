use std::collections::HashMap;

pub fn evaluatescore<'a>(tfidf:HashMap<String,f64>,freqdist:HashMap<&'a str, usize>)->f64{
    let mut num:f64 = 0.0;
    for (key,value) in &tfidf{
        let floatfrmt:f64 = *freqdist.get(key.as_str()).unwrap() as f64;
        num += value*floatfrmt;
    }
    let den = tfidf.len() as f64;
    let ratio = num/den;
    ratio as f64
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluatescore() {
        let mut tfidf: HashMap<String, f64> = HashMap::new();
        tfidf.insert(String::from("apple"), 0.5);
        tfidf.insert(String::from("banana"), 0.8);
        tfidf.insert(String::from("cherry"), 0.3);

        let mut freqdist: HashMap<&str, usize> = HashMap::new();
        freqdist.insert("apple", 2);
        freqdist.insert("banana", 3);
        freqdist.insert("cherry", 1);

        let score = evaluatescore(tfidf, freqdist);
        println!("im looking for {}",score);
        assert_eq!(score, 1.2333333333333334);
    }
}
