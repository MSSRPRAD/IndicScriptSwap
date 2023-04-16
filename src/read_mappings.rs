// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;


// #[derive(Debug, Serialize, Deserialize)]
// pub struct Scripts {
//     pub scripts: HashMap<String, Script>,
// }

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

// pub struct ScriptData {
//     pub data: Scripts,
// }

// pub fn read_mappings() -> Scripts {
//     let path = "./src/data/script_mapping.json";
//     let data = fs::read_to_string(path).expect("Unable to read file");
//     let my_data: Scripts = serde_json::from_str(&data).expect("Unable to parse");
//     return my_data;
// }

// pub fn read_mappings_native() -> Scripts {
//     return Scripts {
//         scripts: data::HASH_MAP,
//     };
// }
