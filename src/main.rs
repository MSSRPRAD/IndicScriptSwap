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
    let _telugu = foo.data.scripts.get("telugu").unwrap();
    let slp1 = foo.data.scripts.get("slp1").unwrap();

    let input: String = "asty uttarasyAM diSi devatAtmA himAlayo nAma nagADirAjaH . pUrvAparO toyaniDI vigAhya sTitaH pfTivyA iva mAnadaRqaH .."
        .to_string();

    let converted = convert::convert_roman_to_indic(&input, slp1, devanagari);

    println!("{}\n\n{}", input, converted);

    let input: String = "अस्त्य् उत्तरस्यां दिशि देवतात्मा हिमालयो नाम नगाधिराजः । पूर्वापरौ तोयनिधी विगाह्य स्थितः पृथिव्या इव मानदण्डः ॥"
    .to_string();

    let converted = convert::convert_indic_to_roman(&input, devanagari, slp1);

    println!("\n\n{}\n\n{}\n\n", input, converted);

    // let input: String = "asty uttarasyAM diSi devatAtmA himAlayo nAma nagADirAjaH . pUrvAparO toyaniDI vigAhya sTitaH pfTivyA iva mAnadaRqaH .."
    //     .to_string();

    // let converted = convert::convert_roman_to_indic(&input, slp1, telugu);

    // println!("{}\n\n{}", input, converted);

    // let input: String = "అస్త్య్ ఉత్తరస్యాం దిశి దేవతాత్మా హిమాలయో నామ నగాధిరాజః ।
    // పూర్వాపరౌ తోయనిధీ విగాహ్య స్థితః పృథివ్యా ఇవ మానదణ్డః ॥"
    // .to_string();

    // let converted = convert::convert_indic_to_roman(&input, telugu, slp1);

    // println!("{}\n\n{}", input, converted);
}
