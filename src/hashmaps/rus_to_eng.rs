use std::collections::HashMap;

pub fn init() -> HashMap<char, char> {
    let mut rus_to_eng = HashMap::new();

    //  Russian to English mappings (uppercase and lowercase)
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

    return rus_to_eng;
}