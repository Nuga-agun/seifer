use seifer_core::*;

use clap::{Parser,ValueEnum};
use std::io::{Write,Read};
use std::fs::File;


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
}

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct Seifer {
    algorithm: Algorithm,
    key: String,
    input: String,

    #[arg(short,long)]
    decrypt: bool,

    #[arg(short,long)]
    read_from_file: bool,

    #[arg(short,long)]
    output_file: Option<String>,

    #[arg(short,long)]
    format_clear: Option<Format>,
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

    let result = match cli.algorithm {
        Algorithm::Caesar => Caesar::process_stream(cli.key.as_bytes(), &input, cli.decrypt),
        Algorithm::Rc4 => Rc4::process_stream(cli.key.as_bytes(), &input, cli.decrypt),
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
