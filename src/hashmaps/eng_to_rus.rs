use std::collections::HashMap;

pub fn init() -> HashMap<char, char> {
    let mut eng_to_rus = HashMap::new();

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

    return eng_to_rus;
}
