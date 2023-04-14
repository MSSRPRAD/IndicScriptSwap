use crate::functions::{identify_type, make_hash_map};
use crate::read_mappings::Script;

pub fn convert_indic_to_roman(input: String, source: &Script, destination: &Script) -> String {
    // Make a hashmap from source characters to corresponding destination ones
    // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.

    let hash_map = make_hash_map(source, destination);
    let mut output: String = String::new();

    for i in 0..input.len() {
        let c: char;
        if let Some(_val) = input.chars().nth(i) {
            c = input.chars().nth(i).unwrap();
            let s = &c.clone().to_string();
            let (t, pos) = identify_type(c, source);
            match t.as_str() {
                "consonants.main" => {output.push_str(hash_map.get(s.as_str()).unwrap());}
                "vowels.main" => {
                    output.push_str(hash_map.get(s.as_str()).unwrap());
                }
                "vowelsigns.main" => {output.push_str(hash_map.get(s.as_str()).unwrap());}
                "vowelsigns.virama" => {output.push_str(hash_map.get(s.as_str()).unwrap());}
                "others.symbols" => {output.push_str(hash_map.get(s.as_str()).unwrap());}
                "others.aytham" => {output.push_str(hash_map.get(s.as_str()).unwrap());}
                _ => {}
            };
        }
    }

    return output;

    "Some Error Occured!".to_string()
}
