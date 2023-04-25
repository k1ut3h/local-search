use walkdir::{WalkDir, DirEntry};
use regex::Regex;
use ansi_term::Colour;

fn main() {
    let input = &std::env::args().collect::<Vec<_>>()[1];
    for entry in WalkDir::new("."){
        let path = entry.as_ref().unwrap().clone();
        let contents = match std::fs::read_to_string(entry.unwrap().path()){
            Ok(contents) => contents,
            Err(_) => "".to_string()
        };
        check_match(input, contents, path);
    }
}

fn check_match(input: &String, contents: String, path: DirEntry){
    let mut count = 0;
    let re = Regex::new(&format!(r"{}", input)).unwrap();
    for line in contents.lines() {
        let matches: Vec<_> = re.find_iter(line).collect();
        if !matches.is_empty() {
            count += matches.len();
            println!("{}", Colour::Red.bold().paint(format!("{}",path.path().display())));
            println!("Matches found for \"{}\": {}", Colour::Green.paint(input),Colour::Yellow.paint(format!("{}",count)));
            println!("-------------------------");
        }
    }
}

/*
 * store the term frequency
 * store the document frequency
 * somehow apply TF-IDF (term frequency - inverse document frequency)
 */
