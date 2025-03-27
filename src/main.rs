use clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::HashMap;
use std::io::{self, Write};

mod dictionary;
mod audio;
mod parser;


fn encode(text: String, dict: HashMap<char, &'static str>) -> String {
    return text.chars()
        .filter_map(|c| dict.get(&c).copied())
        .collect::<Vec<&str>>()
        .join(" ");
}

fn decode(text: String, dict: HashMap<&'static str, char>) -> String {
    return text.split(" ")
        .filter_map(|c| dict.get(&c).copied())
        .collect::<String>()
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    println!("Morse Translator (type 'exit' to quit)");
}

fn main() {
    let (text_to_morse, morse_to_text) = dictionary::morse_dict();
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    clear();
    println!("Decoded Audio: {}", decode(parser::input(), morse_to_text.clone()));
    
    loop {
        print!("\nEnter text or Morse: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the input");
        input = input.trim().to_uppercase();
        let mut result = String::new();
        
        match input.as_str() {
            "EXIT" | "QUIT" => break,
            "CLEAR" => clear(),
            _ if input.chars().all(|c| c == '.' || c == '-' || c == '/' || c == ' ') => {
                result = decode(input.clone(), morse_to_text.clone());
                println!("Decoded text: {}", result);
            }
            _ => {
                result = encode(input, text_to_morse.clone());
                println!("Morse Code: {}", result);
                audio::output(&result);
            }
        }

        clipboard.set_contents(result).unwrap();
    }
}