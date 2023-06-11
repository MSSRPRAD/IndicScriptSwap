// use transliterate_ferris::convert::{
//     convert_indic_to_indic, convert_indic_to_roman, convert_roman_to_indic, convert_roman_to_roman,
// };
use transliterate_ferris::data::HASH_MAP;
fn main() {
    let foo = &HASH_MAP;

    let slp1 = foo.get("slp1").unwrap();
    // let devanagari = foo.get("devanagari").unwrap();

    println!("{:?}", slp1);

    // let input = "తపఃస్వాధ్యాయనిరతం తపస్వీ వాగ్విదాం వరమ్ ।
    //             నారదం పరిపప్రచ్ఛ వాల్మీకిర్మునిపుంగవమ్ ॥ ౧ ॥
    //             కో న్వస్మిన్సామ్ప్రతం లోకే గుణవాన్కశ్చ వీర్యవాన్ ।
    //             ధర్మజ్ఞశ్చ కృతజ్ఞశ్చ సత్యవాక్యో దృఢవ్రతః ॥ ౨ ॥
    //             చారిత్రేణ చ కో యుక్తః సర్వభూతేషు కో హితః ।
    //             విద్వాన్కః కః సమర్థశ్చ కశ్చైకప్రియదర్శనః ॥ ౩ ॥
    //             ఆత్మవాన్కో జితక్రోధో ద్యుతిమాన్కోఽనసూయకః ।
    //             కస్య బిభ్యతి దేవాశ్చ జాతరోషస్య సంయుగే ॥ ౪ ॥
    //             ఏతదిచ్ఛామ్యహం శ్రోతుం పరం కౌతూహలం హి మే ।
    //             మహర్షే త్వం సమర్థోఽసి జ్ఞాతుమేవంవిధం నరమ్ ॥ ౫ ॥";
    // let converted = convert_indic_to_indic(input, telugu, devanagari);
    // println!("{}", converted);
}
