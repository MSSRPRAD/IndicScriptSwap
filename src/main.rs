use transliterate_ferris::data::HASH_MAP;
use transliterate_ferris::data::SCRIPT_INTERMEDIATE;
use transliterate_ferris::functions::make_hash_map_from_intermediate;

fn main() {
    let foo = &HASH_MAP;

    let telugu = foo.get("telugu").unwrap();

    println!("{:?}", SCRIPT_INTERMEDIATE);

    let my_tuple = make_hash_map_from_intermediate(telugu, &SCRIPT_INTERMEDIATE);
    // Print individual elements using tuple indexing
    println!("Element 0: {:?}", my_tuple.0);
    println!("Element 1: {:?}", my_tuple.1);
    println!("Element 2: {:?}", my_tuple.2);
    println!("Element 3: {:?}", my_tuple.3);
    println!("Element 4: {:?}", my_tuple.4);
    println!("Element 5: {:?}", my_tuple.5);
    println!("Element 6: {:?}", my_tuple.6);
    println!("Element 7: {:?}", my_tuple.7);
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
