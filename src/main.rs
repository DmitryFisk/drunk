use std::collections::HashMap;

#[macro_use]
extern crate text_io;
fn main() {
    let mut rus_to_eng = HashMap::new();
    let mut eng_to_rus = HashMap::new();

    // Russian to English mappings (uppercase and lowercase)
    rus_to_eng.insert('А', 'A');
    rus_to_eng.insert('а', 'a');
    rus_to_eng.insert('В', 'B');
    rus_to_eng.insert('Е', 'E');
    rus_to_eng.insert('е', 'e');
    rus_to_eng.insert('К', 'K');
    rus_to_eng.insert('М', 'M');
    rus_to_eng.insert('Н', 'H');
    rus_to_eng.insert('О', 'O');
    rus_to_eng.insert('о', 'o');
    rus_to_eng.insert('Р', 'P');
    rus_to_eng.insert('р', 'p');
    rus_to_eng.insert('С', 'C');
    rus_to_eng.insert('с', 'c');
    rus_to_eng.insert('Т', 'T');
    rus_to_eng.insert('у', 'y');
    rus_to_eng.insert('Х', 'X');
    rus_to_eng.insert('х', 'x');

    // English to Russian mappings (uppercase and lowercase)
    eng_to_rus.insert('A', 'А');
    eng_to_rus.insert('a', 'а');
    eng_to_rus.insert('B', 'В');
    eng_to_rus.insert('E', 'Е');
    eng_to_rus.insert('e', 'е');
    eng_to_rus.insert('K', 'К');
    eng_to_rus.insert('M', 'М');
    eng_to_rus.insert('H', 'Н');
    eng_to_rus.insert('O', 'О');
    eng_to_rus.insert('o', 'о');
    eng_to_rus.insert('P', 'Р');
    eng_to_rus.insert('p', 'р');
    eng_to_rus.insert('C', 'С');
    eng_to_rus.insert('c', 'с');
    eng_to_rus.insert('T', 'Т');
    eng_to_rus.insert('y', 'у');
    eng_to_rus.insert('X', 'Х');
    eng_to_rus.insert('x', 'х');

    println!("Select option (rus to translate to Russian, eng to translate to English): ");
    let translation_to: String = read!();
    
    let _: String = read!("{}\n");

    println!("Input:");
    let text: String = read!("{}\n");

    let output = if translation_to == "eng" {
        translate(&text, &rus_to_eng)
    } else if translation_to == "rus" {
        translate(&text, &eng_to_rus)
    } else {
        "Invalid option".to_string()
    };

    println!("Output: {}", output);
}

fn translate(text: &str, char_map: &HashMap<char, char>) -> String {
    text.chars()
        .map(|c| *char_map.get(&c).unwrap_or(&c))
        .collect()
}