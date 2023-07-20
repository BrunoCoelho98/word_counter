use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};



fn main() {
    let file = File::open("F:/NotePad/bokkeepers.txt").expect("Falha ao abrir o arquivo");

    let reader = BufReader::new(file);

    let mut word_count: HashMap<String, usize> = HashMap::new();

    for line in reader.lines(){
        if let Ok(line) = line {
            let words: Vec<String> = line.split_whitespace().map(String::from).collect();

            for word in words {
                *word_count.entry(word).or_insert(0) += 1;
            }
        }
    }

    for (word, count) in &word_count{
        println!("{}: {}", word, count);
    }


}
