// use crate::read_mappings::read_mappings;

// use crate::read_mappings::ScriptData;
mod convert;
mod data;
mod functions;
mod read_mappings;

fn main() {
    let foo = &data::HASH_MAP;

    let destination = foo.get("telugu").unwrap();
    let source = foo.get("slp1").unwrap();

    let input = "asty uttarasyAM diSi devatAtmA himAlayo nAma nagADirAjaH .\n    pUrvAparO toyaniDI vigAhya sTitaH pfTivyA iva mAnadaRqaH ..".to_string();
    let converted = convert::convert_roman_to_indic(&input, source, destination);
    println!("{}", converted);
}
