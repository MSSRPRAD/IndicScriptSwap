// use transliterate_ferris::convert::convert_indic_to_indic;
use std::time::{Duration, Instant};
use transliterate_ferris::convert::convert_indic_to_intermediate;
use transliterate_ferris::convert::convert_intermediate_to_indic;
use transliterate_ferris::data::HASH_MAP;
use transliterate_ferris::read_mappings::Script;

fn main() {
    let mut start = Instant::now();

    let foo = &HASH_MAP;

    let telugu: &Script = foo.get("telugu").unwrap();
    // let devanagari: &Script = foo.get("devanagari").unwrap();
    // println!("{:?}", SCRIPT_INTERMEDIATE);

    let input = "అస్త్య్ ఉత్తరస్యాం దిశి దేవతాత్మా హిమాలయో నామ నగాధిరాజః ।
    పూర్వాపరౌ తోయనిధీ విగాహ్య స్థితః పృథివ్యా ఇవ మానదణ్డః ॥
    యం సర్వశైలాః పరికల్ప్య వత్సం మేరౌ స్థితే దోగ్ధరి దోహదక్షే ।
    భాస్వన్తి రత్నాని మహౌషధీశ్ చ పృథూపదిష్టాం దుదుహుర్ ధరిత్రీమ్ ॥";

    let initialization = start.elapsed();
    println!("initialization: {:?}", initialization);
    start = Instant::now();
    // println!("{:?}", convert_indic_to_indic(input, telugu, devanagari));
    let intermediate = convert_indic_to_intermediate(input, telugu);
    // println!("{:?}", intermediate);

    let conversion: Duration = start.elapsed();

    println!("conversion: {:?}", conversion);

    let printing = Instant::now();
    println!("{:?}", intermediate);
    let printing_time = printing.elapsed();

    println!("printing: {:?}", printing_time);

    let converted = convert_intermediate_to_indic(intermediate, telugu);
    println!("final: {}", converted);
}
