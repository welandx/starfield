use pinyin::ToPinyin;
use std::collections::HashMap;

fn word_to_pinyin3(hans: &String, map: &HashMap<String, Vec<String>>) -> (String, String, bool) {
    let han = &hans[..];
    let mut res = String::new();
    let mut res_fly = String::new();
    let mut has_fly = false;

    for pinyin in han.to_pinyin() {
        if let Some(pinyin) = pinyin {
            let current_pinyin = map.get(pinyin.plain()).unwrap();
            res.push(current_pinyin[0].chars().nth(0).unwrap());
            res_fly.push(current_pinyin[0].chars().nth(0).unwrap());

            if current_pinyin.len() == 2 {
                has_fly = true;
                res_fly.pop();
                res_fly.push(current_pinyin[1].chars().nth(0).unwrap());
            }
        }
    }

    (res, res_fly, has_fly)
}

fn word_to_pinyin(hans: &String, map: &HashMap<String, Vec<String>>) -> (String, String, bool) {
    let han = &hans[..];
    let mut res = String::new();
    let mut res_fly = String::new();
    let mut has_fly = false;

    for pinyin in han.to_pinyin() {
        if let Some(pinyin) = pinyin {
            let current_pinyin = map.get(pinyin.plain()).unwrap();
                res.push_str(&current_pinyin[0]);
                res_fly.push_str(&current_pinyin[0]);

            if current_pinyin.len() == 2 {
                has_fly = true;
                for _ in 0..2 {
                    res_fly.pop();
                }
                res_fly.push_str(&current_pinyin[1]);
            }
        }
    }

    (res, res_fly, has_fly)
}

fn apply_danzi_correction(
    han: &str,
    res: &mut String,
    prefix_index: &mut usize,
    danzi: &HashMap<String, Vec<String>>,
    code: &HashMap<String, bool>,
) {
    while res.len() < 6 {
        if code.contains_key(res) {
            res.push(
                danzi.get(&han[*prefix_index..*prefix_index + 3]).unwrap()[0]
                    .chars()
                    .nth(2)
                    .unwrap(),
            );
            *prefix_index += 3;
        } else {
            println!("{}\t{}", han, res);
            return;
        }
    }
    println!("{}\t{}", han, res);
}

pub fn word2(
    hans: &String,
    map: &HashMap<String, Vec<String>>,
    danzi: &HashMap<String, Vec<String>>,
    word: &HashMap<String, bool>,
    code: &HashMap<String, bool>,
) {
    if word.contains_key(hans) {
        return;
    }

    let (mut res, mut res_fly, has_fly) = word_to_pinyin(hans, map);

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
        println!("{}\t{}", hans, res_fly);
    }

    let mut index = 0;
    apply_danzi_correction(hans, &mut res, &mut index, danzi, code);
}

pub fn word3(
    hans: &String,
    map: &HashMap<String, Vec<String>>,
    danzi: &HashMap<String, Vec<String>>,
    word: &HashMap<String, bool>,
    code: &HashMap<String, bool>,
) {
    if word.contains_key(hans) {
        return;
    }

    let (mut res, mut res_fly, has_fly) = word_to_pinyin3(hans, map);

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
        println!("{}\t{}", hans, res_fly);
    }

    let mut index = 0;
    apply_danzi_correction(hans, &mut res, &mut index, danzi, code);
}
