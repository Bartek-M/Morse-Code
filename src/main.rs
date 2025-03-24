use clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::HashMap;
use std::io::{self, Write};

mod dictionary;


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

fn main() {
    println!("Morse Translator (type 'exit' to quit)");
    let (text_to_morse, morse_to_text) = dictionary::morse_dict();
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

    loop {
        print!("\nEnter text or Morse: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the input");
        input = input.trim().to_uppercase();
        
        if input == "EXIT" || input == "QUIT" {
            break;
        } else if input.chars().all(|c| c == '.' || c == '-' || c == '/' || c == ' ') {
            let result = decode(input.clone(), morse_to_text.clone());
            println!("Decoded text: {}", result);
            clipboard.set_contents(result).unwrap();
        } else {
            let result = encode(input, text_to_morse.clone());
            println!("Morse Code: {}", result);
            clipboard.set_contents(result).unwrap();
        }
    }
}