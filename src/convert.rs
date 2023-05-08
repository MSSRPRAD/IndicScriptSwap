use crate::functions::{identify_type, make_hash_map, CharType};
use crate::read_mappings::Script;

pub fn convert_indic_to_roman(input: &str, source: &Script, destination: &Script) -> String {
    // Make a hashmap from source characters to corresponding destination ones
    // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.

    // let _hash_map = make_hash_map(source, destination, 0);
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
            let t = identify_type(
                &c.to_string(),
                &hash_map_consonants_main,
                &hash_map_vowels_main,
                &hash_map_vowelsigns_main,
                &hash_map_vowelsigns_virama,
                &hash_map_numerals,
                &hash_map_others_aytham,
                &hash_map_combiningsigns_ayogavaha,
                &hash_map_others_symbols,
            );
            match t {
                CharType::ConsonantsMain => {
                    // If there is a next character
                    if i < len - 1 {
                        match identify_type(
                            &chars[i + 1].to_string(),
                            &hash_map_consonants_main,
                            &hash_map_vowels_main,
                            &hash_map_vowelsigns_main,
                            &hash_map_vowelsigns_virama,
                            &hash_map_numerals,
                            &hash_map_others_aytham,
                            &hash_map_combiningsigns_ayogavaha,
                            &hash_map_others_symbols,
                        ) {
                            // If the next character is a virama or vowelsign or symbol or it is a vowel
                            CharType::VowelSignsVirama
                            | CharType::VowelSignsMain
                            | CharType::OthersSymbols
                            | CharType::Numerals
                            | CharType::VowelsMain => {
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
                CharType::VowelsMain => {
                    output.push_str(hash_map_vowels_main.get(s.as_str()).unwrap());
                }
                CharType::VowelSignsMain => {
                    output.push_str(hash_map_vowelsigns_main.get(s.as_str()).unwrap());
                }
                CharType::VowelSignsVirama => {
                    output.push_str(hash_map_vowelsigns_virama.get(s.as_str()).unwrap());
                }
                CharType::OthersSymbols => {
                    output.push_str(hash_map_others_symbols.get(s.as_str()).unwrap());
                }
                CharType::OthersAytham => {
                    output.push_str(hash_map_others_aytham.get(s.as_str()).unwrap());
                }
                CharType::CombiningSignsAyogavaha => {
                    output.push_str(hash_map_combiningsigns_ayogavaha.get(s.as_str()).unwrap());
                }
                CharType::Space => {
                    output.push_str(" ");
                }
                CharType::NewLine => {
                    output.push_str("\n");
                }
                CharType::Numerals => {
                    output.push_str(hash_map_numerals.get(s.as_str()).unwrap());
                }
                CharType::CouldNotIdentify => {}
            };
        }
    }

    output.replace("×", "")
}

pub fn convert_roman_to_roman(input: &str, source: &Script, destination: &Script) -> String {
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
            let t: CharType;

            if i < len - 1 {
                foo.push_str(&chars[i].to_string());
                foo.push_str(&chars[i + 1].to_string());
                if identify_type(
                    foo,
                    &hash_map_consonants_main,
                    &hash_map_vowels_main,
                    &hash_map_vowelsigns_main,
                    &hash_map_vowelsigns_virama,
                    &hash_map_numerals,
                    &hash_map_others_aytham,
                    &hash_map_combiningsigns_ayogavaha,
                    &hash_map_others_symbols,
                ) == CharType::CouldNotIdentify
                {
                    // println!("false| {:?}", foo);
                    s.push_str(&chars[i].to_string());
                } else {
                    // println!("true| {:?} | ", foo);
                    s.push_str(&chars[i].to_string());
                    s.push_str(&chars[i + 1].to_string());
                    skip = true;
                }
            } else {
                s.push_str(&chars[i].to_string());
            }
            t = identify_type(
                s,
                &hash_map_consonants_main,
                &hash_map_vowels_main,
                &hash_map_vowelsigns_main,
                &hash_map_vowelsigns_virama,
                &hash_map_numerals,
                &hash_map_others_aytham,
                &hash_map_combiningsigns_ayogavaha,
                &hash_map_others_symbols,
            );
            // if skip {
            //     println!("{:?}", t);
            // }
            match t {
                CharType::ConsonantsMain => {
                    output.push_str(hash_map_consonants_main.get(s.as_str()).unwrap());
                }
                CharType::VowelsMain => {
                    output.push_str(hash_map_vowels_main.get(s.as_str()).unwrap());
                }
                CharType::VowelSignsMain => {
                    output.push_str(hash_map_vowelsigns_main.get(s.as_str()).unwrap());
                }
                CharType::VowelSignsVirama => {
                    output.push_str(hash_map_vowelsigns_virama.get(s.as_str()).unwrap());
                }
                CharType::OthersSymbols => {
                    output.push_str(hash_map_others_symbols.get(s.as_str()).unwrap());
                }
                CharType::OthersAytham => {
                    output.push_str(hash_map_others_aytham.get(s.as_str()).unwrap());
                }
                CharType::CombiningSignsAyogavaha => {
                    output.push_str(hash_map_combiningsigns_ayogavaha.get(s.as_str()).unwrap());
                }
                CharType::Space => {
                    output.push_str(" ");
                }
                CharType::NewLine => {
                    output.push_str("\n");
                }
                CharType::Numerals => {
                    output.push_str(hash_map_numerals.get(s.as_str()).unwrap());
                }
                _ => {}
            };
        }
    }
    output
}

pub fn convert_roman_to_indic(input: &str, source: &Script, destination: &Script) -> String {
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
            let t = identify_type(
                &c.to_string(),
                &hash_map_consonants_main,
                &hash_map_vowels_main,
                &hash_map_vowelsigns_main,
                &hash_map_vowelsigns_virama,
                &hash_map_numerals,
                &hash_map_others_aytham,
                &hash_map_combiningsigns_ayogavaha,
                &hash_map_others_symbols,
            );
            match t {
                CharType::ConsonantsMain => {
                    // check if the next character in input is also a character
                    // if so, push a virama also
                    output.push_str(hash_map_consonants_main.get(s.as_str()).unwrap());
                    if i < len - 1 {
                        match identify_type(
                            &chars[i + 1].to_string(),
                            &hash_map_consonants_main,
                            &hash_map_vowels_main,
                            &hash_map_vowelsigns_main,
                            &hash_map_vowelsigns_virama,
                            &hash_map_numerals,
                            &hash_map_others_aytham,
                            &hash_map_combiningsigns_ayogavaha,
                            &hash_map_others_symbols,
                        ) {
                            CharType::ConsonantsMain
                            | CharType::Space
                            | CharType::NewLine
                            | CharType::Numerals => {
                                output.push_str(destination.vowelsigns.virama[0].as_str());
                            }
                            _ => {
                                // Do Nothing
                            }
                        }
                    }
                }
                CharType::VowelSignsMain | CharType::VowelsMain => {
                    // println!("reached here");
                    // Check if the previous character in input was a consonant
                    // If so push a vowelsigns.main
                    // else push vowels.main
                    if i > 0 {
                        match identify_type(
                            &chars[i - 1].to_string(),
                            &hash_map_consonants_main,
                            &hash_map_vowels_main,
                            &hash_map_vowelsigns_main,
                            &hash_map_vowelsigns_virama,
                            &hash_map_numerals,
                            &hash_map_others_aytham,
                            &hash_map_combiningsigns_ayogavaha,
                            &hash_map_others_symbols,
                        ) {
                            CharType::ConsonantsMain => {
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
                CharType::VowelSignsVirama => {
                    output.push_str(hash_map_vowelsigns_virama.get(s.as_str()).unwrap());
                }
                CharType::OthersSymbols => {
                    output.push_str(hash_map_others_symbols.get(s.as_str()).unwrap());
                }
                CharType::OthersAytham => {
                    output.push_str(hash_map_others_aytham.get(s.as_str()).unwrap());
                }
                CharType::CombiningSignsAyogavaha => {
                    output.push_str(hash_map_combiningsigns_ayogavaha.get(s.as_str()).unwrap());
                }
                CharType::Space => {
                    output.push_str(" ");
                }
                CharType::NewLine => {
                    output.push_str("\n");
                }
                CharType::Numerals => {
                    output.push_str(hash_map_numerals.get(s.as_str()).unwrap());
                }
                _ => {}
            };
        }
    }

    output
}
