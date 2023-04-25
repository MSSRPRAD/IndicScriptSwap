use transliterate_ferris::convert::{
    convert_indic_to_indic, convert_indic_to_roman, convert_roman_to_indic, convert_roman_to_roman,
};
use transliterate_ferris::data::HASH_MAP;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

const def_input: String = String::new();
const def_output: String = String::new();

#[derive(Parser, Debug)]
#[clap(author="Pradyumna Malladi", version = "0.5.0", about="This is a tool to help transliterate between various indic scripts. 
It is not ready yet and has many issues.
If you encounter any issues, please contact me (https://github.com/mssrprad/transliterator-ferris or pradyumna.malladi2003@gmail.com)")]
struct Args {
    
    /// Source Script
    #[arg(short, long)]
    source: String,

    /// Destination Script
    #[arg(short, long)]
    destination: String,

    /// Input File (With no special characters that shouldn't be transliterated)
    #[arg(short, long, default_value_t = def_input)]
    input: String,

    /// Output File
    #[arg(short, long, default_value_t = def_output)]
    output: String,

    /// Type of Conversion
    #[arg(short, long)]
    conversion: String,

}



fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut file = match File::open(args.input){
        Ok(file) => file, 
        Err(error) => return Err(error),
    };
    let mut contents = String::new();
    let source = HASH_MAP.get(&args.source as &str).unwrap();
    let destination = HASH_MAP.get(&args.destination as &str).unwrap();
    file.read_to_string(&mut contents)?;
    let converted: String;
    match args.conversion.as_str() {
        "i2i" => {converted=convert_indic_to_indic(&contents, source, destination);},
        "i2r" => {converted=convert_indic_to_roman(&contents, source, destination);},
        "r2i" => {converted=convert_roman_to_indic(&contents, source, destination);},
        "r2r" => {converted=convert_roman_to_roman(&contents, source, destination);},
        _ => {converted=String::from("Could Not Convert!")},
    }
    if args.output.len()!=0 {
        file = File::create(args.output)?;
        file.write_all(converted.as_bytes());
        Ok(())
    } else {
        println!("{}", converted);
        Ok(())
    }
    
}