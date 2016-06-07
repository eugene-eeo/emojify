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

    // Emoticons
    M!("grin",          "😁");
    M!("joy",           "😂");
    M!("smiley",        "😃");
    M!("smile",         "😄");
    M!("sweat_smile",   "😅");
    M!("satisfied",     "😆");
    M!("wink",          "😉");
    M!("relaxed",       "😊");
    M!("yum",           "😋");
    M!("relieved",      "😌");
    M!("heart_eyes",    "😍");
    M!("smirk",         "😏");
    M!("unamused",      "😒");
    M!("sweat",         "😓");
    M!("pensive",       "😔");
    M!("confounded",    "😖");
    M!("kissing_heart", "😘");
    M!("kissing_closed_eyes", "😚");
    M!("stuck_out_tongue_winking_eye", "😜");
    M!("stuck_out_tongue_closed_eyes", "😝");
    M!("disappointed",  "😞");
    M!("angry",         "😠");
    M!("rage",          "😡");
    M!("cry",           "😢");
    M!("persevere",     "😣");
    M!("triumph",       "😤");
    M!("disappointed_relieved", "😥");
    M!("fearful",       "😨");
    M!("weary",         "😩");
    M!("sleepy",        "😪");
    M!("tired_face",    "😫");
    M!("sob",           "😭");
    M!("cold_sweat",    "😰");
    M!("scream",        "😱");
    M!("astonished",    "😲");
    M!("flushed",       "😳");
    M!("dizzy_face",    "😵");
    M!("mask",          "😷");
    M!("smile_cat",     "😸");
    M!("joy_cat",       "😹");
    M!("smiley_cat",    "😺");
    M!("heart_eyes_cat","😻");
    M!("smirk_cat",     "😼");
    M!("kissing_cat",   "😽");
    M!("pouting_cat",   "😾");
    M!("crying_cat_face","😿");
    M!("scream_cat",    "🙀");
    M!("no_good",       "🙅");
    M!("ok_woman",      "🙆");
    M!("bow",           "🙇");
    M!("see_no_evil",   "🙈");
    M!("hear_no_evil",  "🙉");
    M!("speak_no_evil", "🙊");
    M!("raising_hand",  "🙋");
    M!("raised_hands",  "🙌");
    M!("person_frowning", "🙍");
    M!("person_with_pouting_face", "🙎");
    M!("pray",          "🙏");

    // Dingbats
    M!("scissors",         "✂️");
    M!("white_check_mark", "✅");
    M!("airplane",         "✈");
    M!("envelope",         "✉");
    M!("fist",             "✊");
    M!("hand",             "✋");
    M!("v",                "✌");
    M!("pencil2",          "✏");
    M!("black_nib",        "✒");
    M!("heavy_check_mark", "✔");
    M!("heavy_multiplication_x", "✖");
    M!("sparkles",         "✨");
    M!("eight_spoked_asterisk", "✳");
    M!("eight_pointed_black_star", "✴");
    M!("snowflake",        "❄");
    M!("sparkle",          "❇");
    M!("x",                "❌");
    M!("negative_squared_cross_mark", "❎");
    M!("question",         "❓");
    M!("grey_question",    "❔");
    M!("grey_exclamation", "❕");
    M!("exclamation",      "❗");
    M!("heart",            "❤");
    M!("heavy_plus_sign",  "➕");
    M!("heavy_minus_sign", "➖");
    M!("heavy_division_sign", "➗");
    M!("arrow_right",      "➡");
    M!("curly_loop",       "➰");

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
