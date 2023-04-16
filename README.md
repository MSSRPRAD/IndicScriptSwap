# transliterate-ferris

***Rust Library to transliterate between scripts.***

## Setup and Usage

`git clone https://github.com/MSSRPRAD/ transliterate-ferris.git`

`cd transliterate-ferris`

`time cargo run --release 1>out.txt 2>error.txt`

`out.txt` contains the output.
`error.txt` contains the warnings and error messages emmitted by the program.

`main.rs` contains a sort of 'demo'. The input text is given by:

`let input: String = "अस्त्य् उत्तरस्यां दिशि देवतात्मा हिमालयो नाम नगाधिराजः ।
    पूर्वापरौ तोयनिधी विगाह्य स्थितः पृथिव्या इव मानदण्डः ॥"
        .to_string();`

You can change it to try different test cases.

The output generated for the above input text was:

`"asty uttarasyAM diSi devatAtmA himAlayo nAma nagADirAjaH .\n    pUrvAparO toyaniDI vigAhya sTitaH pfTivyA iva mAnadaRqaH .."`

### NOTE

Only devanagari -> slp1 and slp1 -> devanagari (and as a consequence probably a few other scripts (I have checked telugu in place of devanagari)) is implemented as of now.

## <b> Some conversions that that I have tested (Need to test more) :</b>

(Note: Can't convert from iast/hk to any because code requires seeing two characters of input text at once (current and previous) and only in slp1 each sound has one character.)

Many more transliterations between scripts not mentioned here can be achieved indirectly (eg: telugu->devanagari = telugu->slp1->devanagari)

| source key | destination key | function                 |
| ---------- | --------------- | ------------------------ |
| devanagari | slp1            | convert_indic_to_roman() |
| devanagari | iast            | convert_indic_to_roman() |
| devanagari | hk              | convert_indic_to_roman() |
| telugu     | slp1            | convert_indic_to_roman() |
| telugu     | iast            | convert_indic_to_roman() |
| telugu     | hk              | convert_indic_to_roman() |
| kannada    | slp1            | convert_indic_to_roman() |
| kannada    | iast            | convert_indic_to_roman() |
| kannada    | hk              | convert_indic_to_roman() |
| slp1       | devanagari      | convert_roman_to_indic() |
| slp1       | telugu          | convert_roman_to_indic() |
| slp1       | kannada         | convert_roman_to_indic() |
|            |                 |                          |

Primary Focus is transliterating between devanagari to slp1 as many programs require input in slp1 and rust has no transliteration library from devanagari -> slp1 yet.
