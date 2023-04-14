use crate::functions::{identify_type, make_hash_map};
use crate::read_mappings::Script;

pub fn convert_indic_to_roman(input: &String, source: &Script, destination: &Script) -> String {
    // Make a hashmap from source characters to corresponding destination ones
    // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.

    let hash_map = make_hash_map(source, destination);
    let mut output: String = String::new();

    for i in 0..input.len() {
        let c: char;
        if let Some(_val) = input.chars().nth(i) {
            c = input.chars().nth(i).unwrap();
            let s = &c.clone().to_string();
            let (t, _pos) = identify_type(c, source);
            match t.as_str() {
                "consonants.main" => {
                    // If there is a next character
                    if let Some(_val1) = input.chars().nth(i + 1) {
                        match identify_type(input.chars().nth(i + 1).unwrap(), source)
                            .0
                            .as_str()
                        {
                            // If the next character is a virama or vowelsign or symbol or it is a vowel
                            "vowelsigns.virama" | "vowelsigns.main" | "others.symbols"
                            | "numerals" | "vowels.main" => {
                                output.push_str(hash_map.get(s.as_str()).unwrap());
                            }
                            // Otherwise add the schwa also to the character
                            _ => {
                                output.push_str(hash_map.get(s.as_str()).unwrap());
                                output.push_str("a");
                            }
                        }
                    } else {
                        output.push_str(hash_map.get(s.as_str()).unwrap());
                    }
                }
                "vowels.main"
                | "vowelsigns.main"
                | "vowelsigns.virama"
                | "others.symbols"
                | "others.aytham"
                | "combiningsigns.ayogavaha" => {
                    output.push_str(hash_map.get(s.as_str()).unwrap());
                }
                _ => {}
            };
        }
    }
    
    output.replace("×", "")
}
