use std::fs;
use std::collections::HashMap;

fn main() {
    let file = "file";
    let mut word_freq = HashMap::new();
    let content = fs::read_to_string(file).unwrap();
    for word in content.split_whitespace(){
        let count = word_freq.entry(word).or_insert(0);
        *count+=1;
    }
    dbg!(&word_freq);
}
