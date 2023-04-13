use crate::read_mappings::Script;

pub fn identify_type(c: char, data: &Script) -> (String, usize) {
    if data.consonants.main.contains(&c.to_string().to_string()) {return ("consonants.main".to_string(), data.consonants.main.iter().position(|x| x.eq(&c.clone().to_string())).unwrap())}
    else if data.vowels.main.contains(&c.to_string().to_string()) {return ("vowels.main".to_string(), data.vowels.main.iter().position(|x| x.eq(&c.clone().to_string())).unwrap())}
    else if data.vowelsigns.main.contains(&c.to_string().to_string()) {return ("vowelsigns.main".to_string(), data.vowelsigns.main.iter().position(|x| x.eq(&c.clone().to_string())).unwrap())}
    else if data.vowelsigns.virama.contains(&c.to_string().to_string()) {return ("vowelsigns.virama".to_string(), data.vowelsigns.virama.iter().position(|x| x.eq(&c.clone().to_string())).unwrap())}
    else if data.numerals.contains(&c.to_string().to_string()) {return ("numerals".to_string(), data.numerals.iter().position(|x| x.eq(&c.clone().to_string())).unwrap())}
    else if data.others.symbols.contains(&c.to_string().to_string()) {return ("data.others.symbols".to_string(), data.others.symbols.iter().position(|x| x.eq(&c.clone().to_string())).unwrap())}
    else if data.others.aytham.contains(&c.to_string().to_string()) {return ("data.others.aytham".to_string(), data.others.aytham.iter().position(|x| x.eq(&c.clone().to_string())).unwrap())}
    else if c==' ' {return ("space".to_string(), 999)}
    else if c=='\n' {return ("new-line".to_string(), 999)}
    else {return (String::from("Could not identify"), 999)}
}