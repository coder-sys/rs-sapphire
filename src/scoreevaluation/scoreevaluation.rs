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