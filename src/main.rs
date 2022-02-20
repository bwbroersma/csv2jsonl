extern crate clap;
extern crate csv;
extern crate indexmap;
#[macro_use] extern crate serde_json;
extern crate serde;

use clap::Parser;
use encoding_rs_io::DecodeReaderBytesBuilder;
use indexmap::IndexMap;
use serde_json::ser::PrettyFormatter;
use serde_json::Serializer;
use serde_json::Value as JsonValue;
use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::prelude::*;

const TEMPLATE: &str = "\
{bin} {version}
{author}

{about}

USAGE: {usage}

ARGS:
{positionals}

OPTIONS:
{options}";

const ABOUT: &str = "\
csv2jsonl (c2j) converts CSV to JSON Lines. 
By default csv2json will stream and perform type interferance without information loss.
Number strings are only converted to numbers if they are equal to the JSON string
(not contain e, E or a long floating point value). If a UTF-8 or UTF-16 BOM is detected,
then an appropriate encoding is automatically detected and transcoding is performed. In
all other cases, the source of the underlying reader is passed through unchanged as if it
were UTF-8.

Project home page: https://github.com/bwbroersma/csv2jsonl/
";

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about=ABOUT, help_template=TEMPLATE)]
struct Cli {
    /// Delimiting character (single byte) of the CSV.
    #[clap(short, long, default_value = ",", display_order = 1)]
    delimiter: String,

    /// Use a tab delimiter (overrides delimiter option).
    #[clap(short, long, overrides_with = "delimiter", display_order = 2)]
    tabs: bool,

    /// The CSV file to operate on. If omitted, will accept input as piped data via STDIN.
    #[clap(parse(from_os_str))]
    file: Option<std::path::PathBuf>,

    /// Indent the output JSON this many spaces. Disabled by default.
    #[clap(short, long, display_order = 3)]
    indent: Option<usize>,

    /// Disable type inference when parsing CSV.
    /// Do not convert empty strings to null and number string to numbers.
    #[clap(short = 'I', long, display_order = 4)]
    no_inference: bool,
}

fn row_to_object(row: IndexMap<String, String>, no_inference: bool) -> IndexMap<String, JsonValue> {
    let mut items = IndexMap::new();
    row.into_iter().for_each(|(key, value)| {
        let json_value = if !no_inference && value == "" {
            JsonValue::Null
        } else if !no_inference {
            match serde_json::from_str::<JsonValue>(&value) {
                Ok(j) => if j.is_number() && (serde_json::to_string(&j).unwrap() == value) { j } else { json!(value) },
                _ => json!(value),
            }
        } else {
            json!(value)
        };
        items.insert(key, json_value);
    });
    items
}

fn row_to_jsonl(row: IndexMap<String, String>, no_inference: bool, indent: Option<usize>) -> String {
    match indent {
        None => serde_json::to_string(&row_to_object(row, no_inference)).unwrap(),
        Some(size) => {
            let spaces = " ".repeat(size);
            let writer = Vec::new();
            let formatter = PrettyFormatter::with_indent(spaces.as_bytes());
            let mut serializer = Serializer::with_formatter(writer, formatter);
            row_to_object(row, no_inference).serialize(&mut serializer).unwrap();
            String::from_utf8(serializer.into_inner()).unwrap()
        },
    }
}

fn main() {
    let cli = Cli::parse();

    let file = match cli.file {
        Some(csv_file) => Box::new(File::open(csv_file).expect("Could not read csv file")) as Box<dyn Read>,
        None => Box::new(io::stdin()) as Box<dyn Read>,
    };

    // Reads BOM and can handle UTF-8, UTF-16LE, etc.
    let transcoded = DecodeReaderBytesBuilder::new()
          .encoding(None) // Could pass Some(encoding_rs::WINDOWS_1252) here
          .build(file);

    let delimiter = if cli.tabs { b'\t' } else { cli.delimiter.as_bytes()[0] };
    let mut csv_reader = csv::ReaderBuilder::new().delimiter(delimiter).from_reader(transcoded);
    
    for result in csv_reader.deserialize() {
        let row: IndexMap<String, String> = result.expect("CSV line read error");
        println!("{}", row_to_jsonl(row, cli.no_inference, cli.indent));
    }
}