use crate::{
    read_mappings::{Script, ScriptIntermediate},
    tokens::{
        Ayogavaha, Aytham, ConsonantsMain, Numerals, Symbols, VowelMain, VowelSignMain,
        VowelSignVirama,
    },
};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum CharType {
    ConsonantsMain,
    VowelsMain,
    VowelSignsMain,
    VowelSignsVirama,
    Numerals,
    OthersAytham,
    CombiningSignsAyogavaha,
    OthersSymbols,
    Space,
    NewLine,
    CouldNotIdentify,
}

pub fn identify_type(
    c: &str,
    hash_map_consonants_main: &HashMap<&str, &str>,
    hash_map_vowels_main: &HashMap<&str, &str>,
    hash_map_vowelsigns_main: &HashMap<&str, &str>,
    hash_map_vowelsigns_virama: &HashMap<&str, &str>,
    hash_map_numerals: &HashMap<&str, &str>,
    hash_map_others_aytham: &HashMap<&str, &str>,
    hash_map_combining_signs_ayogavaha: &HashMap<&str, &str>,
    hash_map_others_symbols: &HashMap<&str, &str>,
) -> CharType {
    if hash_map_consonants_main.contains_key(&*c.to_string()) {
        return CharType::ConsonantsMain;
    } else if hash_map_combining_signs_ayogavaha.contains_key(&*c.to_string()) {
        return CharType::CombiningSignsAyogavaha;
    } else if hash_map_vowels_main.contains_key(&*c.to_string()) {
        return CharType::VowelsMain;
    } else if hash_map_vowelsigns_main.contains_key(&*c.to_string()) {
        return CharType::VowelSignsMain;
    } else if hash_map_vowelsigns_virama.contains_key(&*c.to_string()) {
        return CharType::VowelSignsVirama;
    } else if hash_map_numerals.contains_key(&*c.to_string()) {
        return CharType::Numerals;
    } else if hash_map_others_symbols.contains_key(&*c.to_string()) {
        return CharType::OthersSymbols;
    } else if hash_map_others_aytham.contains_key(&*c.to_string()) {
        return CharType::OthersAytham;
    } else if c == " " {
        return CharType::Space;
    } else if c == "\n" {
        return CharType::NewLine;
    } else {
        return CharType::CouldNotIdentify;
    }
}

pub fn identify_type_intermediate(
    c: &str,
    hash_map_consonants_main: &HashMap<&str, ConsonantsMain>,
    hash_map_vowels_main: &HashMap<&str, VowelMain>,
    hash_map_vowelsigns_main: &HashMap<&str, VowelSignMain>,
    hash_map_vowelsigns_virama: &HashMap<&str, VowelSignVirama>,
    hash_map_numerals: &HashMap<&str, Numerals>,
    hash_map_others_aytham: &HashMap<&str, Aytham>,
    hash_map_combining_signs_ayogavaha: &HashMap<&str, Ayogavaha>,
    hash_map_others_symbols: &HashMap<&str, Symbols>,
) -> CharType {
    if hash_map_consonants_main.contains_key(&*c.to_string()) {
        return CharType::ConsonantsMain;
    } else if hash_map_combining_signs_ayogavaha.contains_key(&*c.to_string()) {
        return CharType::CombiningSignsAyogavaha;
    } else if hash_map_vowels_main.contains_key(&*c.to_string()) {
        return CharType::VowelsMain;
    } else if hash_map_vowelsigns_main.contains_key(&*c.to_string()) {
        return CharType::VowelSignsMain;
    } else if hash_map_vowelsigns_virama.contains_key(&*c.to_string()) {
        return CharType::VowelSignsVirama;
    } else if hash_map_numerals.contains_key(&*c.to_string()) {
        return CharType::Numerals;
    } else if hash_map_others_symbols.contains_key(&*c.to_string()) {
        return CharType::OthersSymbols;
    } else if hash_map_others_aytham.contains_key(&*c.to_string()) {
        return CharType::OthersAytham;
    } else if c == " " {
        return CharType::Space;
    } else if c == "\n" {
        return CharType::NewLine;
    } else {
        return CharType::CouldNotIdentify;
    }
}

pub fn make_hash_map<'a>(
    source: &'a Script,
    destination: &'a Script,
    t: usize,
) -> HashMap<&'a str, &'a str> {
    let mut hash_map: HashMap<&str, &str> = HashMap::new();
    let mut v = Vec::new();
    match t {
        0 => {
            v = vec![
                (&source.consonants.main, &destination.consonants.main),
                (&source.vowels.main, &destination.vowels.main),
                (&source.vowelsigns.main, &destination.vowelsigns.main),
                (&source.vowelsigns.virama, &destination.vowelsigns.virama),
                (&source.numerals, &destination.numerals),
                (&source.others.aytham, &destination.others.aytham),
                (&source.others.symbols, &destination.others.symbols),
                (
                    &source.combiningsigns.ayogavaha,
                    &destination.combiningsigns.ayogavaha,
                ),
            ];
        }
        1 => {
            v = vec![(&source.consonants.main, &destination.consonants.main)];
        }
        2 => {
            v = vec![(&source.vowels.main, &destination.vowels.main)];
        }
        3 => {
            v = vec![(&source.vowelsigns.main, &destination.vowelsigns.main)];
        }
        4 => {
            v = vec![(&source.vowelsigns.virama, &destination.vowelsigns.virama)];
        }
        5 => {
            v = vec![(&source.numerals, &destination.numerals)];
        }
        6 => {
            v = vec![(&source.others.aytham, &destination.others.aytham)];
        }
        7 => {
            v = vec![(
                &source.combiningsigns.ayogavaha,
                &destination.combiningsigns.ayogavaha,
            )];
        }
        8 => {
            v = vec![(&source.others.symbols, &destination.others.symbols)];
        }
        _ => {}
    }
    for (s, d) in v {
        hash_map.extend(
            s.iter()
                .zip(d.iter())
                .map(|(k, v)| (k.as_str(), v.as_str())),
        );
    }

    return hash_map;
}

pub fn make_hash_map_from_intermediate_to_script<'a>(
    script: &'a Script,
    script_intermediate: &'a ScriptIntermediate,
) -> (
    HashMap<ConsonantsMain,&'a str>,
    HashMap<VowelMain,&'a str>,
    HashMap<VowelSignMain,&'a str>,
    HashMap<VowelSignVirama,&'a str>,
    HashMap<Numerals,&'a str>,
    HashMap<Aytham,&'a str>,
    HashMap<Ayogavaha,&'a str>,
    HashMap<Symbols,&'a str>,
) {
    let mut consonants_main_mapping: HashMap<ConsonantsMain,&'a str> = HashMap::new();

    for (sound, intermediate_consonant) in script_intermediate
        .consonants
        .main
        .iter()
        .zip(script.consonants.main.iter())
    {
        consonants_main_mapping.insert(*sound, intermediate_consonant);
    }

    let mut vowels_main_mapping: HashMap<VowelMain, &'a str> = HashMap::new();

    for (sound, intermediate_vowel_main) in script_intermediate
        .vowels
        .main
        .iter()
        .zip(script.vowels.main.iter())
    {
        vowels_main_mapping.insert(*sound, intermediate_vowel_main);
    }

    let mut vowelsigns_main_mapping: HashMap<VowelSignMain,&'a str> = HashMap::new();

    for (sound, intermediate_vowelsigns_main) in script_intermediate
        .vowelsigns
        .main
        .iter()
        .zip(script.vowelsigns.main.iter())
    {
        vowelsigns_main_mapping.insert(*sound, intermediate_vowelsigns_main);
    }

    let mut vowelsigns_virama_mapping: HashMap<VowelSignVirama, &'a str> = HashMap::new();

    for (sound, intermediate_vowelsigns_virama) in script_intermediate
        .vowelsigns
        .virama
        .iter()
        .zip(script.vowelsigns.virama.iter())
    {
        vowelsigns_virama_mapping.insert(*sound, intermediate_vowelsigns_virama);
    }

    let mut numerals_mapping: HashMap<Numerals, &'a str> = HashMap::new();

    for (sound, numeral) in script_intermediate
        .numerals
        .iter()
        .zip(script.numerals.iter())
    {
        numerals_mapping.insert(*sound, numeral);
    }

    let mut combiningsigns_ayogavaha_mapping: HashMap<Ayogavaha, &'a str> = HashMap::new();

    for (sound, ayogavaha) in script_intermediate
        .combiningsigns
        .ayogavaha
        .iter()
        .zip(script.combiningsigns.ayogavaha.iter())
    {
        combiningsigns_ayogavaha_mapping.insert(*sound, ayogavaha);
    }

    let mut aytham_mapping: HashMap<Aytham, &'a str> = HashMap::new();

    for (sound, aytham) in script_intermediate
        .others
        .aytham
        .iter()
        .zip(script.others.aytham.iter())
    {
        aytham_mapping.insert(*sound, aytham);
    }

    let mut symbols_mapping: HashMap<Symbols, &'a str> = HashMap::new();

    for (sound, symbol) in script_intermediate
        .others
        .symbols
        .iter()
        .zip(script.others.symbols.iter())
    {
        symbols_mapping.insert(*sound, symbol);
    }

    let mut vowelsigns_virama_mapping: HashMap<VowelSignVirama, &'a str> = HashMap::new();

    for (sound, vowelsigns_virama) in script_intermediate
        .vowelsigns
        .virama
        .iter()
        .zip(script.vowelsigns.virama.iter())
    {
        vowelsigns_virama_mapping.insert(*sound, vowelsigns_virama);
    }

    return (
        consonants_main_mapping,
        vowels_main_mapping,
        vowelsigns_main_mapping,
        vowelsigns_virama_mapping,
        numerals_mapping,
        aytham_mapping,
        combiningsigns_ayogavaha_mapping,
        symbols_mapping,
    );
}


pub fn make_hash_map_from_script_to_intermediate<'a>(
    script: &'a Script,
    script_intermediate: &'a ScriptIntermediate,
) -> (
    HashMap<&'a str, ConsonantsMain>,
    HashMap<&'a str, VowelMain>,
    HashMap<&'a str, VowelSignMain>,
    HashMap<&'a str, VowelSignVirama>,
    HashMap<&'a str, Numerals>,
    HashMap<&'a str, Aytham>,
    HashMap<&'a str, Ayogavaha>,
    HashMap<&'a str, Symbols>,
) {
    let mut consonants_main_mapping: HashMap<&'a str, ConsonantsMain> = HashMap::new();

    for (sound, intermediate_consonant) in script
        .consonants
        .main
        .iter()
        .zip(script_intermediate.consonants.main.iter())
    {
        consonants_main_mapping.insert(sound.as_str(), intermediate_consonant.clone());
    }

    let mut vowels_main_mapping: HashMap<&'a str, VowelMain> = HashMap::new();

    for (sound, intermediate_vowel_main) in script
        .vowels
        .main
        .iter()
        .zip(script_intermediate.vowels.main.iter())
    {
        vowels_main_mapping.insert(sound.as_str(), intermediate_vowel_main.clone());
    }

    let mut vowelsigns_main_mapping: HashMap<&'a str, VowelSignMain> = HashMap::new();

    for (sound, intermediate_vowelsigns_main) in script
        .vowelsigns
        .main
        .iter()
        .zip(script_intermediate.vowelsigns.main.iter())
    {
        vowelsigns_main_mapping.insert(sound.as_str(), intermediate_vowelsigns_main.clone());
    }

    let mut vowelsigns_virama_mapping: HashMap<&'a str, VowelSignVirama> = HashMap::new();

    for (sound, intermediate_vowelsigns_virama) in script
        .vowelsigns
        .virama
        .iter()
        .zip(script_intermediate.vowelsigns.virama.iter())
    {
        vowelsigns_virama_mapping.insert(sound.as_str(), intermediate_vowelsigns_virama.clone());
    }

    let mut numerals_mapping: HashMap<&'a str, Numerals> = HashMap::new();

    for (sound, numeral) in script
        .numerals
        .iter()
        .zip(script_intermediate.numerals.iter())
    {
        numerals_mapping.insert(sound.as_str(), numeral.clone());
    }

    let mut combiningsigns_ayogavaha_mapping: HashMap<&'a str, Ayogavaha> = HashMap::new();

    for (sound, ayogavaha) in script
        .combiningsigns
        .ayogavaha
        .iter()
        .zip(script_intermediate.combiningsigns.ayogavaha.iter())
    {
        combiningsigns_ayogavaha_mapping.insert(sound.as_str(), ayogavaha.clone());
    }

    let mut aytham_mapping: HashMap<&'a str, Aytham> = HashMap::new();

    for (sound, aytham) in script
        .others
        .aytham
        .iter()
        .zip(script_intermediate.others.aytham.iter())
    {
        aytham_mapping.insert(sound.as_str(), aytham.clone());
    }

    let mut symbols_mapping: HashMap<&'a str, Symbols> = HashMap::new();

    for (sound, symbol) in script
        .others
        .symbols
        .iter()
        .zip(script_intermediate.others.symbols.iter())
    {
        symbols_mapping.insert(sound.as_str(), symbol.clone());
    }

    let mut vowelsigns_virama_mapping: HashMap<&'a str, VowelSignVirama> = HashMap::new();

    for (sound, vowelsigns_virama) in script
        .vowelsigns
        .virama
        .iter()
        .zip(script_intermediate.vowelsigns.virama.iter())
    {
        vowelsigns_virama_mapping.insert(sound.as_str(), vowelsigns_virama.clone());
    }

    return (
        consonants_main_mapping,
        vowels_main_mapping,
        vowelsigns_main_mapping,
        vowelsigns_virama_mapping,
        numerals_mapping,
        aytham_mapping,
        combiningsigns_ayogavaha_mapping,
        symbols_mapping,
    );
}
