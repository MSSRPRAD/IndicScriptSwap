# transliterate-ferris

***Rust Library to transliterate between scripts.***

## Issues

1. Outrageous build time (36 seconds!!!)
   1. I thought removing the `serde` and `json` dependencies and reading from data.rs would improve the build time but it made it much much worse.
   2. The code does run a bit faster with this.
   3. rust-analyzer crashing on vs-code due to this.
2. ~40 millisecond vs ~120 millisecond by `aksharamukha-python` on the same conversion. (Can be made even better)
3. Too many ugly nested code blocks.
4. No tests so changes can have unintended consequences.

Despite these issues it seems sufficient for basic conversion requirement.

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

## <b> Some conversions that that I have tested (Need to test more) :</b>

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
| slp1       | itrans          | convert_roman_to_roman() |
| iast       | slp1            | convert_roman_to_roman() |
| slp1       | iast            | convert_roman_to_roman() |
| iast       | slp1            | convert_roman_to_roman() |
| iast       | itrans          | convert_roman_to_roman() |
| iast       | hk              | convert_roman_to_roman() |

Primary Focus was transliterating between devanagari to slp1 as many programs require input in slp1 and rust has no transliteration library from devanagari -> slp1 yet. It turned out making the convert_roman_to_indic() and convert_roman_to_roman() was not much different from this so I did that too.
