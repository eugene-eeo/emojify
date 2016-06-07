extern crate regex;
use std::env;
use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use regex::Regex;
use regex::Captures;


fn replace_emojis(map: BTreeMap<&str, &str>, string: String) -> String {
    let pattern = Regex::new(r":([a-z0-9_]+?):").unwrap();
    pattern.replace_all(&string, |cap: &Captures| {
        let key = cap.at(1).unwrap();
        match map.get(key) {
            Some(&value) => {
                let mut p = value.to_owned();
                p.push_str(" ");
                return p;
            },
            None => key.to_owned()
        }
    })
}

fn main() {
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

    let output = match env::args().skip(1).next() {
        Some(string) => {
            let mut str = replace_emojis(map, string);
            str.push('\n');
            str
        },
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            replace_emojis(map, buffer)
        }
    };
    let bytes = output.into_bytes();
    io::stdout().write(&bytes[..]).unwrap();
}
