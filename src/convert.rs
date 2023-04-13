use crate::read_mappings::Script;
use crate::functions::identify_type;

pub fn convert_indic_to_roman(input: String, script: &Script) -> String {
    for i in 0..input.len(){
        let c: char;
        if let Some(_val) = input.chars().nth(i) {
            c = input.chars().nth(i).unwrap();
            println!("{:?},{:?}",c,identify_type(c, script));
        }
    }
    "Some Error Occured!".to_string()
}