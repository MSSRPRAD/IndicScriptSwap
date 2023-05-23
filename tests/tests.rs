#[cfg(test)]
mod tests {
    use transliterate_ferris::convert::{convert_roman_to_indic, convert_indic_to_indic, convert_indic_to_roman};
    use transliterate_ferris::data::HASH_MAP;
    #[test]
    fn test_indic_to_indic() {
        let foo = &HASH_MAP;
        let input = "తపఃస్వాధ్యాయనిరతం తపస్వీ వాగ్విదాం వరమ్ ।
                నారదం పరిపప్రచ్ఛ వాల్మీకిర్మునిపుంగవమ్ ॥ ౧ ॥
                కో న్వస్మిన్సామ్ప్రతం లోకే గుణవాన్కశ్చ వీర్యవాన్ ।
                ధర్మజ్ఞశ్చ కృతజ్ఞశ్చ సత్యవాక్యో దృఢవ్రతః ॥ ౨ ॥
                చారిత్రేణ చ కో యుక్తః సర్వభూతేషు కో హితః ।
                విద్వాన్కః కః సమర్థశ్చ కశ్చైకప్రియదర్శనః ॥ ౩ ॥
                ఆత్మవాన్కో జితక్రోధో ద్యుతిమాన్కోఽనసూయకః ।
                కస్య బిభ్యతి దేవాశ్చ జాతరోషస్య సంయుగే ॥ ౪ ॥
                ఏతదిచ్ఛామ్యహం శ్రోతుం పరం కౌతూహలం హి మే ।
                మహర్షే త్వం సమర్థోఽసి జ్ఞాతుమేవంవిధం నరమ్ ॥ ౫ ॥";
        let expected = "तपःस्वाध्यायनिरतं तपस्वी वाग्विदां वरम् ।
                नारदं परिपप्रच्छ वाल्मीकिर्मुनिपुंगवम् ॥ १ ॥
                को न्वस्मिन्साम्प्रतं लोके गुणवान्कश्च वीर्यवान् ।
                धर्मज्ञश्च कृतज्ञश्च सत्यवाक्यो दृढव्रतः ॥ २ ॥
                चारित्रेण च को युक्तः सर्वभूतेषु को हितः ।
                विद्वान्कः कः समर्थश्च कश्चैकप्रियदर्शनः ॥ ३ ॥
                आत्मवान्को जितक्रोधो द्युतिमान्कोऽनसूयकः ।
                कस्य बिभ्यति देवाश्च जातरोषस्य संयुगे ॥ ४ ॥
                एतदिच्छाम्यहं श्रोतुं परं कौतूहलं हि मे ।
                महर्षे त्वं समर्थोऽसि ज्ञातुमेवंविधं नरम् ॥ ५ ॥";
        let telugu = foo.get("telugu").unwrap();
        let devanagari = foo.get("devanagari").unwrap();
        let converted = convert_indic_to_indic(input, telugu, devanagari);
        assert_eq!(expected, converted);
    }
    #[test]
    fn test_indic_to_roman() {
        let foo = &HASH_MAP;
        let input = "తపఃస్వాధ్యాయనిరతం తపస్వీ వాగ్విదాం వరమ్ ।
                నారదం పరిపప్రచ్ఛ వాల్మీకిర్మునిపుంగవమ్ ॥ ౧ ॥
                కో న్వస్మిన్సామ్ప్రతం లోకే గుణవాన్కశ్చ వీర్యవాన్ ।
                ధర్మజ్ఞశ్చ కృతజ్ఞశ్చ సత్యవాక్యో దృఢవ్రతః ॥ ౨ ॥
                చారిత్రేణ చ కో యుక్తః సర్వభూతేషు కో హితః ।
                విద్వాన్కః కః సమర్థశ్చ కశ్చైకప్రియదర్శనః ॥ ౩ ॥
                ఆత్మవాన్కో జితక్రోధో ద్యుతిమాన్కోఽనసూయకః ।
                కస్య బిభ్యతి దేవాశ్చ జాతరోషస్య సంయుగే ॥ ౪ ॥
                ఏతదిచ్ఛామ్యహం శ్రోతుం పరం కౌతూహలం హి మే ।
                మహర్షే త్వం సమర్థోఽసి జ్ఞాతుమేవంవిధం నరమ్ ॥ ౫ ॥";
        let expected = "tapaHsvADyAyanirataM tapasvI vAgvidAM varam .
                nAradaM paripapracCa vAlmIkirmunipuMgavam .. 1 ..
                ko nvasminsAmprataM loke guRavAnkaSca vIryavAn .
                DarmajYaSca kftajYaSca satyavAkyo dfQavrataH .. 2 ..
                cAritreRa ca ko yuktaH sarvaBUtezu ko hitaH .
                vidvAnkaH kaH samarTaSca kaScEkapriyadarSanaH .. 3 ..
                AtmavAnko jitakroDo dyutimAnko'nasUyakaH .
                kasya biByati devASca jAtarozasya saMyuge .. 4 ..
                etadicCAmyahaM SrotuM paraM kOtUhalaM hi me .
                maharze tvaM samarTo'si jYAtumevaMviDaM naram .. 5 ..";
        let telugu = foo.get("telugu").unwrap();
        let slp1 = foo.get("slp1").unwrap();
        let converted = convert_indic_to_roman(input, telugu, slp1);
        assert_eq!(expected, converted);
    }
    // For scripts where only one character needs to be mapped
    // at one time
    #[test]
    fn test_roman_to_indic_1() {
        let foo = &HASH_MAP;
        let input = "tapaHsvADyAyanirataM tapasvI vAgvidAM varam .
        nAradaM paripapracCa vAlmIkirmunipuMgavam .. 1 ..
        ko nvasminsAmprataM loke guRavAnkaSca vIryavAn .
        DarmajYaSca kftajYaSca satyavAkyo dfQavrataH .. 2 ..
        cAritreRa ca ko yuktaH sarvaBUtezu ko hitaH .
        vidvAnkaH kaH samarTaSca kaScEkapriyadarSanaH .. 3 ..
        AtmavAnko jitakroDo dyutimAnko'nasUyakaH .
        kasya biByati devASca jAtarozasya saMyuge .. 4 ..
        etadicCAmyahaM SrotuM paraM kOtUhalaM hi me .
        maharze tvaM samarTo'si jYAtumevaMviDaM naram .. 5 ..";
        let expected = "తపఃస్వాధ్యాయనిరతం తపస్వీ వాగ్విదాం వరమ్ ।
        నారదం పరిపప్రచ్ఛ వాల్మీకిర్మునిపుంగవమ్ ॥ ౧ ॥
        కో న్వస్మిన్సామ్ప్రతం లోకే గుణవాన్కశ్చ వీర్యవాన్ ।
        ధర్మజ్ఞశ్చ కృతజ్ఞశ్చ సత్యవాక్యో దృఢవ్రతః ॥ ౨ ॥
        చారిత్రేణ చ కో యుక్తః సర్వభూతేషు కో హితః ।
        విద్వాన్కః కః సమర్థశ్చ కశ్చైకప్రియదర్శనః ॥ ౩ ॥
        ఆత్మవాన్కో జితక్రోధో ద్యుతిమాన్కోఽనసూయకః ।
        కస్య బిభ్యతి దేవాశ్చ జాతరోషస్య సంయుగే ॥ ౪ ॥
        ఏతదిచ్ఛామ్యహం శ్రోతుం పరం కౌతూహలం హి మే ।
        మహర్షే త్వం సమర్థోఽసి జ్ఞాతుమేవంవిధం నరమ్ ॥ ౫ ॥";
        let telugu = foo.get("telugu").unwrap();
        let slp1 = foo.get("slp1").unwrap();
        let converted = convert_roman_to_indic(input, slp1, telugu);
        assert_eq!(expected, converted);
    }
}
