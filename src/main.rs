use IndicScriptSwap::convert::{
    convert_indic_to_indic, convert_indic_to_roman, convert_roman_to_indic, convert_roman_to_roman,
};
use IndicScriptSwap::data::HASH_MAP;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

const def_input: String = String::new();
const def_output: String = String::new();

#[derive(Parser, Debug)]
#[clap(author="Pradyumna Malladi", version = "0.6.0", about="This is a tool to help transliterate between various indic scripts. 
It is not ready yet and has many issues.
If you encounter any issues, please contact me (https://github.com/mssrprad/transliterator-ferris or pradyumna.malladi2003@gmail.com)")]
struct Args {
    
   
    #[arg(short, long)]
    source: String,

    /// Print Details
    /// 1 => detailed
    /// 2 => short
    /// Default (0) => None
    #[arg(short, long, default_value_t)]
    more: usize,

    /// Destination Script
    #[arg(short, long)]
    destination: String,

    /// Input File (With no special characters that shouldn't be transliterated)(Default: stdout)
    #[arg(short, long, default_value_t = def_input)]
    input: String,

    /// Output File (Default: stdout)
    #[arg(short, long, default_value_t = def_output)]
    output: String,

    /// Type of Conversion (i2i/i2r/r2r/r2i)
    #[arg(short, long)]
    conversion: String,

}



fn main() -> std::io::Result<()> {

    let args = Args::parse();
    let mut contents = String::new();
    let source = HASH_MAP.get(&args.source as &str).unwrap();
    let destination = HASH_MAP.get(&args.destination as &str).unwrap();
    let converted: String;

    match args.more {
        1 => {
            
            
            
            println!("

            ###                                  #####                                          #####                          
            #   #    #  #####   #   ####       #     #   ####   #####   #  #####   #####      #     #  #    #    ##    #####  
            #   ##   #  #    #  #  #    #      #        #    #  #    #  #  #    #    #        #        #    #   #  #   #    # 
            #   # #  #  #    #  #  #            #####   #       #    #  #  #    #    #         #####   #    #  #    #  #    # 
            #   #  # #  #    #  #  #                 #  #       #####   #  #####     #              #  # ## #  ######  #####  
            #   #   ##  #    #  #  #    #      #     #  #    #  #   #   #  #         #        #     #  ##  ##  #    #  #      
           ###  #    #  #####   #   ####        #####    ####   #    #  #  #         #         #####   #    #  #    #  #      
                                                                                                                              
            
            This is a tool to help transliterate between various indic scripts. 
            It is not ready yet and has many issues.
            If you encounter any issues, please contact me (https://github.com/mssrprad/transliterator-ferris or pradyumna.malladi2003@gmail.com)
            
            Usage: transliterate_ferris [OPTIONS] --source <SOURCE> --destination <DESTINATION> --conversion <CONVERSION>

            Options:
            -s, --source <SOURCE>            Source Script
            -d, --destination <DESTINATION>  Destination Script
            -i, --input <INPUT>              Input File (With no special characters that shouldn't be transliterated)(Default: stdout) [default: ]
            -o, --output <OUTPUT>            Output File (Default: stdout) [default: ]
            -c, --conversion <CONVERSION>    Type of Conversion (i2i/i2r/r2r/r2i)
            -h, --help                       Print help
            -V, --version                    Print version
            
            
            Some Scripts with their types (Taken from aksharamukha-python)

            IndicScripts = [
                        'RomanSemitic',
                            'Makasar',
                            'Nandinagari',
                            'Kawi',
                            'Shahmukhi',
                            'Pallava',
                            'Hebrew',
                        'LaoTham',
                        'LueTham',
                        'KhuenTham',
                        'TamilExtended',
                        'Marchen',
                        'MasaramGondi',
                        'GunjalaGondi',
                        'Soyombo',
                        'Dogra',
                        'KhomThai',
                        'KhamtiShan',
                        'TaiLaing',
                        'Mon',
                        'Khojki',
                        'Shan',
                        'Ranjana',
                        'ZanabazarSquare',
                        'Rejang',
                        'GranthaGrantamil',
                        'Devanagari',
                        'Multani',
                        'Ahom',
                        'Mahajani',
                        'Lao2',
                        'Hanunoo',
                        'Buhid',
                        'Siddham',
                        'SiddhamDevanagari',
                        'GranthaPandya',
                        'Vatteluttu',
                        'Khudawadi',
                        'Bhaiksuki',
                        'Sharada',
                        'Newa',
                        'Takri',
                        'SylotiNagri',
                        'Tirhuta',
                        'Modi',
                        'Kaithi',
                        'Kharoshthi',
                        'Telugu',
                        'Kannada',
                        'Malayalam',
                        'Gujarati',
                        'Bengali',
                        'Oriya',
                        'Gurmukhi',
                        'Tamil',
                        'Assamese',
                        'Saurashtra',
                        'TamilBrahmi',
                        'Grantha',
                        'TamilGrantha',
                        'Sinhala',
                        'Khmer',
                        'Burmese',
                        'Urdu',
                        'Balinese',
                        'Javanese',
                        'Thaana',
                        'Tibetan',
                        'Thai',
                        'OldPersian',
                        'Limbu',
                        'Lepcha',
                        'Sundanese',
                        'Tagalog',
                        'Tagbanwa',
                        'Buginese',
                        'Chakma',
                        'PhagsPa',
                        'MeeteiMayek',
                        'LaoPali',
                        'BatakKaro','BatakPakpak','BatakSima','BatakToba','BatakManda',
                        'Cham',
                        'TaiTham',
                        'Lao',
                        'Brahmi'
                        ]

            LatinScripts = ['IASTLOC', 'RomanColloquial', 'ISOPali', 'RomanKana', 'BarahaNorth', 'BarahaSouth', 'Mongolian', 'SLP1', 'Wancho', 'Mro', 'IASTPali', 'HanifiRohingya','Ariyaka', 'RomanReadable', 'Aksharaa', 'WarangCiti', 'SoraSompeng','WX-kok','Avestan','ISO','IAST','HK','Titus','Itrans','Velthuis','WX','Inter','IPA','TolongSiki','Santali','RussianCyrillic']
            
            ")
        }, 
        2 => {
            println!("
            
            ###                                  #####                                          #####                          
  #   #    #  #####   #   ####       #     #   ####   #####   #  #####   #####      #     #  #    #    ##    #####  
  #   ##   #  #    #  #  #    #      #        #    #  #    #  #  #    #    #        #        #    #   #  #   #    # 
  #   # #  #  #    #  #  #            #####   #       #    #  #  #    #    #         #####   #    #  #    #  #    # 
  #   #  # #  #    #  #  #                 #  #       #####   #  #####     #              #  # ## #  ######  #####  
  #   #   ##  #    #  #  #    #      #     #  #    #  #   #   #  #         #        #     #  ##  ##  #    #  #      
 ###  #    #  #####   #   ####        #####    ####   #    #  #  #         #         #####   #    #  #    #  #      
                                                                                                                    
            
            
            This is a tool to help transliterate between various indic scripts. 
            It is not ready yet and has many issues.
            If you encounter any issues, please contact me (https://github.com/mssrprad/transliterator-ferris or pradyumna.malladi2003@gmail.com)
            
            Usage: transliterate_ferris [OPTIONS] --source <SOURCE> --destination <DESTINATION> --conversion <CONVERSION>

            Options:
            -s, --source <SOURCE>            Source Script
            -d, --destination <DESTINATION>  Destination Script
            -i, --input <INPUT>              Input File (With no special characters that shouldn't be transliterated)(Default: stdout) [default: ]
            -o, --output <OUTPUT>            Output File (Default: stdout) [default: ]
            -c, --conversion <CONVERSION>    Type of Conversion (i2i/i2r/r2r/r2i)
            -h, --help                       Print help
            -V, --version                    Print version
            
            ")
        }, 
        0 | _ => {

        },
    }

    let mut input: Box<dyn BufRead> = if !args.input.is_empty() {
        Box::new(BufReader::new(File::open(args.input)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    input.read_to_string(&mut contents)?;

    match args.conversion.as_str() {
        "i2i" => converted = convert_indic_to_indic(&contents, source, destination),
        "i2r" => converted = convert_indic_to_roman(&contents, source, destination),
        "r2i" => converted = convert_roman_to_indic(&contents, source, destination),
        "r2r" => converted = convert_roman_to_roman(&contents, source, destination),
        _ => converted = String::from("Could Not Convert!"),
    }

    if !args.output.is_empty() {
        let mut file = File::create(args.output)?;
        file.write_all(converted.as_bytes())?;
    } else {
        println!("{}", converted);
    }

    Ok(())
    
}