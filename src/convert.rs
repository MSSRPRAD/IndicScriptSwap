use crate::functions::{identify_type, make_hash_map, CharType};
use crate::read_mappings::Script;

pub fn convert_indic_to_indic(input: &str, source: &Script, destination: &Script) -> String {
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
    // Transliterating indic to indic is very simple. We just map the current character to the
    // corresponding destination character and it works!
    let mut output: String = String::new();
    let len = input.chars().count();
    let chars: Vec<char> = input.chars().collect();
    for i in 0..len {
        let s = &mut String::new();
        s.push(chars[i]);
        {
            let t: CharType;

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
    // Conversion is a bit more complicated here. This is because there is often a hidden 'a' after
    // most consonants. We need to check this case by seeing the next character after a consonant.
    let mut output: String = String::new();
    let len = input.chars().count();
    let chars: Vec<char> = input.chars().collect();
    for i in 0..len {
        let s = &mut String::new();
        s.push(chars[i]);
        let t = identify_type(
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
        match t {
            CharType::ConsonantsMain => {
                // Push the corresponding destination consonant
                output.push_str(hash_map_consonants_main.get(s.as_str()).unwrap());
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
                            // Do Nothing
                        }
                        // Otherwise add the schwa also to the output
                        _ => {
                            output.push_str(destination.vowels.main[0].as_str());
                        }
                    }
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

    // Remove the viramas as they aren't used in roman scripts
    output.replace("×", "")
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
    // Conversion is a bit more complicated here. Some roman scripts have more
    // than one character that are mapped to one indic character.
    // So we need to check in order if the next 3/2/1 characters have a mapping
    // and once a mapping is found skip the next 2/1 character if it is 3/2
    let mut output: String = String::new();
    let len = input.chars().count();
    let chars: Vec<char> = input.chars().collect();
    let mut skip: bool = false;
    let mut skip_twice: bool = false;
    let mut was_char: bool = false;
    for i in 0..len {
        if skip {
            skip = false;
            continue;
        }
        if skip_twice {
            skip_twice = false;
            continue;
        }
        let s = &mut String::new();
        let foo = &mut String::new();
        let foo1 = &mut String::new();
        s.push(chars[i]);
        // If a next two characters exist
        if i < len - 2 {
            foo1.push(chars[i]);
            foo1.push(chars[i + 1]);
            foo1.push(chars[i + 2]);
            if identify_type(
                foo1,
                &hash_map_consonants_main,
                &hash_map_vowels_main,
                &hash_map_vowelsigns_main,
                &hash_map_vowelsigns_virama,
                &hash_map_numerals,
                &hash_map_others_aytham,
                &hash_map_combiningsigns_ayogavaha,
                &hash_map_others_symbols,
            ) != CharType::CouldNotIdentify
            {
                skip = true;
                skip_twice = true;
                s.push(chars[i + 1]);
                s.push(chars[i + 2]);
            }
        }
        // Check for next 1 char now
        if !skip_twice && i < len - 1 {
            foo.push(chars[i]);
            foo.push(chars[i + 1]);
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
            ) != CharType::CouldNotIdentify
            {
                skip = true;
                s.push(chars[i + 1]);
            }
        }
        let t = identify_type(
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
        match t {
            CharType::ConsonantsMain => {
                
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                // Push the corresponding output consonant
                output.push_str(hash_map_consonants_main.get(s.as_str()).unwrap());
                was_char = true;
            }
            CharType::VowelSignsMain | CharType::VowelsMain => {
                
                // Check if the previous character in input was a consonant
                // If so push a vowelsigns.main
                // else push vowels.main

                // Edge case:
                // If input is 'a' and was_char is true, we need not push
                // anything because 'a' is present implicitly at end of every
                // consonant unless mentioned otherwise
                match was_char {
                    true => {
                        match s.as_str() {
                            _ if s == source.vowels.main[0].as_str() => {
                                // Do Nothing
                            }
                            _ => {
                                output.push_str(
                                    hash_map_vowelsigns_main.get(s.as_str()).unwrap_or_else(|| {
                                        hash_map_vowels_main.get(s.as_str()).unwrap()
                                    }),
                                );
                            }
                        }
                    }
                    false => {
                        output.push_str(
                            hash_map_vowels_main.get(s.as_str()).unwrap_or_else(|| {
                                hash_map_vowelsigns_main.get(s.as_str()).unwrap()
                            }),
                        );
                    }
                }
                was_char = false;
            }
            CharType::VowelSignsVirama => {
                output.push_str(hash_map_vowelsigns_virama.get(s.as_str()).unwrap());
                was_char = false;
            }
            CharType::OthersSymbols => {
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                output.push_str(hash_map_others_symbols.get(s.as_str()).unwrap());
                was_char = false;
            }
            CharType::OthersAytham => {
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                output.push_str(hash_map_others_aytham.get(s.as_str()).unwrap());
                was_char = false;
            }
            CharType::CombiningSignsAyogavaha => {
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                output.push_str(hash_map_combiningsigns_ayogavaha.get(s.as_str()).unwrap());
                was_char = false;
            }
            CharType::Space => {
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                output.push_str(" ");
                was_char = false;
            }
            CharType::NewLine => {
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                output.push_str("\n");
                was_char = false;
            }
            CharType::Numerals => {
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                output.push_str(hash_map_numerals.get(s.as_str()).unwrap());
                was_char = false;
            }
            _ => {
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                was_char = false;
            }
        };
    }

    output
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
    // Conversion is same as convert_roman_to_indic except we don't add viramas
    // as roman scripts don't have viramas
    let mut output: String = String::new();
    let len = input.chars().count();
    let chars: Vec<char> = input.chars().collect();
    let mut skip: bool = false;
    let mut skip_twice: bool = false;
    for i in 0..len {
        if skip {
            skip = false;
            continue;
        }
        if skip_twice {
            skip_twice = false;
            continue;
        }
        let s = &mut String::new();
        let foo = &mut String::new();
        let foo1 = &mut String::new();
        s.push(chars[i]);
        // If a next two characters map
        if i < len - 2 {
            foo1.push(chars[i]);
            foo1.push(chars[i + 1]);
            foo1.push(chars[i + 2]);
            if identify_type(
                foo1,
                &hash_map_consonants_main,
                &hash_map_vowels_main,
                &hash_map_vowelsigns_main,
                &hash_map_vowelsigns_virama,
                &hash_map_numerals,
                &hash_map_others_aytham,
                &hash_map_combiningsigns_ayogavaha,
                &hash_map_others_symbols,
            ) != CharType::CouldNotIdentify
            {
                skip = true;
                skip_twice = true;
                s.push(chars[i + 1]);
                s.push(chars[i + 2]);
            }
        }
        // Check for next 1 char
        if !skip_twice && i < len - 1 {
            foo.push(chars[i]);
            foo.push(chars[i + 1]);
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
            ) != CharType::CouldNotIdentify
            {
                skip = true;
                s.push(chars[i + 1]);
                
            }
        }
        let t = identify_type(
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
        match t {
            CharType::ConsonantsMain => {
                // Push the corresponding output consonant
                output.push_str(hash_map_consonants_main.get(s.as_str()).unwrap());
            }
            CharType::VowelSignsMain | CharType::VowelsMain => {
                output.push_str(hash_map_vowels_main.get(s.as_str()).unwrap());
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
            _ => {
            }
        };
    }

    output
}