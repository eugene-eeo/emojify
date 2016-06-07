extern crate regex;
use std::env;
use std::collections::BTreeMap;
use regex::Regex;
use regex::Captures;

fn main() {
    let pattern = Regex::new(r":([a-z0-9_]+?):").unwrap();
    let mut map = BTreeMap::new();
    macro_rules! M { ($k:expr, $v:expr) => { map.insert($k, $v); } }

    M!("grin",        "\u{1F601}");
    M!("joy",         "\u{1F602}");
    M!("smiley",      "\u{1F603}");
    M!("smile",       "\u{1F604}");
    M!("sweat_smile", "\u{1F605}");
    M!("satisfied",   "\u{1F606}");
    M!("wink",        "\u{1F609}");
    M!("relaxed",     "\u{1F60A}");
    M!("yum",         "\u{1F60B}");

    match env::args().skip(1).next() {
        Some(string) => {
            let result = pattern.replace_all(&string, |cap: &Captures| {
                let key = cap.at(1).unwrap();
                match map.get(key) {
                    Some(&value) => {
                        let mut p = value.to_owned();
                        p.push_str(" ");
                        return p;
                    },
                    None => key.to_owned()
                }
            });
            println!("{}", result);
        },
        None => {}
    }
}
