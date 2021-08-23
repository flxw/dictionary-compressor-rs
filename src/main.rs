use std::{collections::{HashMap, hash_map::Entry}, fs::File, io::Read};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct CompressionData {
    pub compressed_data: Vec<u64>,
    pub dictionary: HashMap<String, u64>
}

fn main() {
    let filepath = "/Users/f.wolffcelonis.com/code/dictionary-compressor-rs/chapter-background.tex";
    let mut file = File::open(filepath).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    let mut dictionary: HashMap<String, u64> = HashMap::new();
    let mut starter_value: u64 = 1;

    let compressed_data: Vec<u64> = file_contents
        .split_ascii_whitespace()
        .map(|token| 
            match dictionary.entry(token.to_string()) {
                Entry::Occupied(o) => o.get().clone(),
                Entry::Vacant(v) => {
                    starter_value += 1;
                    v.insert(starter_value).clone()
                }
            }
        )
        .collect();

    let data = CompressionData{ compressed_data, dictionary };
    let compressed_filepath = filepath.to_owned() + ".dictionary-compressed";
    let destination_file = File::create(compressed_filepath).unwrap();
    serde_json::to_writer(destination_file, &data).expect("oh noes");
}