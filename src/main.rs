use std::collections::HashMap;

mod hashmaps;
mod utils;

#[macro_use]
extern crate text_io;
fn main() {
    print!("Select language (rus, eng): ");
    let translation_to: String = read!();
    
    let _: String = read!("{}\n");

    print!("Input: ");
    let text: String = read!("{}\n");

    let output = if translation_to == "eng" {
        translate(&text, &hashmaps::rus_to_eng::init())
    } else if translation_to == "rus" {
        translate(&text, &hashmaps::eng_to_rus::init())
    } else {
        println!("Invalid option");
        return;
    };

    println!("Output: {}", output);
    utils::clipboard::copy(output);
}

fn translate(text: &str, char_map: &HashMap<char, char>) -> String {
    text.chars()
        .map(|c| *char_map.get(&c).unwrap_or(&c))
        .collect()
}