# transliterate-ferris
***Rust Library to transliterate between scripts.***

## Setup and Usage:

`git clone https://github.com/MSSRPRAD/ transliterate-ferris.git`

`cd transliterate-ferris`

`time cargo run --release`

`main.rs` contains a sort of 'demo'. The input text is given by:

`let input: String = "अस्त्य् उत्तरस्यां दिशि देवतात्मा हिमालयो नाम नगाधिराजः ।
    पूर्वापरौ तोयनिधी विगाह्य स्थितः पृथिव्या इव मानदण्डः ॥"
        .to_string();`

You can change it to try different test cases.

The output generated for the above input text was:

`"asty uttarasyAM diSi devatAtmA himAlayo nAma nagADirAjaH .\n    pUrvAparO toyaniDI vigAhya sTitaH pfTivyA iva mAnadaRqaH .."`

### NOTE:

Only devanagari -> slp1 (and as a consequence probably a few other roman scripts) is implemented as of now.

Primary Focus is transliterating between devanagari to slp1 as many programs require input in slp1 and rust has no transliteration library from devanagari -> slp1 yet.
