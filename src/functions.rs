use crate::read_mappings::Script;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum CharType {
    ConsonantsMain,
    CombiningSignsAyogavaha,
    VowelsMain,
    VowelSignsMain,
    VowelSignsVirama,
    Numerals,
    OthersSymbols,
    OthersAytham,
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
