use crate::read_mappings::Script;
use std::collections::HashMap;

pub fn identify_type(c: &str, data: &Script) -> (String, usize) {
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
    } else if c == " " {
        return ("space".to_string(), 999);
    } else if c == "\n" {
        return ("new-line".to_string(), 999);
    } else {
        return (String::from("could.not.identify"), 999);
    }
}

pub fn make_hash_map<'a>(
    source: &'a Script,
    destination: &'a Script,
    t: usize,
) -> HashMap<&'a str, &'a str> {
    let mut hash_map: HashMap<&str, &str> = HashMap::new();
    // let mut v = Vec::new();
    match t {
        // 0 => {
        //     hash_map =  source.consonants.main.iter().zip(destination.consonants.main.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect();
        //     hash_map.extend(source.vowels.main.iter().zip(destination.vowels.main.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect());
        //     hash_map.extend(source.vowels.main.iter().zip(destination.vowels.main.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect());
        //     hash_map.extend(source.vowels.main.iter().zip(destination.vowels.main.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect());
        //     hash_map.extend(source.vowels.main.iter().zip(destination.vowels.main.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect());
        //     hash_map.extend(source.vowels.main.iter().zip(destination.vowels.main.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect());
        //     hash_map.extend(source.vowels.main.iter().zip(destination.vowels.main.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect());
        //     hash_map.extend(source.vowels.main.iter().zip(destination.vowels.main.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect());
        // }
        1 => {
            // v = vec![(&source.consonants.main, &destination.consonants.main)];
            hash_map =  source.consonants.main.iter().zip(destination.consonants.main.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect();
        }
        2 => {
            hash_map =  source.vowels.main.iter().zip(destination.vowels.main.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect();
        }
        3 => {
            hash_map =  source.vowelsigns.main.iter().zip(destination.vowelsigns.main.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect();
        }
        4 => {
            hash_map =  source.vowelsigns.virama.iter().zip(destination.vowelsigns.virama.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect();
        }
        5 => {
            hash_map =  source.numerals.iter().zip(destination.numerals.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect();
        }
        6 => {
            hash_map =  source.others.aytham.iter().zip(destination.others.aytham.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect();
        }
        7 => {
            hash_map =  source.combiningsigns.ayogavaha.iter().zip(destination.combiningsigns.ayogavaha.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect();
        }
        8 => {
            hash_map =  source.others.symbols.iter().zip(destination.others.symbols.iter()).map(|(s, d)| (s.as_str(), d.as_str())).collect();
        }
        _ => {}
    }
    // for (s, d) in v {
    //     hash_map.extend(
    //         s.iter()
    //             .zip(d.iter())
    //             .map(|(k, v)| (k.as_str(), v.as_str())),
    //     );
    // }

    return hash_map;
}
