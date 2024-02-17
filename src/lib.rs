use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn readmap(file_path: &std::path::PathBuf) -> HashMap<String, Vec<String>>{
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut map:HashMap<String, _> = HashMap::new();
    for line in reader.lines() {
        if let Some((key, values)) = line.unwrap()
            .split_whitespace()
            .collect::<Vec<_>>()
            .split_first() {
                map.insert(
                    key.to_string(),
                    values.iter()
                        .map(|&s| s.to_string())
                        .collect::<Vec<_>>());
        }
    }

    return map;
}
