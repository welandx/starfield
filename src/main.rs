use clap::Parser;
use pinyin::ToPinyin;
//use std::{ascii::AsciiExt, collections::HashMap, fmt::Debug};
use std::collections::HashMap;

#[derive(Parser)]
struct Cli {
    word: String,
    path: std::path::PathBuf,
    danzi: std::path::PathBuf,
}

fn word2(han: &String, map: HashMap<String, Vec<String>>, danzi: HashMap<String, Vec<String>>) {
    let han = &han[..];
    // convert to pinyin
    for pinyin in han.to_pinyin() {
        if let Some(pinyin) = pinyin {
            print!("{}", map.get(pinyin.plain()).unwrap()[0]);
        }
    }
}
fn word3(han: &String, map: HashMap<String, Vec<String>>, danzi: HashMap<String, Vec<String>>) {
    let han = &han[..];
    for pinyin in han.to_pinyin() {
        if let Some(pinyin) = pinyin {
            let sf = &map.get(pinyin.plain()).unwrap()[0];
            let mut res = String::new();
            let nth0 = sf.chars().nth(0).unwrap();
            res.push(nth0);
            // match nth0 {
            //     'z'|'c'|'s' =>{
            //         // get zh/ch/sh
            //         // let nth1=str.chars().nth(1).unwrap();
            //         // if nth1=='h'{
            //         //     res.push(nth1);
            //         // }
            //     },
            //     _ =>{}
            // }
            print!("{}", res);
        }
    }
}

fn main() {
    let args = Cli::parse();
    let len = &args.word.len();
    //let sf_map: HashMap<String, String> = HashMap::new();
    let map = starfield::readmap(&args.path);
    let danzi = starfield::readmap(&args.danzi);
    match len / 3 {
        2 => word2(&args.word, map, danzi),
        3 | 4 => word3(&args.word, map, danzi),
        _ => {}
    };
}
