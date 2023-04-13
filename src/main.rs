use read_mappings::read_mappings;
use crate::read_mappings::ScriptData;
mod functions;
mod read_mappings;

fn main() {
    let foo =  ScriptData {
        data: read_mappings(),
    };

    let devanagari = foo.data.scripts.get("devanagari").unwrap();
    let slp1 = foo.data.scripts.get("slp1").unwrap();

    let input: String = "अस्त्य् उत्तरस्यां दिशि देवतात्मा हिमालयो नाम नगाधिराजः ।
    पूर्वापरौ तोयनिधी विगाह्य स्थितः पृथिव्या इव मानदण्डः ॥".to_string();

    for i in input.chars(){
        // println!("{:?},{:?}", i, devanagari.consonants.main.contains(&i.to_string()) );
        println!("{:?},{:?}", i, functions::identify_type(i, devanagari));
    }


    println!("\n\n{:?}", devanagari);
    println!("\n\n{:?}", slp1);
}
