#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use regex::Regex;
use regex::Captures;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r":([a-z0-9_]+?):").unwrap();
    static ref MAP: BTreeMap<&'static str, &'static str> = {
        let mut M = BTreeMap::new();

        // Emoticons
        M.insert("grin",          "😁");
        M.insert("joy",           "😂");
        M.insert("smiley",        "😃");
        M.insert("smile",         "😄");
        M.insert("sweat_smile",   "😅");
        M.insert("satisfied",     "😆");
        M.insert("wink",          "😉");
        M.insert("relaxed",       "😊");
        M.insert("yum",           "😋");
        M.insert("relieved",      "😌");
        M.insert("heart_eyes",    "😍");
        M.insert("smirk",         "😏");
        M.insert("unamused",      "😒");
        M.insert("sweat",         "😓");
        M.insert("pensive",       "😔");
        M.insert("confounded",    "😖");
        M.insert("kissing_heart", "😘");
        M.insert("kissing_closed_eyes", "😚");
        M.insert("stuck_out_tongue_winking_eye", "😜");
        M.insert("stuck_out_tongue_closed_eyes", "😝");
        M.insert("disappointed",  "😞");
        M.insert("angry",         "😠");
        M.insert("rage",          "😡");
        M.insert("cry",           "😢");
        M.insert("persevere",     "😣");
        M.insert("triumph",       "😤");
        M.insert("disappointed_relieved", "😥");
        M.insert("fearful",       "😨");
        M.insert("weary",         "😩");
        M.insert("sleepy",        "😪");
        M.insert("tired_face",    "😫");
        M.insert("sob",           "😭");
        M.insert("cold_sweat",    "😰");
        M.insert("scream",        "😱");
        M.insert("astonished",    "😲");
        M.insert("flushed",       "😳");
        M.insert("dizzy_face",    "😵");
        M.insert("mask",          "😷");
        M.insert("smile_cat",     "😸");
        M.insert("joy_cat",       "😹");
        M.insert("smiley_cat",    "😺");
        M.insert("heart_eyes_cat","😻");
        M.insert("smirk_cat",     "😼");
        M.insert("kissing_cat",   "😽");
        M.insert("pouting_cat",   "😾");
        M.insert("crying_cat_face","😿");
        M.insert("scream_cat",    "🙀");
        M.insert("no_good",       "🙅");
        M.insert("ok_woman",      "🙆");
        M.insert("bow",           "🙇");
        M.insert("see_no_evil",   "🙈");
        M.insert("hear_no_evil",  "🙉");
        M.insert("speak_no_evil", "🙊");
        M.insert("raising_hand",  "🙋");
        M.insert("raised_hands",  "🙌");
        M.insert("person_frowning", "🙍");
        M.insert("person_with_pouting_face", "🙎");
        M.insert("pray",          "🙏");

        // Dingbats
        M.insert("scissors",         "✂️");
        M.insert("white_check_mark", "✅");
        M.insert("airplane",         "✈");
        M.insert("envelope",         "✉");
        M.insert("fist",             "✊");
        M.insert("hand",             "✋");
        M.insert("v",                "✌");
        M.insert("pencil2",          "✏");
        M.insert("black_nib",        "✒");
        M.insert("heavy_check_mark", "✔");
        M.insert("heavy_multiplication_x", "✖");
        M.insert("sparkles",         "✨");
        M.insert("eight_spoked_asterisk", "✳");
        M.insert("eight_pointed_black_star", "✴");
        M.insert("snowflake",        "❄");
        M.insert("sparkle",          "❇");
        M.insert("x",                "❌");
        M.insert("negative_squared_cross_mark", "❎");
        M.insert("question",         "❓");
        M.insert("grey_question",    "❔");
        M.insert("grey_exclamation", "❕");
        M.insert("exclamation",      "❗");
        M.insert("heart",            "❤");
        M.insert("heavy_plus_sign",  "➕");
        M.insert("heavy_minus_sign", "➖");
        M.insert("heavy_division_sign", "➗");
        M.insert("arrow_right",      "➡");
        M.insert("curly_loop",       "➰");
        M
    };
}

fn replace_emojis(string: String) -> String {
    PATTERN.replace_all(&string, |cap: &Captures| {
        let key = cap.at(1).unwrap();
        match MAP.get(key) {
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
    let output = match env::args().skip(1).next() {
        Some(string) => {
            let mut str = replace_emojis(string);
            str.push('\n');
            str
        },
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            replace_emojis(buffer)
        }
    };
    let bytes = output.into_bytes();
    io::stdout().write(&bytes[..]).unwrap();
}
