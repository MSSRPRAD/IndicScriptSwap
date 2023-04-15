use crate::read_mappings::Script;
use std::collections::HashMap;

pub fn identify_type(c: char, data: &Script) -> (String, usize) {
    if data.consonants.main.contains(&c.to_string().to_string()) {
        return (
            "consonants.main".to_string(),
            data.consonants
                .main
                .iter()
                .position(|x| x.eq(&c.clone().to_string()))
                .unwrap(),
        );
    } else if data
        .combiningsigns
        .ayogavaha
        .contains(&c.to_string().to_string())
    {
        return (
            "combiningsigns.ayogavaha".to_string(),
            data.combiningsigns
                .ayogavaha
                .iter()
                .position(|x| x.eq(&c.clone().to_string()))
                .unwrap(),
        );
    } else if data.vowels.main.contains(&c.to_string().to_string()) {
        return (
            "vowels.main".to_string(),
            data.vowels
                .main
                .iter()
                .position(|x| x.eq(&c.clone().to_string()))
                .unwrap(),
        );
    } else if data.vowelsigns.main.contains(&c.to_string().to_string()) {
        return (
            "vowelsigns.main".to_string(),
            data.vowelsigns
                .main
                .iter()
                .position(|x| x.eq(&c.clone().to_string()))
                .unwrap(),
        );
    } else if data.vowelsigns.virama.contains(&c.to_string().to_string()) {
        return (
            "vowelsigns.virama".to_string(),
            data.vowelsigns
                .virama
                .iter()
                .position(|x| x.eq(&c.clone().to_string()))
                .unwrap(),
        );
    } else if data.numerals.contains(&c.to_string().to_string()) {
        return (
            "numerals".to_string(),
            data.numerals
                .iter()
                .position(|x| x.eq(&c.clone().to_string()))
                .unwrap(),
        );
    } else if data.others.symbols.contains(&c.to_string().to_string()) {
        return (
            "others.symbols".to_string(),
            data.others
                .symbols
                .iter()
                .position(|x| x.eq(&c.clone().to_string()))
                .unwrap(),
        );
    } else if data.others.aytham.contains(&c.to_string().to_string()) {
        return (
            "others.aytham".to_string(),
            data.others
                .aytham
                .iter()
                .position(|x| x.eq(&c.clone().to_string()))
                .unwrap(),
        );
    } else if c == ' ' {
        return ("space".to_string(), 999);
    } else if c == '\n' {
        return ("new-line".to_string(), 999);
    } else {
        return (String::from("Could not identify"), 999);
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
