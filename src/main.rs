use clap::Parser;
use pinyin::ToPinyin;
//use std::{ascii::AsciiExt, collections::HashMap, fmt::Debug};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    danzi: std::path::PathBuf,
    dict: std::path::PathBuf,
    add: std::path::PathBuf,
}

fn word2(
    hans: &String,
    map: &HashMap<String, Vec<String>>,
    danzi: &HashMap<String, Vec<String>>,
    word: &HashMap<String, bool>,
    code: &HashMap<String, bool>,
) {
    let han = &hans[..];
    // convert to pinyin
    let mut res = String::new();
    let mut has_fly = false;
    let mut res_fly = String::new();
    if word.contains_key(hans) {
        return;
    }
    for pinyin in han.to_pinyin() {
        if let Some(pinyin) = pinyin {
            let fly = map.get(pinyin.plain()).unwrap().len();
            res.push_str(&map.get(pinyin.plain()).unwrap()[0]);
            if fly == 2 {
                has_fly = true;
                res_fly.push_str(&map.get(pinyin.plain()).unwrap()[1])
            } else {
                res_fly.push_str(&map.get(pinyin.plain()).unwrap()[0]);
            }
        }
    }
    let mut index = 0;
    if has_fly {
        let mut index = 0;
        while res_fly.len() < 6 {
            if code.contains_key(&res_fly) {
                res_fly.push(
                    danzi.get(&hans[index..index + 3]).unwrap()[0]
                        .chars()
                        .nth(2)
                        .unwrap(),
                );
                index += 3;
            } else {
                break;
            }
        }
        println!("{}\t{}", han, res_fly);
    }
    while res.len() < 6 {
        if code.contains_key(&res) {
            res.push(
                danzi.get(&hans[index..index + 3]).unwrap()[0]
                    .chars()
                    .nth(2)
                    .unwrap(),
            );
            index += 3;
        } else {
            println!("{}\t{}", han, res);
            return;
        }
    }
    println!("{}\t{}", han, res);
}

fn word3(
    hans: &String,
    map: &HashMap<String, Vec<String>>,
    danzi: &HashMap<String, Vec<String>>,
    word: &HashMap<String, bool>,
    code: &HashMap<String, bool>,
) {
    let han = &hans[..];
    let mut res = String::new();
    if word.contains_key(hans) {
        return;
    }
    for pinyin in han.to_pinyin() {
        if let Some(pinyin) = pinyin {
            let sf = &map.get(pinyin.plain()).unwrap()[0];
            let nth0 = sf.chars().nth(0).unwrap();
            res.push(nth0);
        }
    }
    let mut index = 0;
    while res.len() < 6 {
        if code.contains_key(&res) {
            res.push(
                danzi.get(&hans[index..index + 3]).unwrap()[0]
                    .chars()
                    .nth(2)
                    .unwrap(),
            );
            index += 3;
        } else {
            println!("{}\t{}", han, res);
            return;
        }
    }
    println!("{}\t{}", han, res);
}
fn main() {
    let args = Cli::parse();
    //let sf_map: HashMap<String, String> = HashMap::new();
    let map = starfield::readmap(&args.path);
    let danzi = starfield::readmap(&args.danzi);
    let (word, code) = starfield::readdict(&args.dict);
    let file = File::open(&args.add).expect("no such file");
    let buf = BufReader::new(file);

    for line in buf.lines() {
        let i = line.unwrap().to_string();
        let len = &i.len();
        match len / 3 {
            2 => word2(&i, &map, &danzi, &word, &code),
            3 | 4 => word3(&i, &map, &danzi, &word, &code),
            _ => {}
        };
    }
}
