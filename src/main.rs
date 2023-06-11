// use transliterate_ferris::convert::convert_indic_to_indic;
use transliterate_ferris::convert::convert_indic_to_intermediate;
use transliterate_ferris::data::HASH_MAP;
use transliterate_ferris::data::SCRIPT_INTERMEDIATE;
use transliterate_ferris::read_mappings::Script;

fn main() {
    let foo = &HASH_MAP;

    let telugu: &Script = foo.get("telugu").unwrap();
    // let devanagari: &Script = foo.get("devanagari").unwrap();

    println!("{:?}", SCRIPT_INTERMEDIATE);

    let input = "తపఃస్వాధ్యాయనిరతం తపస్వీ వాగ్విదాం వరమ్ ।
    నారదం పరిపప్రచ్ఛ వాల్మీకిర్మునిపుంగవమ్ ॥ ౧ ॥
    కో న్వస్మిన్సామ్ప్రతం లోకే గుణవాన్కశ్చ వీర్యవాన్ ।
    ధర్మజ్ఞశ్చ కృతజ్ఞశ్చ సత్యవాక్యో దృఢవ్రతః ॥ ౨ ॥";

    let intermediate = convert_indic_to_intermediate(input, telugu);
    println!("{:?}", intermediate);

    // println!("{:?}", convert_indic_to_indic(input, telugu, devanagari));
}
