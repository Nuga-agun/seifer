use seifer_core::*;
use clap::{Parser,ValueEnum};
use std::io::{Write,Read};
use std::fs::File;
use std::fmt::{self,Display};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Utf8,
    Hex,
    Bytes
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    Caesar,
    Rc4,
    Hc128,
    Tea,
}
impl Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Caesar => "Caesar",
            Self::Rc4 => "Rc4",
            Self::Hc128 => "Hc128",
            Self::Tea => "Tea",
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum BlockMode {
    Ecb,
}

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct Seifer {
    algorithm: Algorithm,
    key: String,
    input: String,
    iv: Option<String>,

    #[arg(short,long)]
    decrypt: bool,

    #[arg(short,long)]
    read_from_file: bool,

    #[arg(short,long)]
    output_file: Option<String>,

    #[arg(short,long)]
    format_clear: Option<Format>,

    #[arg(short,long)]
    block_mode: Option<BlockMode>,
}

fn main() {
    let cli = Seifer::parse();

    let format = cli.format_clear.unwrap_or(Format::Utf8);
    let (input_format, output_format) = match cli.decrypt {
        true => (Format::Hex,format),
        false => (format,match &cli.output_file {Some(_) => Format::Bytes, None => Format::Hex})
    };

    let input = match cli.read_from_file {
        true => {
            let mut file = File::open(cli.input).expect("Error while openning input file");
            let mut input: Vec<u8> = Vec::new();
            file.read_to_end(&mut input).expect("Error while reading input file");
            input
        },
        false => convert_to_bytes(cli.input, input_format)
    };

    let block_ciphers = vec![Algorithm::Tea];
    if cli.block_mode.is_none() && block_ciphers.contains(&cli.algorithm) {
        panic!("Error: a block mode is required for {}", cli.algorithm);
    }

    let result = match cli.algorithm {
        Algorithm::Caesar => Caesar::process_stream(cli.key.as_bytes(), &(), &input, cli.decrypt),
        Algorithm::Rc4 => Rc4::process_stream(cli.key.as_bytes(), &(), &input, cli.decrypt),
        Algorithm::Hc128 => {
            let iv = match cli.decrypt {
                true => {
                    let bytes = convert_to_bytes(cli.iv.unwrap_or_else(|| panic!("Error : an initialization vector is necessary for decrypting with Hc-128")), Format::Hex);
                    Hc128::iv_from_bytes(&bytes).unwrap_or_else(|msg| panic!("Error while parsing initialization vector : {msg}"))
                },
                false => Hc128::random_iv(),
            };
            print!("IV : ");
            for byte in Hc128::bytes_from_iv(&iv) {print!("{:02x}", byte);}
            println!("");
            Hc128::process_stream(cli.key.as_bytes(), &iv, &input, cli.decrypt)
        },
        Algorithm::Tea => Tea::process_stream::<Ecb>(cli.key.as_bytes(), &[], &input, cli.decrypt),
    }.unwrap_or_else(|e| panic!("Error : {e}"));

    match cli.output_file {
        Some(path) => {
            let mut file = File::create(path).expect("Error while creating output file");
            if output_format == Format::Bytes {
                file.write_all(&result).expect("Error while writing to the output file");
            } else {
                write!(file, "{}", convert_from_bytes(&result, output_format)).expect("Error while writing to the output file");
            }
        },
        None => println!("{}", convert_from_bytes(&result, output_format))
    };

}

fn convert_to_bytes(input: String, format: Format) -> Vec<u8> {
    match format {
        Format::Utf8 => input.into_bytes().to_vec(),
        Format::Hex => hex::decode(input).unwrap_or_else(|e| panic!("Error while parsing from hex : {e}")),
        Format::Bytes => todo!(),
    }
}

fn convert_from_bytes(input: &[u8], format: Format) -> String {
    match format {
        Format::Utf8 => String::from_utf8(input.to_vec()).unwrap_or_else(|e| panic!("Error while parsing to utf-8 : {e}")),
        Format::Hex => input.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>().join(""),
        Format::Bytes => unimplemented!("To print bytes, use Hex or Bin format")
    }
}
