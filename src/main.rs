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
        let mut map = BTreeMap::new();

        // Emoticons
        map.insert("grin",          "😁");
        map.insert("joy",           "😂");
        map.insert("smiley",        "😃");
        map.insert("smile",         "😄");
        map.insert("sweat_smile",   "😅");
        map.insert("satisfied",     "😆");
        map.insert("wink",          "😉");
        map.insert("relaxed",       "😊");
        map.insert("yum",           "😋");
        map.insert("relieved",      "😌");
        map.insert("heart_eyes",    "😍");
        map.insert("smirk",         "😏");
        map.insert("unamused",      "😒");
        map.insert("sweat",         "😓");
        map.insert("pensive",       "😔");
        map.insert("confounded",    "😖");
        map.insert("kissing_heart", "😘");
        map.insert("kissing_closed_eyes", "😚");
        map.insert("stuck_out_tongue_winking_eye", "😜");
        map.insert("stuck_out_tongue_closed_eyes", "😝");
        map.insert("disappointed",  "😞");
        map.insert("angry",         "😠");
        map.insert("rage",          "😡");
        map.insert("cry",           "😢");
        map.insert("persevere",     "😣");
        map.insert("triumph",       "😤");
        map.insert("disappointed_relieved", "😥");
        map.insert("fearful",       "😨");
        map.insert("weary",         "😩");
        map.insert("sleepy",        "😪");
        map.insert("tired_face",    "😫");
        map.insert("sob",           "😭");
        map.insert("cold_sweat",    "😰");
        map.insert("scream",        "😱");
        map.insert("astonished",    "😲");
        map.insert("flushed",       "😳");
        map.insert("dizzy_face",    "😵");
        map.insert("mask",          "😷");
        map.insert("smile_cat",     "😸");
        map.insert("joy_cat",       "😹");
        map.insert("smiley_cat",    "😺");
        map.insert("heart_eyes_cat","😻");
        map.insert("smirk_cat",     "😼");
        map.insert("kissing_cat",   "😽");
        map.insert("pouting_cat",   "😾");
        map.insert("crying_cat_face","😿");
        map.insert("scream_cat",    "🙀");
        map.insert("no_good",       "🙅");
        map.insert("ok_woman",      "🙆");
        map.insert("bow",           "🙇");
        map.insert("see_no_evil",   "🙈");
        map.insert("hear_no_evil",  "🙉");
        map.insert("speak_no_evil", "🙊");
        map.insert("raising_hand",  "🙋");
        map.insert("raised_hands",  "🙌");
        map.insert("person_frowning", "🙍");
        map.insert("person_with_pouting_face", "🙎");
        map.insert("pray",          "🙏");

        // Dingbats
        map.insert("scissors",         "✂️");
        map.insert("white_check_mark", "✅");
        map.insert("airplane",         "✈");
        map.insert("envelope",         "✉");
        map.insert("fist",             "✊");
        map.insert("hand",             "✋");
        map.insert("v",                "✌");
        map.insert("pencil2",          "✏");
        map.insert("black_nib",        "✒");
        map.insert("heavy_check_mark", "✔");
        map.insert("heavy_multiplication_x", "✖");
        map.insert("sparkles",         "✨");
        map.insert("eight_spoked_asterisk", "✳");
        map.insert("eight_pointed_black_star", "✴");
        map.insert("snowflake",        "❄");
        map.insert("sparkle",          "❇");
        map.insert("x",                "❌");
        map.insert("negative_squared_cross_mark", "❎");
        map.insert("question",         "❓");
        map.insert("grey_question",    "❔");
        map.insert("grey_exclamation", "❕");
        map.insert("exclamation",      "❗");
        map.insert("heart",            "❤");
        map.insert("heavy_plus_sign",  "➕");
        map.insert("heavy_minus_sign", "➖");
        map.insert("heavy_division_sign", "➗");
        map.insert("arrow_right",      "➡");
        map.insert("curly_loop",       "➰");
        map
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
