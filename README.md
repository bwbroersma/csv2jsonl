# csv2jsonl

csv2jsonl (c2j) converts CSV to JSON Lines.

By default csv2json will stream and perform type interferance without information loss.
Number strings are only converted to numbers if they are equal to the JSON string
(not contain e, E or a long floating point value). If a UTF-8 or UTF-16 BOM is detected,
then an appropriate encoding is automatically detected and transcoding is performed. In
all other cases, the source of the underlying reader is passed through unchanged as if it
were UTF-8.

## Install

The binary name for csv2jsonl is `c2j`.
Currently csv2jsonl can only be installed via cargo, first [install rust](https://www.rust-lang.org/tools/install), then:
```bash
$ cargo install csv2jsonl
```

## Usage

```bash
$ c2j [OPTIONS] [FILE]
```

```
ARGS:
    <FILE>    The CSV file to operate on. If omitted, will accept input as piped data via STDIN

OPTIONS:
    -d, --delimiter <DELIMITER>    Delimiting character (single byte) of the CSV [default: ,]
    -t, --tabs                     Use a tab delimiter (overrides delimiter option)
    -i, --indent <INDENT>          Indent the output JSON this many spaces. Disabled by default
    -I, --no-inference             Disable type inference when parsing CSV. Do not convert empty
                                   strings to null and number string to numbers
    -h, --help                     Print help information
    -V, --version                  Print version information
```

## Sample

```bash
$ c2j <<EOF
sample,value
1,string
2,
3,123
4,0123
5,1E
6,1E6
7,100000000000000000000
EOF
```

outputs:

```json
{"sample":1,"value":"string"}
{"sample":2,"value":null}
{"sample":3,"value":123}
{"sample":4,"value":"0123"}
{"sample":5,"value":"1E"}
{"sample":6,"value":"1E6"}
{"sample":7,"value":"100000000000000000000"}
```

## ToDo

- [ ] implement sniff with csv-sniffer
- [ ] add encoding arguments (e.g. CP-1252, ISO 8859-1/latin1, etc.)
- [ ] add tests
- [ ] add workflow / ci build (release)
- [ ] option to set column type
- [ ] do not convert 9007199254740993
- [ ] infer column types using csv-sniffer

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/bwbroersma/csv2jsonl/blob/main/LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/bwbroersma/csv2jsonl/blob/main/LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Thanks to/Inspiration from

I started looking for a fast rust version of [csvjson](https://csvkit.readthedocs.io/en/latest/scripts/csvjson.html) of the csvkit. I initially found [csv_to_json](https://github.com/divyekapoor/csv_to_json/), streaming but not properly reading csv. And [csv2json](https://github.com/apolitical/csv2json/) which was using [csv](https://github.com/BurntSushi/rust-csv), but no stream. After altering the package a lot, I thought it would be best to create a new one instead of a fork.
