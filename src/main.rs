#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use regex::Regex;
use regex::Captures;

lazy_static! {
    static ref MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();

        // Emoticons
        m.insert("grin",          "😁");
        m.insert("joy",           "😂");
        m.insert("smiley",        "😃");
        m.insert("smile",         "😄");
        m.insert("sweat_smile",   "😅");
        m.insert("satisfied",     "😆");
        m.insert("wink",          "😉");
        m.insert("relaxed",       "😊");
        m.insert("yum",           "😋");
        m.insert("relieved",      "😌");
        m.insert("heart_eyes",    "😍");
        m.insert("smirk",         "😏");
        m.insert("unamused",      "😒");
        m.insert("sweat",         "😓");
        m.insert("pensive",       "😔");
        m.insert("confounded",    "😖");
        m.insert("kissing_heart", "😘");
        m.insert("kissing_closed_eyes", "😚");
        m.insert("stuck_out_tongue_winking_eye", "😜");
        m.insert("stuck_out_tongue_closed_eyes", "😝");
        m.insert("disappointed",  "😞");
        m.insert("angry",         "😠");
        m.insert("rage",          "😡");
        m.insert("cry",           "😢");
        m.insert("persevere",     "😣");
        m.insert("triumph",       "😤");
        m.insert("disappointed_relieved", "😥");
        m.insert("fearful",       "😨");
        m.insert("weary",         "😩");
        m.insert("sleepy",        "😪");
        m.insert("tired_face",    "😫");
        m.insert("sob",           "😭");
        m.insert("cold_sweat",    "😰");
        m.insert("scream",        "😱");
        m.insert("astonished",    "😲");
        m.insert("flushed",       "😳");
        m.insert("dizzy_face",    "😵");
        m.insert("mask",          "😷");
        m.insert("smile_cat",     "😸");
        m.insert("joy_cat",       "😹");
        m.insert("smiley_cat",    "😺");
        m.insert("heart_eyes_cat","😻");
        m.insert("smirk_cat",     "😼");
        m.insert("kissing_cat",   "😽");
        m.insert("pouting_cat",   "😾");
        m.insert("crying_cat_face","😿");
        m.insert("scream_cat",    "🙀");
        m.insert("no_good",       "🙅");
        m.insert("ok_woman",      "🙆");
        m.insert("bow",           "🙇");
        m.insert("see_no_evil",   "🙈");
        m.insert("hear_no_evil",  "🙉");
        m.insert("speak_no_evil", "🙊");
        m.insert("raising_hand",  "🙋");
        m.insert("raised_hands",  "🙌");
        m.insert("person_frowning", "🙍");
        m.insert("person_with_pouting_face", "🙎");
        m.insert("pray",          "🙏");

        // Dingbats
        m.insert("scissors",         "✂️");
        m.insert("white_check_mark", "✅");
        m.insert("airplane",         "✈");
        m.insert("envelope",         "✉");
        m.insert("fist",             "✊");
        m.insert("hand",             "✋");
        m.insert("v",                "✌");
        m.insert("pencil2",          "✏");
        m.insert("black_nib",        "✒");
        m.insert("heavy_check_mark", "✔");
        m.insert("heavy_multiplication_x", "✖");
        m.insert("sparkles",         "✨");
        m.insert("eight_spoked_asterisk", "✳");
        m.insert("eight_pointed_black_star", "✴");
        m.insert("snowflake",        "❄");
        m.insert("sparkle",          "❇");
        m.insert("x",                "❌");
        m.insert("negative_squared_cross_mark", "❎");
        m.insert("question",         "❓");
        m.insert("grey_question",    "❔");
        m.insert("grey_exclamation", "❕");
        m.insert("exclamation",      "❗");
        m.insert("heart",            "❤");
        m.insert("heavy_plus_sign",  "➕");
        m.insert("heavy_minus_sign", "➖");
        m.insert("heavy_division_sign", "➗");
        m.insert("arrow_right",      "➡");
        m.insert("curly_loop",       "➰");
        m
    };
}

fn replace_emojis(string: String) -> String {
    let pattern = Regex::new(r":([a-z0-9_]+?):").unwrap();
    pattern.replace_all(&string, |cap: &Captures| {
        let key = cap.at(1).unwrap();
        match MAP.get(key) {
            Some(&value) => {
                let mut p = value.to_owned();
                p.push(' ');
                p
            },
            None => cap.at(0).unwrap().to_owned()
        }
    })
}

fn main() {
    let string = match env::args().skip(1).next() {
        Some(mut str) => {
            str.push('\n');
            str
        },
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        }
    };
    let bytes = replace_emojis(string).into_bytes();
    io::stdout().write(&bytes[..]).unwrap();
}
