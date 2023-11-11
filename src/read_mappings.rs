use crate::tokens::{
    Ayogavaha, Aytham, ConsonantsMain, Nukta, Numerals, Om, PersoArabic, Sinhala, South, Symbols,
    VowelMain, VowelModern, VowelSignMain, VowelSignModern, VowelSignSinhala, VowelSignSouth,
    VowelSignVirama, VowelSinhala, VowelSouth,
};

#[derive(Debug)]
pub struct Script {
    pub combiningsigns: CombiningSign,
    pub consonants: Consonant,
    pub numerals: Vec<String>,
    pub others: Other,
    pub vowels: Vowel,
    pub vowelsigns: VowelSign,
}

#[derive(Debug)]
pub struct VowelSign {
    pub main: Vec<String>,
    pub modern: Vec<String>,
    pub sinhala: Vec<String>,
    pub south: Vec<String>,
    pub virama: Vec<String>,
}

#[derive(Debug)]
pub struct Vowel {
    pub main: Vec<String>,
    pub modern: Vec<String>,
    pub sinhala: Vec<String>,
    pub south: Vec<String>,
}

#[derive(Debug)]
pub struct Other {
    pub aytham: Vec<String>,
    pub om: Vec<String>,
    pub symbols: Vec<String>,
}

#[derive(Debug)]
pub struct CombiningSign {
    pub ayogavaha: Vec<String>,
    pub nukta: Vec<String>,
}

#[derive(Debug)]
pub struct Consonant {
    pub main: Vec<String>,
    pub persoarabic: Vec<String>,
    pub sinhala: Vec<String>,
    pub south: Vec<String>,
}

// Intermediate structs from here on

#[derive(Debug)]
pub struct ConsonantIntermediate {
    pub main: Vec<ConsonantsMain>,
    pub persoarabic: Vec<PersoArabic>,
    pub sinhala: Vec<Sinhala>,
    pub south: Vec<South>,
}

#[derive(Debug)]
pub struct CombiningSignIntermediate {
    pub ayogavaha: Vec<Ayogavaha>,
    pub nukta: Vec<Nukta>,
}

#[derive(Debug)]
pub struct OtherIntermediate {
    pub aytham: Vec<Aytham>,
    pub om: Vec<Om>,
    pub symbols: Vec<Symbols>,
}

#[derive(Debug)]
pub struct VowelIntermediate {
    pub main: Vec<VowelMain>,
    pub modern: Vec<VowelModern>,
    pub sinhala: Vec<VowelSinhala>,
    pub south: Vec<VowelSouth>,
}

#[derive(Debug)]
pub struct VowelSignIntermediate {
    pub main: Vec<VowelSignMain>,
    pub modern: Vec<VowelSignModern>,
    pub sinhala: Vec<VowelSignSinhala>,
    pub south: Vec<VowelSignSouth>,
    pub virama: Vec<VowelSignVirama>,
}

#[derive(Debug)]
pub struct ScriptIntermediate {
    pub combiningsigns: CombiningSignIntermediate,
    pub consonants: ConsonantIntermediate,
    pub numerals: Vec<Numerals>,
    pub others: OtherIntermediate,
    pub vowels: VowelIntermediate,
    pub vowelsigns: VowelSignIntermediate,
}
