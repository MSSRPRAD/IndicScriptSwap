use crate::data::SCRIPT_INTERMEDIATE;
use crate::functions::{
    identify_type, identify_type_intermediate, make_hash_map,
    make_hash_map_from_intermediate_to_script, make_hash_map_from_script_to_intermediate, CharType, identify_type_intermediate_new,
};
use crate::read_mappings::{Other, Script};
use crate::tokens::{Akshara, Others};
use crate::tokens::{
    Ayogavaha, Aytham, ConsonantsMain, Nukta, Numerals, Om, PersoArabic, Sinhala, South, Symbols,
    VowelMain, VowelModern, VowelSignMain, VowelSignModern, VowelSignSinhala, VowelSignSouth,
    VowelSignVirama, VowelSinhala, VowelSouth,
};

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
            _ => {}
        };
    }

    output
}

// Intermediate Conversion functions

pub fn convert_intermediate_to_indic(
    input: Vec<
        Akshara<
            String,
            Ayogavaha,
            Aytham,
            ConsonantsMain,
            Nukta,
            Numerals,
            Om,
            PersoArabic,
            Sinhala,
            South,
            Symbols,
            VowelMain,
            VowelModern,
            VowelSignMain,
            VowelSignModern,
            VowelSignSinhala,
            VowelSignSouth,
            VowelSignVirama,
            VowelSinhala,
            VowelSouth,
            Others,
        >,
    >,
    destination: &Script,
) -> String {
    // // Make a hashmap from source characters to corresponding destination ones
    // // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.

    let my_tuple = make_hash_map_from_intermediate_to_script(destination, &SCRIPT_INTERMEDIATE);

    let hash_map_consonants_main = my_tuple.0;
    let hash_map_vowels_main = my_tuple.1;
    let hash_map_vowelsigns_main = my_tuple.2;
    let hash_map_vowelsigns_virama = my_tuple.3;
    let hash_map_numerals = my_tuple.4;
    let hash_map_others_aytham = my_tuple.5;
    let hash_map_combiningsigns_ayogavaha = my_tuple.6;
    let hash_map_others_symbols = my_tuple.7;

    // Conversion is easy here because our intermediate is input and only one token needs to be checked at once.
    let mut output: String = String::new();
    let len = input.len();
    let mut was_char: bool = false;
    for i in 0..len {
        let s = input[i].clone();

        match s {
            Akshara::ConsonantsMain(consonant) => {
                // If previous token was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                // Push the corresponding output consonant
                output.push_str(hash_map_consonants_main.get(&consonant).unwrap());
                was_char = true;
            }
            Akshara::VowelSignMain(_) | Akshara::VowelMain(_) => {
                // Check if the previous character in input was a consonant
                // If so push a vowelsigns.main
                // else push vowels.main

                // Edge case:
                // If input is 'a' and was_char is true, we need not push
                // anything because 'a' is present implicitly at end of every
                // consonant unless mentioned otherwise

                match was_char {
                    true => {
                        match s {
                            _ if s == Akshara::VowelMain(VowelMain::a) => {
                                // Do Nothing
                            }
                            _ => {
                                // First push the vowelsign if it is one
                                // If not a vowelsign push the vowel
                                match s {
                                    Akshara::VowelSignMain(vowelsign) => {
                                        output.push_str(
                                            hash_map_vowelsigns_main.get(&vowelsign).unwrap(),
                                        );
                                    }
                                    Akshara::VowelMain(vowel) => {
                                        output.push_str(hash_map_vowels_main.get(&vowel).unwrap());
                                    }
                                    _ => {
                                        // Do Nothing
                                    }
                                }
                            }
                        }
                    }
                    false => {
                        // First push the vowel if it is one
                        // If not a vowel push the vowelsign
                        match s {
                            Akshara::VowelMain(vowel) => {
                                output.push_str(hash_map_vowels_main.get(&vowel).unwrap());
                            }
                            Akshara::VowelSignMain(vowelsign) => {
                                output.push_str(hash_map_vowelsigns_main.get(&vowelsign).unwrap());
                            }
                            _ => {
                                // Do Nothing
                            }
                        }
                    }
                }
                was_char = false;
            }
            Akshara::VowelSignVirama(vowelsignvirama) => {
                output.push_str(hash_map_vowelsigns_virama.get(&vowelsignvirama).unwrap());
                was_char = false;
            }
            Akshara::Symbols(symbol) => {
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                output.push_str(hash_map_others_symbols.get(&symbol).unwrap());
                was_char = false;
            }
            Akshara::Aytham(aytham) => {
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                output.push_str(hash_map_others_aytham.get(&aytham).unwrap());
                was_char = false;
            }
            Akshara::Ayogavaha(ayogavaha) => {
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                output.push_str(hash_map_combiningsigns_ayogavaha.get(&ayogavaha).unwrap());
                was_char = false;
            }
            Akshara::Others(Others::Space) => {
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
            Akshara::Others(Others::NewLine) => {
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
            Akshara::Numerals(numeral) => {
                // If previous char was a consonant we push a virama before
                match was_char {
                    true => {
                        output.push_str(destination.vowelsigns.virama[0].as_str());
                    }
                    false => {
                        // Do Nothing
                    }
                }
                output.push_str(hash_map_numerals.get(&numeral).unwrap());
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

pub fn convert_intermediate_to_roman(
    input: Vec<
        Akshara<
            String,
            Ayogavaha,
            Aytham,
            ConsonantsMain,
            Nukta,
            Numerals,
            Om,
            PersoArabic,
            Sinhala,
            South,
            Symbols,
            VowelMain,
            VowelModern,
            VowelSignMain,
            VowelSignModern,
            VowelSignSinhala,
            VowelSignSouth,
            VowelSignVirama,
            VowelSinhala,
            VowelSouth,
            Others,
        >,
    >,
    destination: &Script,
) -> String {
    // // Make a hashmap from source characters to corresponding destination ones
    // // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.

    let my_tuple = make_hash_map_from_intermediate_to_script(destination, &SCRIPT_INTERMEDIATE);

    let hash_map_consonants_main = my_tuple.0;
    let hash_map_vowels_main = my_tuple.1;
    let hash_map_vowelsigns_main = my_tuple.2;
    let hash_map_vowelsigns_virama = my_tuple.3;
    let hash_map_numerals = my_tuple.4;
    let hash_map_others_aytham = my_tuple.5;
    let hash_map_combiningsigns_ayogavaha = my_tuple.6;
    let hash_map_others_symbols = my_tuple.7;

    // Conversion is same as convert_roman_to_indic except we don't add viramas
    // as roman scripts don't have viramas
    let mut output: String = String::new();
    let len = input.len();
    for i in 0..len {
        let s = input[i].clone();
        match s {
            Akshara::ConsonantsMain(consonant) => {
                // Push the corresponding output consonant
                output.push_str(hash_map_consonants_main.get(&consonant).unwrap());
            }
            Akshara::VowelSignMain(vowelsign) => {
                output.push_str(hash_map_vowelsigns_main.get(&vowelsign).unwrap());
            }
            Akshara::VowelMain(vowel) => {
                output.push_str(hash_map_vowels_main.get(&vowel).unwrap());
            }
            Akshara::VowelSignVirama(virama) => {
                output.push_str(hash_map_vowelsigns_virama.get(&virama).unwrap());
            }
            Akshara::Symbols(symbol) => {
                output.push_str(hash_map_others_symbols.get(&symbol).unwrap());
            }
            Akshara::Aytham(aytham) => {
                output.push_str(hash_map_others_aytham.get(&aytham).unwrap());
            }
            Akshara::Ayogavaha(ayogavaha) => {
                output.push_str(hash_map_combiningsigns_ayogavaha.get(&ayogavaha).unwrap());
            }
            Akshara::Others(Others::Space) => {
                output.push_str(" ");
            }
            Akshara::Others(Others::NewLine) => {
                output.push_str("\n");
            }
            Akshara::Numerals(numeral) => {
                output.push_str(hash_map_numerals.get(&numeral).unwrap());
            }
            _ => {}
        };
    }

    output
}

pub fn convert_indic_to_intermediate(
    input: &str,
    source: &Script,
) -> Vec<
    Akshara<
        String,
        Ayogavaha,
        Aytham,
        ConsonantsMain,
        Nukta,
        Numerals,
        Om,
        PersoArabic,
        Sinhala,
        South,
        Symbols,
        VowelMain,
        VowelModern,
        VowelSignMain,
        VowelSignModern,
        VowelSignSinhala,
        VowelSignSouth,
        VowelSignVirama,
        VowelSinhala,
        VowelSouth,
        Others,
    >,
> {
    // Make a hashmap from source characters to corresponding destination ones
    // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.

    let my_tuple = make_hash_map_from_script_to_intermediate(source, &SCRIPT_INTERMEDIATE);

    // Make the hash maps
    let hash_map_consonants_main = my_tuple.0;
    let hash_map_vowels_main = my_tuple.1;
    let hash_map_vowelsigns_main = my_tuple.2;
    let hash_map_vowelsigns_virama = my_tuple.3;
    let hash_map_numerals = my_tuple.4;
    let hash_map_others_aytham = my_tuple.5;
    let hash_map_combiningsigns_ayogavaha = my_tuple.6;
    let hash_map_others_symbols = my_tuple.7;

    // Conversion is a bit more complicated here. This is because there is often a hidden 'a' after
    // most consonants. We need to check this case by seeing the next character after a consonant.

    // Create the vector of output
    let mut output: Vec<
        Akshara<
            String,
            Ayogavaha,
            Aytham,
            ConsonantsMain,
            Nukta,
            Numerals,
            Om,
            PersoArabic,
            Sinhala,
            South,
            Symbols,
            VowelMain,
            VowelModern,
            VowelSignMain,
            VowelSignModern,
            VowelSignSinhala,
            VowelSignSouth,
            VowelSignVirama,
            VowelSinhala,
            VowelSouth,
            Others,
        >,
    > = Vec::new();

    let len = input.chars().count();
    let chars: Vec<char> = input.chars().collect();
    for i in 0..len {
        let s = &mut String::new();
        s.push(chars[i]);
        let t = identify_type_intermediate(
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
                output.push(Akshara::ConsonantsMain(
                    *hash_map_consonants_main.get(s.as_str()).unwrap(),
                ));
                // If there is a next character
                if i < len - 1 {
                    match identify_type_intermediate(
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
                            output.push(Akshara::VowelMain(VowelMain::a));
                        }
                    }
                }
            }
            CharType::VowelsMain => {
                output.push(Akshara::VowelMain(
                    *hash_map_vowels_main.get(s.as_str()).unwrap(),
                ));
            }
            CharType::VowelSignsMain => {
                output.push(Akshara::VowelSignMain(
                    *hash_map_vowelsigns_main.get(s.as_str()).unwrap(),
                ));
            }
            CharType::VowelSignsVirama => {
                // Do Nothing as roman intermediate does not have virama

                // output.push(Akshara::VowelSignVirama(
                //     *hash_map_vowelsigns_virama.get(s.as_str()).unwrap(),
                // ));
            }
            CharType::OthersSymbols => {
                output.push(Akshara::Symbols(
                    *hash_map_others_symbols.get(s.as_str()).unwrap(),
                ));
            }
            CharType::OthersAytham => {
                output.push(Akshara::Aytham(
                    *hash_map_others_aytham.get(s.as_str()).unwrap(),
                ));
            }
            CharType::CombiningSignsAyogavaha => {
                output.push(Akshara::Ayogavaha(
                    *hash_map_combiningsigns_ayogavaha.get(s.as_str()).unwrap(),
                ));
            }
            CharType::Space => {
                output.push(Akshara::Others(Others::Space));
            }
            CharType::NewLine => {
                output.push(Akshara::Others(Others::NewLine));
            }
            CharType::Numerals => {
                output.push(Akshara::Numerals(
                    *hash_map_numerals.get(s.as_str()).unwrap(),
                ));
            }
            CharType::CouldNotIdentify => {
                output.push(Akshara::Unknown(s.to_string()));
            }
        };
    }

    return output;
}

pub fn convert_roman_to_intermediate(
    input: &str,
    source: &Script,
) -> Vec<
    Akshara<
        String,
        Ayogavaha,
        Aytham,
        ConsonantsMain,
        Nukta,
        Numerals,
        Om,
        PersoArabic,
        Sinhala,
        South,
        Symbols,
        VowelMain,
        VowelModern,
        VowelSignMain,
        VowelSignModern,
        VowelSignSinhala,
        VowelSignSouth,
        VowelSignVirama,
        VowelSinhala,
        VowelSouth,
        Others,
    >,
> {
    // Make a hashmap from source characters to corresponding destination ones
    // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.

    let my_tuple = make_hash_map_from_script_to_intermediate(source, &SCRIPT_INTERMEDIATE);

    // Make the hash maps
    let hash_map_consonants_main = my_tuple.0;
    let hash_map_vowels_main = my_tuple.1;
    let hash_map_vowelsigns_main = my_tuple.2;
    let hash_map_vowelsigns_virama = my_tuple.3;
    let hash_map_numerals = my_tuple.4;
    let hash_map_others_aytham = my_tuple.5;
    let hash_map_combiningsigns_ayogavaha = my_tuple.6;
    let hash_map_others_symbols = my_tuple.7;

    // Conversion is same as convert_roman_to_indic except we don't add viramas
    // as roman scripts don't have viramas
    // Create the vector of output
    let mut output: Vec<
        Akshara<
            String,
            Ayogavaha,
            Aytham,
            ConsonantsMain,
            Nukta,
            Numerals,
            Om,
            PersoArabic,
            Sinhala,
            South,
            Symbols,
            VowelMain,
            VowelModern,
            VowelSignMain,
            VowelSignModern,
            VowelSignSinhala,
            VowelSignSouth,
            VowelSignVirama,
            VowelSinhala,
            VowelSouth,
            Others,
        >,
    > = Vec::new();
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
            if identify_type_intermediate(
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
            if identify_type_intermediate(
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
        let t = identify_type_intermediate(
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
                output.push(Akshara::ConsonantsMain(
                    *hash_map_consonants_main.get(s.as_str()).unwrap(),
                ));
            }
            CharType::VowelSignsMain | CharType::VowelsMain => {
                output.push(Akshara::VowelMain(
                    *hash_map_vowels_main.get(s.as_str()).unwrap(),
                ));
            }
            CharType::VowelSignsVirama => {
                output.push(Akshara::VowelSignVirama(
                    *hash_map_vowelsigns_virama.get(s.as_str()).unwrap(),
                ));
            }
            CharType::OthersSymbols => {
                output.push(Akshara::Symbols(
                    *hash_map_others_symbols.get(s.as_str()).unwrap(),
                ));
            }
            CharType::OthersAytham => {
                output.push(Akshara::Aytham(
                    *hash_map_others_aytham.get(s.as_str()).unwrap(),
                ));
            }
            CharType::CombiningSignsAyogavaha => {
                output.push(Akshara::Ayogavaha(
                    *hash_map_combiningsigns_ayogavaha.get(s.as_str()).unwrap(),
                ));
            }
            CharType::Space => {
                output.push(Akshara::Others(Others::Space));
            }
            CharType::NewLine => {
                output.push(Akshara::Others(Others::NewLine));
            }
            CharType::Numerals => {
                output.push(Akshara::Numerals(
                    *hash_map_numerals.get(s.as_str()).unwrap(),
                ));
            }
            _ => {}
        };
    }

    output
}


pub fn convert_roman_to_intermediate_new(
    input: &str,
    source: &Script,
) -> Vec<
    Akshara<
        String,
        Ayogavaha,
        Aytham,
        ConsonantsMain,
        Nukta,
        Numerals,
        Om,
        PersoArabic,
        Sinhala,
        South,
        Symbols,
        VowelMain,
        VowelModern,
        VowelSignMain,
        VowelSignModern,
        VowelSignSinhala,
        VowelSignSouth,
        VowelSignVirama,
        VowelSinhala,
        VowelSouth,
        Others,
    >,
> {
    // Make a hashmap from source characters to corresponding destination ones
    // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.

    let my_tuple = make_hash_map_from_script_to_intermediate(source, &SCRIPT_INTERMEDIATE);

    // Make the hash maps
    let hash_map_consonants_main = my_tuple.0;
    let hash_map_vowels_main = my_tuple.1;
    let hash_map_vowelsigns_main = my_tuple.2;
    let hash_map_vowelsigns_virama = my_tuple.3;
    let hash_map_numerals = my_tuple.4;
    let hash_map_others_aytham = my_tuple.5;
    let hash_map_combiningsigns_ayogavaha = my_tuple.6;
    let hash_map_others_symbols = my_tuple.7;

    // Conversion is same as convert_roman_to_indic except we don't add viramas
    // as roman scripts don't have viramas
    // Create the vector of output
    let mut output: Vec<
        Akshara<
            String,
            Ayogavaha,
            Aytham,
            ConsonantsMain,
            Nukta,
            Numerals,
            Om,
            PersoArabic,
            Sinhala,
            South,
            Symbols,
            VowelMain,
            VowelModern,
            VowelSignMain,
            VowelSignModern,
            VowelSignSinhala,
            VowelSignSouth,
            VowelSignVirama,
            VowelSinhala,
            VowelSouth,
            Others,
        >,
    > = Vec::new();
    let len = input.chars().count();
    let chars: Vec<char> = input.chars().collect();
    let mut skip: bool = false;
    let mut skip_twice: bool = false;
    let mut t = Akshara::Unknown("".to_string());
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
            t = identify_type_intermediate_new(
                foo1,
                &hash_map_consonants_main,
                &hash_map_vowels_main,
                &hash_map_vowelsigns_main,
                &hash_map_vowelsigns_virama,
                &hash_map_numerals,
                &hash_map_others_aytham,
                &hash_map_combiningsigns_ayogavaha,
                &hash_map_others_symbols,);
            if let Akshara::Unknown(_) = t {} else
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
            t = identify_type_intermediate_new(
                foo,
                &hash_map_consonants_main,
                &hash_map_vowels_main,
                &hash_map_vowelsigns_main,
                &hash_map_vowelsigns_virama,
                &hash_map_numerals,
                &hash_map_others_aytham,
                &hash_map_combiningsigns_ayogavaha,
                &hash_map_others_symbols,
            );
            if let Akshara::Unknown(_) = t {} else
            {
                skip = true;
                s.push(chars[i + 1]);
            }
        }
        if !skip && !skip_twice {
            t = identify_type_intermediate_new(
                s,
                &hash_map_consonants_main,
                &hash_map_vowels_main,
                &hash_map_vowelsigns_main,
                &hash_map_vowelsigns_virama,
                &hash_map_numerals,
                &hash_map_others_aytham,
                &hash_map_combiningsigns_ayogavaha,
                &hash_map_others_symbols,);
        }
        match t {
            Akshara::ConsonantsMain(consonant) => {
                // Push the corresponding output consonant
                output.push(Akshara::ConsonantsMain(
                    consonant,
                ));
            }
            Akshara::VowelMain(vowel) => {
                output.push(Akshara::VowelMain(
                    vowel,
                ));
            }
            Akshara::VowelSignMain(vowelsign) => {
                output.push(Akshara::VowelSignMain(
                    vowelsign,
                ));
            }
            Akshara::VowelSignVirama(virama) => {
                output.push(Akshara::VowelSignVirama(
                    virama,
                ));
            }
            Akshara::Symbols(symbol) => {
                output.push(Akshara::Symbols(
                    symbol,
                ));
            }
            Akshara::Aytham(aytham) => {
                output.push(Akshara::Aytham(
                    aytham,
                ));
            }
            Akshara::Ayogavaha(ayogavaha) => {
                output.push(Akshara::Ayogavaha(
                    ayogavaha,
                ));
            }
            Akshara::Others(Others::Space) => {
                output.push(Akshara::Others(Others::Space));
            }
            Akshara::Others(Others::NewLine) => {
                output.push(Akshara::Others(Others::NewLine));
            }
            Akshara::Numerals(numeral) => {
                output.push(Akshara::Numerals(
                    numeral,
                ));
            }
            _ => {}
        };
    }

    output
}


pub fn convert_indic_to_intermediate_new(
    input: &str,
    source: &Script,
) -> Vec<
    Akshara<
        String,
        Ayogavaha,
        Aytham,
        ConsonantsMain,
        Nukta,
        Numerals,
        Om,
        PersoArabic,
        Sinhala,
        South,
        Symbols,
        VowelMain,
        VowelModern,
        VowelSignMain,
        VowelSignModern,
        VowelSignSinhala,
        VowelSignSouth,
        VowelSignVirama,
        VowelSinhala,
        VowelSouth,
        Others,
    >,
> {
    // Make a hashmap from source characters to corresponding destination ones
    // Since all we need now is the consonants, numerals, vowels, vowelsigns, others, we will make only for them for now.

    let my_tuple = make_hash_map_from_script_to_intermediate(source, &SCRIPT_INTERMEDIATE);

    // Make the hash maps
    let hash_map_consonants_main = my_tuple.0;
    let hash_map_vowels_main = my_tuple.1;
    let hash_map_vowelsigns_main = my_tuple.2;
    let hash_map_vowelsigns_virama = my_tuple.3;
    let hash_map_numerals = my_tuple.4;
    let hash_map_others_aytham = my_tuple.5;
    let hash_map_combiningsigns_ayogavaha = my_tuple.6;
    let hash_map_others_symbols = my_tuple.7;

    // Conversion is a bit more complicated here. This is because there is often a hidden 'a' after
    // most consonants. We need to check this case by seeing the next character after a consonant.

    // Create the vector of output
    let mut output: Vec<
        Akshara<
            String,
            Ayogavaha,
            Aytham,
            ConsonantsMain,
            Nukta,
            Numerals,
            Om,
            PersoArabic,
            Sinhala,
            South,
            Symbols,
            VowelMain,
            VowelModern,
            VowelSignMain,
            VowelSignModern,
            VowelSignSinhala,
            VowelSignSouth,
            VowelSignVirama,
            VowelSinhala,
            VowelSouth,
            Others,
        >,
    > = Vec::new();

    let len = input.chars().count();
    let chars: Vec<char> = input.chars().collect();
    for i in 0..len {
        let s = &mut String::new();
        s.push(chars[i]);
        let t = identify_type_intermediate_new(
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
        // println!("char: {:?}", s);
        // println!("token: {:?}", t);
        match t {
            Akshara::ConsonantsMain(consonant) => {
                // Push the corresponding destination consonant
                output.push(Akshara::ConsonantsMain(
                    consonant,
                ));
                // If there is a next character
                if i < len - 1 {
                    match identify_type_intermediate_new(
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
                        Akshara::VowelSignVirama(_)
                        | Akshara::Symbols(_)
                        | Akshara::Numerals(_)
                        | Akshara::VowelMain(_)
                        | Akshara::VowelSignMain(_) => {
                            // Do Nothing
                        }
                        // Otherwise add the schwa also to the output
                        _ => {
                            // println!("pushing \'a\'");
                            output.push(Akshara::VowelMain(VowelMain::a));
                        }
                    }
                }
            }
            Akshara::VowelMain(vowel) => {
                output.push(Akshara::VowelMain(
                    vowel,
                ));
            }
            Akshara::VowelSignMain(vowelsign) => {
                output.push(Akshara::VowelSignMain(
                    vowelsign,
                ));
            }
            Akshara::VowelSignVirama(virama) => {
                // Do Nothing as roman intermediate does not have virama

                // output.push(Akshara::VowelSignVirama(
                //     *hash_map_vowelsigns_virama.get(s.as_str()).unwrap(),
                // ));
            }
            Akshara::Symbols(symbol) => {
                output.push(Akshara::Symbols(
                    symbol,
                ));
            }
            Akshara::Aytham(aytham) => {
                output.push(Akshara::Aytham(
                    aytham,
                ));
            }
            Akshara::Ayogavaha(ayogavaha) => {
                output.push(Akshara::Ayogavaha(
                    ayogavaha,
                ));
            }
            Akshara::Others(Others::Space) => {
                output.push(Akshara::Others(Others::Space));
            }
            Akshara::Others(Others::NewLine) => {
                output.push(Akshara::Others(Others::NewLine));
            }
            Akshara::Numerals(numeral) => {
                output.push(Akshara::Numerals(
                    numeral,
                ));
            }
            _ => {
                // Do Nothing
            }
        };
    }

    return output;
}
