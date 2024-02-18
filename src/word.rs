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

            if current_pinyin.len() == 2
                && current_pinyin[0].chars().nth(0).unwrap()
                    != current_pinyin[1].chars().nth(0).unwrap()
            {
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
    han: &String,
    res: &mut String,
    danzi: &HashMap<String, Vec<String>>,
    code: &mut HashMap<String, bool>,
) {

    let mut index=0;
    while res.len() < 6 {
        if code.contains_key(res) {
            let tmp = han.chars().nth(index).unwrap();
            if !danzi.contains_key(&tmp.to_string()){
                return;
            }
            res.push(
                danzi.get(&String::from(han.chars().nth(index).unwrap())).unwrap()[0]
                    .chars()
                    .nth(2)
                    .unwrap(),
            );
            index += 1;
        } else {
            println!("{}\t{}", han, res);
            code.insert(res.to_string(), true);
            return;
        }
    }
    code.insert(res.to_string(), true);
    println!("{}\t{}", han, res);
}

pub fn word2(
    hans: &String,
    map: &HashMap<String, Vec<String>>,
    danzi: &HashMap<String, Vec<String>>,
    word: &mut HashMap<String, bool>,
    code: &mut HashMap<String, bool>,
) {
    if word.contains_key(hans) {
        return;
    }

    word.insert(hans.to_string(), true);

    let (mut res, mut res_fly, has_fly) = if hans.chars().count() == 2 {
        word_to_pinyin(hans, map)
    } else {
        word_to_pinyin3(hans, map)
    };

    let len = hans.chars().count();
    let rl = res.len();

    if (len==2 && rl!=4) || (len==3 && rl!=3) || (len==4 && rl!=4){
        return;
    }

    let backhan=hans.clone();

    if has_fly {
        let mut index = 0;
        while res_fly.len() < 6 {
            if code.contains_key(&res_fly) {
                let tmp = hans.chars().nth(index).unwrap();
                if !danzi.contains_key(&tmp.to_string()){
                    return;
                }
                res_fly.push(
                    danzi.get(&String::from(hans.chars().nth(index).unwrap())).unwrap()[0]
                        .chars()
                        .nth(2)
                        .unwrap(),
                );
                index += 1;
            } else {
                break;
            }
        }
        println!("{}\t{}", hans, res_fly);
        code.insert(res_fly, true);
    }
    apply_danzi_correction(&backhan, &mut res, danzi, code);
}
