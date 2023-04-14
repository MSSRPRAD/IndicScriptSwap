use crate::read_mappings::ScriptData;
use read_mappings::read_mappings;
mod convert;
mod functions;
mod read_mappings;

fn main() {
    let foo = ScriptData {
        data: read_mappings(),
    };

    let devanagari = foo.data.scripts.get("devanagari").unwrap();
    let slp1 = foo.data.scripts.get("slp1").unwrap();

    let input: String = "अस्त्य् उत्तरस्यां दिशि देवतात्मा हिमालयो नाम नगाधिराजः ।
    पूर्वापरौ तोयनिधी विगाह्य स्थितः पृथिव्या इव मानदण्डः ॥"
        .to_string();

    let converted = convert::convert_indic_to_roman(&input, devanagari, slp1);

    println!("\n\n{:?}\n\n{:?}\n\n", input, converted);
}
