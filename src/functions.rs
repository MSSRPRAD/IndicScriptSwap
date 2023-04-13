use crate::read_mappings::Script;

pub fn identify_type(c: char, data: &Script) -> String {
    if data.consonants.main.contains(&c.to_string()) {return "consonants.main".to_string()}
    else if data.vowels.main.contains(&c.to_string()) {return "consonants.main".to_string()}
    else if data.vowelsigns.main.contains(&c.to_string()) {return "consonants.main".to_string()}
    else if data.vowelsigns.virama.contains(&c.to_string()) {return "consonants.virama".to_string()}
    else if data.numerals.contains(&c.to_string()) {return "consonants.numerals".to_string()}
    else if data.others.symbols.contains(&c.to_string()) {return "data.others.symbols".to_string()}
    else if data.others.aytham.contains(&c.to_string()) {return "data.others.aytham".to_string()}
    else if c==' ' {return "space".to_string()}
    else if c=='\n' {return "new-line".to_string()}
    else {return String::from("Could not identify")}
}