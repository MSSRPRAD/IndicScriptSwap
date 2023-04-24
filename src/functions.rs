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

pub fn identify_type(c: &str, data: &Script) -> CharType {
    if data.consonants.main.contains(&c.to_string().to_string()) {
        return CharType::ConsonantsMain;
    } else if data
        .combiningsigns
        .ayogavaha
        .contains(&c.to_string().to_string())
    {
        return CharType::CombiningSignsAyogavaha;
    } else if data.vowels.main.contains(&c.to_string().to_string()) {
        return CharType::VowelsMain;
    } else if data.vowelsigns.main.contains(&c.to_string().to_string()) {
        return CharType::VowelSignsMain;
    } else if data.vowelsigns.virama.contains(&c.to_string().to_string()) {
        return CharType::VowelSignsVirama;
    } else if data.numerals.contains(&c.to_string().to_string()) {
        return CharType::Numerals;
    } else if data.others.symbols.contains(&c.to_string().to_string()) {
        return CharType::OthersSymbols;
    } else if data.others.aytham.contains(&c.to_string().to_string()) {
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
