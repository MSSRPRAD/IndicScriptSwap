use crate::read_mappings::Script;
use crate::functions::identify_type;

pub fn convert_indic_to_roman(input: String, script: &Script) -> String {
    for i in 0..input.len(){
        let mut c: char = 'x';
        if let Some(val) = input.chars().nth(i) {
            c = input.chars().nth(i).unwrap();
            println!("\n\n{:?},{:?}",c,identify_type(c, script));
        }
    }
    println!("\n\nDONE\n\n");
    "Some Error Occured!".to_string()
}