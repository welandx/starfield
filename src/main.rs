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
    let (mut word, mut code) = starfield::readdict(&args.dict);
    let file = File::open(&args.add).expect("no such file");
    let buf = BufReader::new(file);

    for line in buf.lines() {
        let i = line.unwrap().to_string();
        let len = &i.chars().count();
        match len {
            2 |3 | 4 => word::word2(&i, &map, &danzi,&mut word,&mut code),
            _     => {}
        };
    }
}
