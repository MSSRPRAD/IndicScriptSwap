# transliterate-ferris
***Rust Library to transliterate between scripts.***

## See CLI Branch for cli-application

IMP: Don't use rust-analyzer if you're working on the code. It has crashed my pc multiple times on both vscode and nvim. I suspect the data.rs is causing this issue.

INSTALLATION:

`cargo install IndicScriptSwap`

USAGE:

`IndicScriptSwap --source devanagari --destination telugu --conversion i2i --input data.txt --output out.txt`

Note: Piping is also supported for input and output:
`cat data.txt | IndicScriptSwap --source devanagari --destination telugu --conversion i2i`



## Issues

1. Outrageous build time (30-60 seconds!!!)
   1. I thought removing the `serde` and `json` dependencies and reading from data.rs would improve the build time but it made it much much worse.
   2. The code does run a bit faster with this.
   3. rust-analyzer crashing on vs-code due to this.
2. ~6 millisecond vs ~25 millisecond by `aksharamukha-python` on the same conversion. (Can be made even better)

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
