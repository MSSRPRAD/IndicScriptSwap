use crate::tokens::{
    Ayogavaha, Aytham, ConsonantsMain, Nukta, PersoArabic, Sinhala, South, Symbols, VowelMain,
    VowelModern, VowelSignMain, VowelSignModern, VowelSignSinhala, VowelSignSouth, VowelSignVirama,
    VowelSinhala, VowelSouth,
};
use crate::{
    read_mappings::*,
    tokens::{Numerals, Om},
};
use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;

lazy_static! {

#[derive(Debug)]
pub static ref HASH_MAP: HashMap<&'static str, Script> = hashmap!{

    "telugu" => Script { combiningsigns: CombiningSign { ayogavaha: ["\u{c00}", "ం", "ః"].iter().map(|&s| s.into()).collect(), nukta: ["·"].iter().map(|&s| s.into()).collect() }, consonants: Consonant { main: ["క", "ఖ", "గ", "ఘ", "ఙ", "చ", "ఛ", "జ", "ఝ", "ఞ", "ట", "ఠ", "డ", "ఢ", "ణ", "త", "థ", "ద", "ధ", "న", "ప", "ఫ", "బ", "భ", "మ", "య", "ర", "ల", "వ", "శ", "ష", "స", "హ"].iter().map(|&s| s.into()).collect(), persoarabic: ["క·", "ఖ·", "గ·", "జ·", "డ·", "ఢ·", "ఫ·", "య·"].iter().map(|&s| s.into()).collect(), sinhala: ["ఁˆగ", "ఁˆజ", "ఁˆడ", "ఁˆద", "ఁˆబ"].iter().map(|&s| s.into()).collect(), south: ["ళ", "ఴ", "ఱ", "న·"].iter().map(|&s| s.into()).collect() }, numerals: ["౦", "౧", "౨", "౩", "౪", "౫", "౬", "౭", "౮", "౯"].iter().map(|&s| s.into()).collect(), others: Other { aytham: ["ఃʼ"].iter().map(|&s| s.into()).collect(), om: ["ఓం"].iter().map(|&s| s.into()).collect(), symbols: ["ఽ", "।", "॥"].iter().map(|&s| s.into()).collect() }, vowels: Vowel { main: ["అ", "ఆ", "ఇ", "ఈ", "ఉ", "ఊ", "ఋ", "ౠ", "ఌ", "ౡ", "ఏ", "ఐ", "ఓ", "ఔ"].iter().map(|&s| s.into()).collect(), modern: ["ఎ\u{952}\u{200b}", "ఒ\u{952}\u{200b}"].iter().map(|&s| s.into()).collect(), sinhala: ["ఏ\u{952}\u{200b}"].iter().map(|&s| s.into()).collect(), south: ["ఎ", "ఒ"].iter().map(|&s| s.into()).collect() }, vowelsigns: VowelSign { main: ["\u{c3e}", "\u{c3f}", "\u{c40}", "ు", "ూ", "ృ", "ౄ", "\u{c62}", "\u{c63}", "\u{c47}", "\u{c48}", "\u{c4b}", "\u{c4c}"].iter().map(|&s| s.into()).collect(), modern: ["\u{c46}\u{952}\u{200b}", "\u{c4a}\u{952}\u{200b}"].iter().map(|&s| s.into()).collect(), sinhala: ["\u{c47}\u{952}\u{200b}"].iter().map(|&s| s.into()).collect(), south: ["\u{c46}", "\u{c4a}"].iter().map(|&s| s.into()).collect(), virama: ["\u{c4d}"].iter().map(|&s| s.into()).collect() } },

    "grantha" => Script { combiningsigns: CombiningSign { ayogavaha: ["\u{11301}", "𑌂", "𑌃"].iter().map(|&s| s.into()).collect(), nukta: ["\u{1133c}"].iter().map(|&s| s.into()).collect() }, consonants: Consonant { main: ["𑌕", "𑌖", "𑌗", "𑌘", "𑌙", "𑌚", "𑌛", "𑌜", "𑌝", "𑌞", "𑌟", "𑌠", "𑌡", "𑌢", "𑌣", "𑌤", "𑌥", "𑌦", "𑌧", "𑌨", "𑌪", "𑌫", "𑌬", "𑌭", "𑌮", "𑌯", "𑌰", "𑌲", "𑌵", "𑌶", "𑌷", "𑌸", "𑌹"].iter().map(|&s| s.into()).collect(), persoarabic: ["𑌕\u{1133c}", "𑌖\u{1133c}", "𑌗\u{1133c}", "𑌜\u{1133c}", "𑌡\u{1133c}", "𑌢\u{1133c}", "𑌫\u{1133c}", "𑌯\u{1133c}"].iter().map(|&s| s.into()).collect(), sinhala: ["\u{11301}ʽ𑌗", "\u{11301}ʽ𑌜", "\u{11301}ʽ𑌡", "\u{11301}ʽ𑌦", "\u{11301}ʽ𑌬"].iter().map(|&s| s.into()).collect(), south: ["𑌳", "𑌳\u{1133c}", "𑌰\u{1133c}", "𑌨\u{1133c}"].iter().map(|&s| s.into()).collect() }, numerals: ["௦", "௧", "௨", "௩", "௪", "௫", "௬", "௭", "௮", "௯"].iter().map(|&s| s.into()).collect(), others: Other { aytham: ["𑌃ʽ"].iter().map(|&s| s.into()).collect(), om: ["𑍐"].iter().map(|&s| s.into()).collect(), symbols: ["𑌽", "।", "॥"].iter().map(|&s| s.into()).collect() }, vowels: Vowel { main: ["𑌅", "𑌆", "𑌇", "𑌈", "𑌉", "𑌊", "𑌋", "𑍠", "𑌌", "𑍡", "𑌏", "𑌐", "𑌓", "𑌔"].iter().map(|&s| s.into()).collect(), modern: ["𑌏ʽ", "𑌆ʽ"].iter().map(|&s| s.into()).collect(), sinhala: ["𑌏ʽ"].iter().map(|&s| s.into()).collect(), south: ["𑌏\u{11300}", "𑌓\u{11300}"].iter().map(|&s| s.into()).collect() }, vowelsigns: VowelSign { main: ["\u{1133e}", "𑌿", "\u{11340}", "𑍁", "𑍂", "𑍃", "𑍄", "𑍢", "𑍣", "𑍇", "𑍈", "𑍋", "\u{11357}"].iter().map(|&s| s.into()).collect(), modern: ["𑍇ʽ", "\u{1133e}ʽ"].iter().map(|&s| s.into()).collect(), sinhala: ["𑍇ʽ"].iter().map(|&s| s.into()).collect(), south: ["𑍇\u{11300}", "𑍋\u{11300}"].iter().map(|&s| s.into()).collect(), virama: ["𑍍"].iter().map(|&s| s.into()).collect() } },

    "malayalam" => Script { combiningsigns: CombiningSign { ayogavaha: ["\u{d01}", "ം", "ഃ"].iter().map(|&s| s.into()).collect(), nukta: ["·"].iter().map(|&s| s.into()).collect() }, consonants: Consonant { main: ["ക", "ഖ", "ഗ", "ഘ", "ങ", "ച", "ഛ", "ജ", "ഝ", "ഞ", "ട", "ഠ", "ഡ", "ഢ", "ണ", "ത", "ഥ", "ദ", "ധ", "ന", "പ", "ഫ", "ബ", "ഭ", "മ", "യ", "ര", "ല", "വ", "ശ", "ഷ", "സ", "ഹ"].iter().map(|&s| s.into()).collect(), persoarabic: ["ക·", "ഖ·", "ഗ·", "ജ·", "ഡ·", "ഢ·", "ഫ·", "യ·"].iter().map(|&s| s.into()).collect(), sinhala: ["ംˆഗ", "ംˆജ", "ംˆഡ", "ംˆദ", "ംˆബ"].iter().map(|&s| s.into()).collect(), south: ["ള", "ഴ", "റ", "ഩ"].iter().map(|&s| s.into()).collect() }, numerals: ["൦", "൧", "൨", "൩", "൪", "൫", "൬", "൭", "൮", "൯"].iter().map(|&s| s.into()).collect(), others: Other { aytham: ["ഃʼ"].iter().map(|&s| s.into()).collect(), om: ["ഓം"].iter().map(|&s| s.into()).collect(), symbols: ["ഽ", "।", "॥"].iter().map(|&s| s.into()).collect() }, vowels: Vowel { main: ["അ", "ആ", "ഇ", "ഈ", "ഉ", "ഊ", "ഋ", "ൠ", "ഌ", "ൡ", "ഏ", "ഐ", "ഓ", "ഔ"].iter().map(|&s| s.into()).collect(), modern: ["എʼ", "ആʼ"].iter().map(|&s| s.into()).collect(), sinhala: ["ഏˇ"].iter().map(|&s| s.into()).collect(), south: ["എ", "ഒ"].iter().map(|&s| s.into()).collect() }, vowelsigns: VowelSign { main: ["\u{d3e}", "ി", "ീ", "\u{d41}", "\u{d42}", "\u{d43}", "\u{d44}", "\u{d62}", "\u{d63}", "േ", "ൈ", "ോ", "\u{d57}"].iter().map(|&s| s.into()).collect(), modern: ["െʼ", "\u{d3e}ʼ"].iter().map(|&s| s.into()).collect(), sinhala: ["േˇ"].iter().map(|&s| s.into()).collect(), south: ["െ", "ൊ"].iter().map(|&s| s.into()).collect(), virama: ["\u{d4d}"].iter().map(|&s| s.into()).collect() } },

    "tamilgrantha" => Script { combiningsigns: CombiningSign { ayogavaha: ["\u{981}", "ং", "ঃ"].iter().map(|&s| s.into()).collect(), nukta: ["\u{9bc}"].iter().map(|&s| s.into()).collect() }, consonants: Consonant { main: ["க", "খ", "গ", "ঘ", "ங", "ச", "ছ", "ஜ", "ঝ", "ஞ", "ட", "ঠ", "ড", "ঢ", "ண", "த", "থ", "দ", "ধ", "ந", "ப", "ফ", "ব", "ভ", "ம", "ய", "ர", "ல", "வ", "ஶ", "ஷ", "ஸ", "ஹ"].iter().map(|&s| s.into()).collect(), persoarabic: ["க\u{9bc}", "খ\u{9bc}", "গ\u{9bc}", "ஜ\u{9bc}", "ড\u{9bc}", "ঢ\u{9bc}", "ফ\u{9bc}", "ய\u{9bc}"].iter().map(|&s| s.into()).collect(), sinhala: ["ங\u{bcd}ˆগ", "ஞ\u{bcd}ˆஜ", "ண\u{bcd}ˆড", "ந\u{bcd}ˆদ", "ம\u{bcd}ˆব"].iter().map(|&s| s.into()).collect(), south: ["ள", "ழ", "ற", "ன"].iter().map(|&s| s.into()).collect() }, numerals: ["০", "১", "২", "৩", "৪", "৫", "৬", "৭", "৮", "৯"].iter().map(|&s| s.into()).collect(), others: Other { aytham: ["ঃʼ"].iter().map(|&s| s.into()).collect(), om: ["ௐ"].iter().map(|&s| s.into()).collect(), symbols: ["ঽ", "।", "॥"].iter().map(|&s| s.into()).collect() }, vowels: Vowel { main: ["அ", "ஆ", "இ", "ஈ", "உ", "ஊ", "ঋ", "ৠ", "ঌ", "ৡ", "ஏ", "ஐ", "ஓ", "ஔ"].iter().map(|&s| s.into()).collect(), modern: ["எʼ", "ஆʼ"].iter().map(|&s| s.into()).collect(), sinhala: ["ஏˇ"].iter().map(|&s| s.into()).collect(), south: ["எ", "ஒ"].iter().map(|&s| s.into()).collect() }, vowelsigns: VowelSign { main: ["\u{bbe}", "ி", "\u{bc0}", "ு", "ூ", "\u{9c3}", "\u{9c4}", "\u{9e2}", "\u{9e3}", "ே", "ை", "ோ", "ௌ"].iter().map(|&s| s.into()).collect(), modern: ["ெʼ", "\u{bbe}ʼ"].iter().map(|&s| s.into()).collect(), sinhala: ["ேˇ"].iter().map(|&s| s.into()).collect(), south: ["ெ", "ொ"].iter().map(|&s| s.into()).collect(), virama: ["\u{bcd}"].iter().map(|&s| s.into()).collect() } },

    "slp1" => Script { combiningsigns: CombiningSign { ayogavaha: ["~", "M", "H"].iter().map(|&s| s.into()).collect(), nukta: ["Q"].iter().map(|&s| s.into()).collect() }, consonants: Consonant { main: ["k", "K", "g", "G", "N", "c", "C", "j", "J", "Y", "w", "W", "q", "Q", "R", "t", "T", "d", "D", "n", "p", "P", "b", "B", "m", "y", "r", "l", "v", "S", "z", "s", "h"].iter().map(|&s| s.into()).collect(), persoarabic: ["k0", "K0", "g0", "j0", "q0", "Q0", "P0", "Y0"].iter().map(|&s| s.into()).collect(), sinhala: ["n*g", "n*j", "n*q", "n*d", "m*b"].iter().map(|&s| s.into()).collect(), south: ["L", "L0", "r2", "n2"].iter().map(|&s| s.into()).collect() }, numerals: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].iter().map(|&s| s.into()).collect(), others: Other { aytham: ["K"].iter().map(|&s| s.into()).collect(), om: ["oM"].iter().map(|&s| s.into()).collect(), symbols: ["'", ".", ".."].iter().map(|&s| s.into()).collect() }, vowels: Vowel { main: ["a", "A", "i", "I", "u", "U", "f", "F", "x", "X", "e", "E", "o", "O"].iter().map(|&s| s.into()).collect(), modern: ["e2", "o2"].iter().map(|&s| s.into()).collect(), sinhala: ["e4"].iter().map(|&s| s.into()).collect(), south: ["e1", "o1"].iter().map(|&s| s.into()).collect() }, vowelsigns: VowelSign { main: ["A", "i", "I", "u", "U", "f", "F", "x", "X", "e", "E", "o", "O"].iter().map(|&s| s.into()).collect(), modern: ["e2", "o2"].iter().map(|&s| s.into()).collect(), sinhala: ["e4"].iter().map(|&s| s.into()).collect(), south: ["e1", "o1"].iter().map(|&s| s.into()).collect(), virama: ["×"].iter().map(|&s| s.into()).collect() } },

    "iso" => Script { combiningsigns: CombiningSign { ayogavaha: ["m\u{310}", "ṁ", "ḥ"].iter().map(|&s| s.into()).collect(), nukta: ["\u{308}"].iter().map(|&s| s.into()).collect() }, consonants: Consonant { main: ["k", "kh", "g", "gh", "ṅ", "c", "ch", "j", "jh", "ñ", "ṭ", "ṭh", "ḍ", "ḍh", "ṇ", "t", "th", "d", "dh", "n", "p", "ph", "b", "bh", "m", "y", "r", "l", "v", "ś", "ṣ", "s", "h"].iter().map(|&s| s.into()).collect(), persoarabic: ["q", "k\u{35f}h", "ġ", "z", "ṛ", "ṛh", "f", "ẏ"].iter().map(|&s| s.into()).collect(), sinhala: ["n\u{306}g", "n\u{306}j", "n\u{306}ḍ", "n\u{306}d", "m\u{306}b"].iter().map(|&s| s.into()).collect(), south: ["ḷ", "ḻ", "ṟ", "ṉ"].iter().map(|&s| s.into()).collect() }, numerals: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].iter().map(|&s| s.into()).collect(), others: Other { aytham: ["ḵ"].iter().map(|&s| s.into()).collect(), om: ["ōṁ"].iter().map(|&s| s.into()).collect(), symbols: ["’", ".", ".."].iter().map(|&s| s.into()).collect() }, vowels: Vowel { main: ["a", "ā", "i", "ī", "u", "ū", "r\u{325}", "r\u{325}\u{304}", "l\u{325}", "l\u{325}\u{304}", "ē", "ai", "ō", "au"].iter().map(|&s| s.into()).collect(), modern: ["æ", "ô"].iter().map(|&s| s.into()).collect(), sinhala: ["ǣ"].iter().map(|&s| s.into()).collect(), south: ["e", "o"].iter().map(|&s| s.into()).collect() }, vowelsigns: VowelSign { main: ["ā", "i", "ī", "u", "ū", "r\u{325}", "r\u{325}\u{304}", "l\u{325}", "l\u{325}\u{304}", "ē", "ai", "ō", "au"].iter().map(|&s| s.into()).collect(), modern: ["æ", "ô"].iter().map(|&s| s.into()).collect(), sinhala: ["ǣ"].iter().map(|&s| s.into()).collect(), south: ["e", "o"].iter().map(|&s| s.into()).collect(), virama: ["×"].iter().map(|&s| s.into()).collect() } },

    "itrans" => Script { combiningsigns: CombiningSign { ayogavaha: [".N", "M", "H"].iter().map(|&s| s.into()).collect(), nukta: ["Q"].iter().map(|&s| s.into()).collect() }, consonants: Consonant { main: ["k", "kh", "g", "gh", "~N", "ch", "Ch", "j", "jh", "~n", "T", "Th", "D", "Dh", "N", "t", "th", "d", "dh", "n", "p", "ph", "b", "bh", "m", "y", "r", "l", "v", "sh", "Sh", "s", "h"].iter().map(|&s| s.into()).collect(), persoarabic: ["q", "K", "G", "z", ".D", ".Dh", "f", "Y"].iter().map(|&s| s.into()).collect(), sinhala: ["n*g", "n*j", "n*D", "n*d", "m*b"].iter().map(|&s| s.into()).collect(), south: ["L", "zh", "R", "^n"].iter().map(|&s| s.into()).collect() }, numerals: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].iter().map(|&s| s.into()).collect(), others: Other { aytham: ["K^"].iter().map(|&s| s.into()).collect(), om: ["oM"].iter().map(|&s| s.into()).collect(), symbols: [".a", ".", ".."].iter().map(|&s| s.into()).collect() }, vowels: Vowel { main: ["a", "A", "i", "I", "u", "U", "R^i", "R^I", "L^i", "L^I", "e", "ai", "o", "au"].iter().map(|&s| s.into()).collect(), modern: ["e.c", "A.c"].iter().map(|&s| s.into()).collect(), sinhala: ["a.C"].iter().map(|&s| s.into()).collect(), south: ["^e", "^o"].iter().map(|&s| s.into()).collect() }, vowelsigns: VowelSign { main: ["A", "i", "I", "u", "U", "R^i", "R^I", "L^i", "L^I", "e", "ai", "o", "au"].iter().map(|&s| s.into()).collect(), modern: ["e.c", "A.c"].iter().map(|&s| s.into()).collect(), sinhala: ["a.C"].iter().map(|&s| s.into()).collect(), south: ["^e", "^o"].iter().map(|&s| s.into()).collect(), virama: ["×"].iter().map(|&s| s.into()).collect() } },

    "iast" => Script { combiningsigns: CombiningSign { ayogavaha: ["m\u{310}", "ṃ", "ḥ"].iter().map(|&s| s.into()).collect(), nukta: ["\u{308}"].iter().map(|&s| s.into()).collect() }, consonants: Consonant { main: ["k", "kh", "g", "gh", "ṅ", "c", "ch", "j", "jh", "ñ", "ṭ", "ṭh", "ḍ", "ḍh", "ṇ", "t", "th", "d", "dh", "n", "p", "ph", "b", "bh", "m", "y", "r", "l", "v", "ś", "ṣ", "s", "h"].iter().map(|&s| s.into()).collect(), persoarabic: ["q", "k\u{35f}h", "ġ", "z", "r\u{324}", "r\u{324}h", "f", "ẏ"].iter().map(|&s| s.into()).collect(), sinhala: ["n\u{306}g", "n\u{306}j", "n\u{306}ḍ", "n\u{306}d", "m\u{306}b"].iter().map(|&s| s.into()).collect(), south: ["l\u{324}", "ḻ", "ṟ", "ṉ"].iter().map(|&s| s.into()).collect() }, numerals: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].iter().map(|&s| s.into()).collect(), others: Other { aytham: ["ḵ"].iter().map(|&s| s.into()).collect(), om: ["oṃ"].iter().map(|&s| s.into()).collect(), symbols: ["'", ".", ".."].iter().map(|&s| s.into()).collect() }, vowels: Vowel { main: ["a", "ā", "i", "ī", "u", "ū", "ṛ", "ṝ", "ḷ", "ḹ", "e", "ai", "o", "au"].iter().map(|&s| s.into()).collect(), modern: ["æ", "ô"].iter().map(|&s| s.into()).collect(), sinhala: ["ǣ"].iter().map(|&s| s.into()).collect(), south: ["ĕ", "ŏ"].iter().map(|&s| s.into()).collect() }, vowelsigns: VowelSign { main: ["ā", "i", "ī", "u", "ū", "ṛ", "ṝ", "ḷ", "ḹ", "e", "ai", "o", "au"].iter().map(|&s| s.into()).collect(), modern: ["æ", "ô"].iter().map(|&s| s.into()).collect(), sinhala: ["ǣ"].iter().map(|&s| s.into()).collect(), south: ["ĕ", "ŏ"].iter().map(|&s| s.into()).collect(), virama: ["×"].iter().map(|&s| s.into()).collect() } },

    "devanagari" => Script { combiningsigns: CombiningSign { ayogavaha: ["\u{901}", "\u{902}", "ः"].iter().map(|&s| s.into()).collect(), nukta: ["\u{93c}"].iter().map(|&s| s.into()).collect() }, consonants: Consonant { main: ["क", "ख", "ग", "घ", "ङ", "च", "छ", "ज", "झ", "ञ", "ट", "ठ", "ड", "ढ", "ण", "त", "थ", "द", "ध", "न", "प", "फ", "ब", "भ", "म", "य", "र", "ल", "व", "श", "ष", "स", "ह"].iter().map(|&s| s.into()).collect(), persoarabic: ["क़", "ख़", "ग़", "ज़", "ड़", "ढ़", "फ़", "य़"].iter().map(|&s| s.into()).collect(), sinhala: ["\u{901}ˆग", "\u{901}ˆज", "\u{901}ˆड", "\u{901}ˆद", "\u{901}ˆब"].iter().map(|&s| s.into()).collect(), south: ["ळ", "ऴ", "ऱ", "ऩ"].iter().map(|&s| s.into()).collect() }, numerals: ["०", "१", "२", "३", "४", "५", "६", "७", "८", "९"].iter().map(|&s| s.into()).collect(), others: Other { aytham: ["ःʼ"].iter().map(|&s| s.into()).collect(), om: ["ॐ"].iter().map(|&s| s.into()).collect(), symbols: ["ऽ", "।", "॥"].iter().map(|&s| s.into()).collect() }, vowels: Vowel { main: ["अ", "आ", "इ", "ई", "उ", "ऊ", "ऋ", "ॠ", "ऌ", "ॡ", "ए", "ऐ", "ओ", "औ"].iter().map(|&s| s.into()).collect(), modern: ["ऍ", "ऑ"].iter().map(|&s| s.into()).collect(), sinhala: ["ए\u{955}"].iter().map(|&s| s.into()).collect(), south: ["ऎ", "ऒ"].iter().map(|&s| s.into()).collect() }, vowelsigns: VowelSign { main: ["ा", "ि", "ी", "\u{941}", "\u{942}", "\u{943}", "\u{944}", "\u{962}", "\u{963}", "\u{947}", "\u{948}", "ो", "ौ"].iter().map(|&s| s.into()).collect(), modern: ["\u{945}", "ॉ"].iter().map(|&s| s.into()).collect(), sinhala: ["\u{955}"].iter().map(|&s| s.into()).collect(), south: ["\u{946}", "ॊ"].iter().map(|&s| s.into()).collect(), virama: ["\u{94d}"].iter().map(|&s| s.into()).collect() } },

    "tamil" => Script { combiningsigns: CombiningSign { ayogavaha: ["ம\u{bcd}ˮ", "ம\u{bcd}ʼ", "꞉"].iter().map(|&s| s.into()).collect(), nukta: ["·"].iter().map(|&s| s.into()).collect() }, consonants: Consonant { main: ["க", "க²", "க³", "க⁴", "ங", "ச", "ச²", "ஜ", "ஜ²", "ஞ", "ட", "ட²", "ட³", "ட⁴", "ண", "த", "த²", "த³", "த⁴", "ந", "ப", "ப²", "ப³", "ப⁴", "ம", "ய", "ர", "ல", "வ", "ஶ", "ஷ", "ஸ", "ஹ"].iter().map(|&s| s.into()).collect(), persoarabic: ["ஃʼக", "ஃக²", "ஃக³", "ஃஜ", "ஃட²", "ஃட³", "ஃப", "ஃய"].iter().map(|&s| s.into()).collect(), sinhala: ["ங\u{bcd}ˆக³", "ஞ\u{bcd}ˆஜ³", "ண\u{bcd}ˆட³", "ந\u{bcd}ˆத³", "ம\u{bcd}ˆப³"].iter().map(|&s| s.into()).collect(), south: ["ள", "ழ", "ற", "ன"].iter().map(|&s| s.into()).collect() }, numerals: ["௦", "௧", "௨", "௩", "௪", "௫", "௬", "௭", "௮", "௯"].iter().map(|&s| s.into()).collect(), others: Other { aytham: ["ஃ"].iter().map(|&s| s.into()).collect(), om: ["ௐ"].iter().map(|&s| s.into()).collect(), symbols: ["(அ)", "।", "॥"].iter().map(|&s| s.into()).collect() }, vowels: Vowel { main: ["அ", "ஆ", "இ", "ஈ", "உ", "ஊ", "ருʼ", "ரூʼ", "லுʼ", "லூʼ", "ஏ", "ஐ", "ஓ", "ஔ"].iter().map(|&s| s.into()).collect(), modern: ["எʼ", "ஆʼ"].iter().map(|&s| s.into()).collect(), sinhala: ["ஏˇ"].iter().map(|&s| s.into()).collect(), south: ["எ", "ஒ"].iter().map(|&s| s.into()).collect() }, vowelsigns: VowelSign { main: ["\u{bbe}", "ி", "\u{bc0}", "ு", "ூ", "\u{bcd}ருʼ", "\u{bcd}ரூʼ", "\u{bcd}லுʼ", "\u{bcd}லூʼ", "ே", "ை", "ோ", "ௌ"].iter().map(|&s| s.into()).collect(), modern: ["ெʼ", "\u{bbe}ʼ"].iter().map(|&s| s.into()).collect(), sinhala: ["ேˇ"].iter().map(|&s| s.into()).collect(), south: ["ெ", "ொ"].iter().map(|&s| s.into()).collect(), virama: ["\u{bcd}"].iter().map(|&s| s.into()).collect() } },

    "hk" => Script { combiningsigns: CombiningSign { ayogavaha: ["~", "M", "H"].iter().map(|&s| s.into()).collect(), nukta: ["Q"].iter().map(|&s| s.into()).collect() }, consonants: Consonant { main: ["k", "kh", "g", "gh", "G", "c", "ch", "j", "jh", "J", "T", "Th", "D", "Dh", "N", "t", "th", "d", "dh", "n", "p", "ph", "b", "bh", "m", "y", "r", "l", "v", "z", "S", "s", "h"].iter().map(|&s| s.into()).collect(), persoarabic: ["q", "qh", "g2", "z2", "r3", "r3h", "f", "Y"].iter().map(|&s| s.into()).collect(), sinhala: ["n*g", "n*j", "n*D", "n*d", "m*b"].iter().map(|&s| s.into()).collect(), south: ["L", "Z", "r2", "n2"].iter().map(|&s| s.into()).collect() }, numerals: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].iter().map(|&s| s.into()).collect(), others: Other { aytham: ["K"].iter().map(|&s| s.into()).collect(), om: ["oM"].iter().map(|&s| s.into()).collect(), symbols: ["'", ".", ".."].iter().map(|&s| s.into()).collect() }, vowels: Vowel { main: ["a", "A", "i", "I", "u", "U", "R", "RR", "lR", "lRR", "e", "ai", "o", "au"].iter().map(|&s| s.into()).collect(), modern: ["aE", "aO"].iter().map(|&s| s.into()).collect(), sinhala: ["AE"].iter().map(|&s| s.into()).collect(), south: ["E", "O"].iter().map(|&s| s.into()).collect() }, vowelsigns: VowelSign { main: ["A", "i", "I", "u", "U", "R", "RR", "lR", "lRR", "e", "ai", "o", "au"].iter().map(|&s| s.into()).collect(), modern: ["aE", "aO"].iter().map(|&s| s.into()).collect(), sinhala: ["AE"].iter().map(|&s| s.into()).collect(), south: ["E", "O"].iter().map(|&s| s.into()).collect(), virama: ["×"].iter().map(|&s| s.into()).collect() } },

    };

}

lazy_static! {
    #[derive(Debug)]
    pub static ref SCRIPT_INTERMEDIATE: ScriptIntermediate = {
        let combiningsigns = CombiningSignIntermediate {
            ayogavaha: vec![Ayogavaha::C, Ayogavaha::M, Ayogavaha::H],
            nukta: vec![Nukta::Q],
        };

        let consonants = ConsonantIntermediate {
            main: vec![
                ConsonantsMain::k,
                ConsonantsMain::K,
                ConsonantsMain::g,
                ConsonantsMain::G,
                ConsonantsMain::N,
                ConsonantsMain::c,
                ConsonantsMain::C,
                ConsonantsMain::j,
                ConsonantsMain::J,
                ConsonantsMain::Y,
                ConsonantsMain::w,
                ConsonantsMain::W,
                ConsonantsMain::q,
                ConsonantsMain::Q,
                ConsonantsMain::R,
                ConsonantsMain::t,
                ConsonantsMain::T,
                ConsonantsMain::d,
                ConsonantsMain::D,
                ConsonantsMain::n,
                ConsonantsMain::p,
                ConsonantsMain::P,
                ConsonantsMain::b,
                ConsonantsMain::B,
                ConsonantsMain::m,
                ConsonantsMain::y,
                ConsonantsMain::r,
                ConsonantsMain::l,
                ConsonantsMain::v,
                ConsonantsMain::S,
                ConsonantsMain::z,
                ConsonantsMain::s,
                ConsonantsMain::h,
            ],
            persoarabic: vec![
                PersoArabic::k0,
                PersoArabic::K0,
                PersoArabic::g0,
                PersoArabic::j0,
                PersoArabic::q0,
                PersoArabic::Q0,
                PersoArabic::P0,
                PersoArabic::Y0,
            ],
            sinhala: vec![
                Sinhala::n_g,
                Sinhala::n_j,
                Sinhala::n_q,
                Sinhala::n_d,
                Sinhala::m_b,
            ],
            south: vec![South::L, South::L0, South::r2, South::n2],
        };

        let numerals = vec![
            Numerals::zero,
            Numerals::one,
            Numerals::two,
            Numerals::three,
            Numerals::four,
            Numerals::five,
            Numerals::six,
            Numerals::seven,
            Numerals::eight,
            Numerals::nine,
        ];

        let others = OtherIntermediate {
            aytham: vec![Aytham::K],
            om: vec![Om::oM],
            symbols: vec![Symbols::A, Symbols::D, Symbols::DD],
        };

        let vowels = VowelIntermediate {
            main: vec![
                VowelMain::a,
                VowelMain::A,
                VowelMain::i,
                VowelMain::I,
                VowelMain::u,
                VowelMain::U,
                VowelMain::f,
                VowelMain::F,
                VowelMain::x,
                VowelMain::X,
                VowelMain::e,
                VowelMain::E,
                VowelMain::o,
                VowelMain::O,
            ],
            modern: vec![VowelModern::e2, VowelModern::o2],
            sinhala: vec![VowelSinhala::e4],
            south: vec![VowelSouth::e1, VowelSouth::o1],
        };

        let vowelsigns = VowelSignIntermediate {
            main: vec![
                VowelSignMain::A,
                VowelSignMain::i,
                VowelSignMain::I,
                VowelSignMain::u,
                VowelSignMain::U,
                VowelSignMain::f,
                VowelSignMain::F,
                VowelSignMain::x,
                VowelSignMain::X,
                VowelSignMain::e,
                VowelSignMain::E,
                VowelSignMain::o,
                VowelSignMain::O,
            ],
            modern: vec![VowelSignModern::e2, VowelSignModern::o2],
            sinhala: vec![VowelSignSinhala::e4],
            south: vec![VowelSignSouth::e1, VowelSignSouth::o1],
            virama: vec![VowelSignVirama::x],
        };

        return ScriptIntermediate {
            combiningsigns: combiningsigns,
            consonants: consonants,
            numerals: numerals,
            others: others,
            vowels: vowels,
            vowelsigns: vowelsigns,
        }
    };
}
