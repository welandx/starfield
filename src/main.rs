use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod word;

#[derive(Parser)]
struct Cli {
    map:   std::path::PathBuf,
    danzi: std::path::PathBuf,
    dict:  std::path::PathBuf,
    add:   std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let map = starfield::readmap(&args.map);
    let danzi = starfield::readmap(&args.danzi);
    let (word, code) = starfield::readdict(&args.dict);
    let file = File::open(&args.add).expect("no such file");
    let buf = BufReader::new(file);

    for line in buf.lines() {
        let i = line.unwrap().to_string();
        let len = &i.len();
        match len / 3 {
            2     => word::word2(&i, &map, &danzi, &word, &code),
            3 | 4 => word::word3(&i, &map, &danzi, &word, &code),
            _     => {}
        };
    }
}
