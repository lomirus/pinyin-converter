mod data;
mod utils;

use crate::{data::SYLLABLE_DATA, utils::WordCase};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub(crate) struct Syllable<'a> {
    pinyin: [&'a str; 5],
    romatzyh: [&'a str; 5],
    pinyin_ascii: [&'a str; 5],
}

#[wasm_bindgen]
pub enum SyllableDisplay {
    Pinyin,
    Romatzyh,
    PinyinAscii,
}

fn convert_syllable_pinyin_ascii(source_str: String, target: &SyllableDisplay) -> String {
    let word_case = WordCase::from(source_str.as_str());
    let pinyin_ascii = source_str.to_ascii_lowercase();
    let syllable = SYLLABLE_DATA
        .iter()
        .find(|item| item.pinyin_ascii.contains(&pinyin_ascii.as_str()))
        .unwrap();

    for i in 0..5 {
        if syllable.pinyin_ascii[i] == pinyin_ascii {
            let result = match target {
                SyllableDisplay::Pinyin => syllable.pinyin[i],
                SyllableDisplay::Romatzyh => syllable.romatzyh[i],
                SyllableDisplay::PinyinAscii => syllable.pinyin_ascii[i],
            }
            .to_string();
            return match word_case {
                WordCase::Capitalized => utils::capitalize(result),
                WordCase::Lowercase => result,
                WordCase::Uppercase => result.to_ascii_uppercase(),
            };
        }
    }
    unreachable!()
}

#[wasm_bindgen]
pub fn convert_pinyin_ascii(input: String, target: SyllableDisplay) -> String {
    let input: Vec<char> = input.chars().collect();
    let mut output: Vec<String> = Vec::new();
    let mut i = 0;
    while i < input.len() {
        if !input[i].is_ascii_alphabetic() {
            output.push(input[i].to_string());
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < input.len() {
            if input[j].is_ascii_digit() {
                let syllable = input.iter().skip(i).take(j - i + 1).collect::<String>();
                output.push(convert_syllable_pinyin_ascii(syllable, &target));
                break;
            } else if !input[j].is_ascii_alphabetic() {
                let letters = input.iter().skip(i).take(j - i + 1).collect::<String>();
                output.push(letters);
                break;
            }
            j += 1;
        }
        if j == input.len() {
            let letters = input.iter().skip(i).take(j - i + 1).collect::<String>();
            output.push(letters);
        }
        i = j + 1;
    }

    output.into_iter().collect()
}
