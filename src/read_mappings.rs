// #[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    pub combiningsigns: CombiningSign,
    pub consonants: Consonant,
    pub numerals: Vec<String>,
    pub others: Other,
    pub vowels: Vowel,
    pub vowelsigns: VowelSign,
}

// #[derive(Debug, Serialize, Deserialize)]
pub struct VowelSign {
    pub main: Vec<String>,
    pub modern: Vec<String>,
    pub sinhala: Vec<String>,
    pub south: Vec<String>,
    pub virama: Vec<String>,
}

// #[derive(Debug, Serialize, Deserialize)]
pub struct Vowel {
    pub main: Vec<String>,
    pub modern: Vec<String>,
    pub sinhala: Vec<String>,
    pub south: Vec<String>,
}

// #[derive(Debug, Serialize, Deserialize)]
pub struct Other {
    pub aytham: Vec<String>,
    pub om: Vec<String>,
    pub symbols: Vec<String>,
}

// #[derive(Debug, Serialize, Deserialize)]
pub struct CombiningSign {
    pub ayogavaha: Vec<String>,
    pub nukta: Vec<String>,
}

// // #[derive(Debug, Serialize, Deserialize)]
pub struct Consonant {
    pub main: Vec<String>,
    pub persoarabic: Vec<String>,
    pub sinhala: Vec<String>,
    pub south: Vec<String>,
}
