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
            None => cap.at(0).unwrap().to_owned()
        }
    })
}

fn main() {
    let mut map = BTreeMap::new();
    macro_rules! M { ($k:expr, $v:expr) => { map.insert($k, $v); } }

    M!("grin",          "\u{1F601}");
    M!("joy",           "\u{1F602}");
    M!("smiley",        "\u{1F603}");
    M!("smile",         "\u{1F604}");
    M!("sweat_smile",   "\u{1F605}");
    M!("satisfied",     "\u{1F606}");
    M!("wink",          "\u{1F609}");
    M!("relaxed",       "\u{1F60A}");
    M!("yum",           "\u{1F60B}");
    M!("relieved",      "\u{1F60C}");
    M!("heart_eyes",    "\u{1F60D}");
    M!("smirk",         "\u{1F60F}");
    M!("unamused",      "\u{1F612}");
    M!("sweat",         "\u{1F613}");
    M!("pensive",       "\u{1F614}");
    M!("confounded",    "\u{1F616}");
    M!("kissing_heart", "\u{1F618}");
    M!("kissing_closed_eyes", "\u{1F61A}");
    M!("stuck_out_tongue_winking_eye", "\u{1F61C}");
    M!("stuck_out_tongue_closed_eyes", "\u{1F61D}");
    M!("disappointed",  "\u{1F61E}");
    M!("angry",         "\u{1F620}");
    M!("rage",          "\u{1F621}");
    M!("cry",           "\u{1F622}");
    M!("persevere",     "\u{1F623}");
    M!("triumph",       "\u{1F624}");
    M!("disappointed_relieved", "\u{1F625}");
    M!("fearful",       "\u{1F628}");
    M!("weary",         "\u{1F629}");
    M!("sleepy",        "\u{1F62A}");
    M!("tired_face",    "\u{1F62B}");
    M!("sob",           "\u{1F62D}");
    M!("cold_sweat",    "\u{1F630}");
    M!("scream",        "\u{1F631}");
    M!("astonished",    "\u{1F632}");
    M!("mask",          "\u{1F637}");
    M!("smile_cat",     "\u{1F638}");

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
