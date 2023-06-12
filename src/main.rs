// use transliterate_ferris::convert::convert_indic_to_indic;
use std::time::{Duration, Instant};
use transliterate_ferris::convert::convert_indic_to_indic;
use transliterate_ferris::convert::convert_indic_to_intermediate;
use transliterate_ferris::convert::convert_intermediate_to_indic;
use transliterate_ferris::convert::convert_intermediate_to_roman;
use transliterate_ferris::convert::convert_roman_to_intermediate;
use transliterate_ferris::data::HASH_MAP;
use transliterate_ferris::read_mappings::Script;

fn main() {
    let mut start = Instant::now();

    let foo = &HASH_MAP;

    let itrans: &Script = foo.get("itrans").unwrap();
    let iast: &Script = foo.get("itrans").unwrap();
    // let devanagari: &Script = foo.get("devanagari").unwrap();
    // println!("{:?}", SCRIPT_INTERMEDIATE);

    let input = "tapaHsvAdhyAyanirataM tapasvI vAgvidAM varam .
    nAradaM paripaprachCha vAlmIkirmunipuMgavam .. 1 ..";

    let intermediate = convert_roman_to_intermediate(input, itrans);
    // let initialization = start.elapsed();
    // println!("initialization: {:?}", initialization);
    // start = Instant::now();
    // // println!("{:?}", convert_indic_to_indic(input, telugu, devanagari));
    // let mut intermediate = Vec::new();
    // for _i in 0..100 {
    //     intermediate = convert_indic_to_intermediate(input, telugu);
    // }// println!("{:?}", intermediate);

    // let conversion: Duration = start.elapsed()/100;

    // println!("conversion to tokens: {:?}", conversion);

    // // let printing = Instant::now();
    // // println!("{:?}", intermediate);
    // // let printing_time = printing.elapsed();

    // // println!("printing intermediate: {:?}", printing_time);
    // let convertingfromintermediate = Instant::now();
    // for _i in 0..100 {
    //     let _ = convert_intermediate_to_indic(intermediate.clone(), telugu);
    // }
    // println!("convertingfromintermediate: {:?}", convertingfromintermediate.elapsed()/100);
    // // println!("final: {}", converted);
    // start = Instant::now();
    // for _i in 0..100 {
    //     let _ = convert_indic_to_indic(input, telugu, telugu);
    // }
    // println!("direct conversion: {:?}", start.elapsed()/100);

    // println!("{}", convert_intermediate_to_indic(intermediate.clone(), telugu));
    println!("Converting 2 intermediate: {:?}", start.elapsed());
    start = Instant::now();
    let converted = convert_intermediate_to_roman(intermediate, iast);
    println!("{:?}", converted);
    
}
