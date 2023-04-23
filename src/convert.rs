use crate::functions::{identify_type, make_hash_map};
use crate::read_mappings::Script;

pub fn convert_indic_to_roman(input: &String, source: &Script, destination: &Script) -> String {
    // Make a hashmap from source characters to corresponding destination ones
    // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.

    let _hash_map = make_hash_map(source, destination, 0);
    let hash_map_consonants_main = make_hash_map(source, destination, 1);
    let hash_map_vowels_main = make_hash_map(source, destination, 2);
    let hash_map_vowelsigns_main = make_hash_map(source, destination, 3);
    let hash_map_vowelsigns_virama = make_hash_map(source, destination, 4);
    let hash_map_numerals = make_hash_map(source, destination, 5);
    let hash_map_others_aytham = make_hash_map(source, destination, 6);
    let hash_map_combiningsigns_ayogavaha = make_hash_map(source, destination, 7);
    let hash_map_others_symbols = make_hash_map(source, destination, 8);

    let mut output: String = String::new();
    let len = input.chars().count();
    let chars: Vec<char> = input.chars().collect();
    for i in 0..len {
        let c: char;
        if true {
            c = chars[i];
            let s = &c.clone().to_string();
            let (t, _pos) = identify_type(&c.to_string(), source);
            match t.as_str() {
                "consonants.main" => {
                    // If there is a next character
                    if i < len - 1 {
                        match identify_type(&chars[i + 1].to_string(), source).0.as_str() {
                            // If the next character is a virama or vowelsign or symbol or it is a vowel
                            "vowelsigns.virama" | "vowelsigns.main" | "others.symbols"
                            | "numerals" | "vowels.main" => {
                                output.push_str(hash_map_consonants_main.get(s.as_str()).unwrap());
                            }
                            // Otherwise add the schwa also to the character
                            _ => {
                                output.push_str(hash_map_consonants_main.get(s.as_str()).unwrap());
                                output.push_str(destination.vowels.main[0].as_str());
                            }
                        }
                    } else {
                        output.push_str(hash_map_consonants_main.get(s.as_str()).unwrap());
                    }
                }
                "vowels.main" => {
                    output.push_str(hash_map_vowels_main.get(s.as_str()).unwrap());
                }
                "vowelsigns.main" => {
                    output.push_str(hash_map_vowelsigns_main.get(s.as_str()).unwrap());
                }
                "vowelsigns.virama" => {
                    output.push_str(hash_map_vowelsigns_virama.get(s.as_str()).unwrap());
                }
                "others.symbols" => {
                    output.push_str(hash_map_others_symbols.get(s.as_str()).unwrap());
                }
                "others.aytham" => {
                    output.push_str(hash_map_others_aytham.get(s.as_str()).unwrap());
                }
                "combiningsigns.ayogavaha" => {
                    output.push_str(hash_map_combiningsigns_ayogavaha.get(s.as_str()).unwrap());
                }
                "space" => {
                    output.push_str(" ");
                }
                "new-line" => {
                    output.push_str("\n");
                }
                "numerals" => {
                    output.push_str(hash_map_numerals.get(s.as_str()).unwrap());
                }
                _ => {}
            };
        }
    }

    output.replace("×", "")
}

pub fn convert_roman_to_roman(input: &String, source: &Script, destination: &Script) -> String {
    // Make a hashmap from source characters to corresponding destination ones
    // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.
    let hash_map_consonants_main = make_hash_map(source, destination, 1);
    let hash_map_vowels_main = make_hash_map(source, destination, 2);
    let hash_map_vowelsigns_main = make_hash_map(source, destination, 3);
    let hash_map_vowelsigns_virama = make_hash_map(source, destination, 4);
    let hash_map_numerals = make_hash_map(source, destination, 5);
    let hash_map_others_aytham = make_hash_map(source, destination, 6);
    let hash_map_combiningsigns_ayogavaha = make_hash_map(source, destination, 7);
    let hash_map_others_symbols = make_hash_map(source, destination, 8);
    // The logic for transliteration is very simple. First we check if the next two characters have a mapping.
    // If they have we push the corresponding destination string to the output.
    // If they don't we check the mapping for the current character only and push the corresponding destination string to the output.
    let mut output: String = String::new();
    let mut skip = false;
    let len = input.chars().count();
    let chars: Vec<char> = input.chars().collect();
    for i in 0..len {
        if skip {
            skip = false;
            continue;
        }
        let s = &mut String::new();
        let foo = &mut String::new();
        {
            let (t, _pos): (String, usize);

            if i < len - 1 {
                foo.push_str(&chars[i].to_string());
                foo.push_str(&chars[i + 1].to_string());
                if identify_type(foo, source).0 == "could.not.identify" {
                    // println!("false| {:?}", foo);
                    s.push_str(&chars[i].to_string());
                } else {
                    println!("true| {:?} | ", foo);
                    s.push_str(&chars[i].to_string());
                    s.push_str(&chars[i + 1].to_string());
                    skip = true;
                }
            } else {
                s.push_str(&chars[i].to_string());
            }
            (t, _pos) = identify_type(s, source);
            if skip {println!("{:?}", t);}
            match t.as_str() {
                "consonants.main" => {
                    output.push_str(hash_map_consonants_main.get(s.as_str()).unwrap());
                }
                "vowels.main" => {
                    output.push_str(hash_map_vowels_main.get(s.as_str()).unwrap());
                }
                "vowelsigns.main" => {
                    output.push_str(hash_map_vowelsigns_main.get(s.as_str()).unwrap());
                }
                "vowelsigns.virama" => {
                    output.push_str(hash_map_vowelsigns_virama.get(s.as_str()).unwrap());
                }
                "others.symbols" => {
                    output.push_str(hash_map_others_symbols.get(s.as_str()).unwrap());
                }
                "others.aytham" => {
                    output.push_str(hash_map_others_aytham.get(s.as_str()).unwrap());
                }
                "combiningsigns.ayogavaha" => {
                    output.push_str(hash_map_combiningsigns_ayogavaha.get(s.as_str()).unwrap());
                }
                "space" => {
                    output.push_str(" ");
                }
                "new-line" => {
                    output.push_str("\n");
                }
                "numerals" => {
                    output.push_str(hash_map_numerals.get(s.as_str()).unwrap());
                }
                _ => {}
            };
        }
    }
    output
}

pub fn convert_roman_to_indic(input: &String, source: &Script, destination: &Script) -> String {
    // Make a hashmap from source characters to corresponding destination ones
    // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.

    let hash_map_consonants_main = make_hash_map(source, destination, 1);
    let hash_map_vowels_main = make_hash_map(source, destination, 2);
    let hash_map_vowelsigns_main = make_hash_map(source, destination, 3);
    let hash_map_vowelsigns_virama = make_hash_map(source, destination, 4);
    let hash_map_numerals = make_hash_map(source, destination, 5);
    let hash_map_others_aytham = make_hash_map(source, destination, 6);
    let hash_map_combiningsigns_ayogavaha = make_hash_map(source, destination, 7);
    let hash_map_others_symbols = make_hash_map(source, destination, 8);

    let mut output: String = String::new();
    let len = input.chars().count();
    let chars: Vec<char> = input.chars().collect();
    for i in 0..len {
        let c: char;
        if true {
            c = chars[i];
            let s = &c.clone().to_string();
            let (t, _pos) = identify_type(&c.to_string(), source);
            match t.as_str() {
                "consonants.main" => {
                    // check if the next character in input is also a character
                    // if so, push a virama also
                    output.push_str(hash_map_consonants_main.get(s.as_str()).unwrap());
                    if i < len - 1 {
                        match identify_type(&chars[i + 1].to_string(), source).0.as_str() {
                            "consonants.main" | "space" | "new-line" | "numerals" => {
                                output.push_str(destination.vowelsigns.virama[0].as_str());
                            }
                            _ => {
                                // Do Nothing
                            }
                        }
                    }
                }
                "vowelsigns.main" | "vowels.main" => {
                    // println!("reached here");
                    // Check if the previous character in input was a consonant
                    // If so push a vowelsigns.main
                    // else push vowels.main
                    if i > 0 {
                        match identify_type(&chars[i - 1].to_string(), source).0.as_str() {
                            "consonants.main" => {
                                if let Some(_) = hash_map_vowelsigns_main.get(s.as_str()) {
                                    output.push_str(
                                        hash_map_vowelsigns_main.get(s.as_str()).unwrap(),
                                    );
                                } else {
                                    if chars[i].to_string() != source.vowels.main[0] {
                                        output.push_str(
                                            hash_map_vowels_main.get(s.as_str()).unwrap(),
                                        );
                                    }
                                }
                            }
                            _ => {
                                output.push_str(hash_map_vowels_main.get(s.as_str()).unwrap());
                            }
                        }
                    } else {
                        output.push_str(hash_map_vowels_main.get(s.as_str()).unwrap());
                    }
                }
                "vowelsigns.virama" => {
                    output.push_str(hash_map_vowelsigns_virama.get(s.as_str()).unwrap());
                }
                "others.symbols" => {
                    output.push_str(hash_map_others_symbols.get(s.as_str()).unwrap());
                }
                "others.aytham" => {
                    output.push_str(hash_map_others_aytham.get(s.as_str()).unwrap());
                }
                "combiningsigns.ayogavaha" => {
                    output.push_str(hash_map_combiningsigns_ayogavaha.get(s.as_str()).unwrap());
                }
                "space" => {
                    output.push_str(" ");
                }
                "new-line" => {
                    output.push_str("\n");
                }
                "numerals" => {
                    output.push_str(hash_map_numerals.get(s.as_str()).unwrap());
                }
                _ => {}
            };
        }
    }

    output
}
