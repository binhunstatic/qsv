#![allow(unused_assignments)]
static USAGE: &str = r#"
Smartly converts CSV to a newline-delimited JSON (JSONL/NDJSON).

By scanning the CSV first, it "smartly" infers the appropriate JSON data type
for each column.

It will infer a column as boolean if it only has a domain of two values,
and the first character of the values are one of the following case-insensitive
combinations: t/f; t/null; 1/0; 1/null; y/n & y/null are treated as true/false.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_tojsonl.rs.

Usage:
    qsv tojsonl [options] [<input>]
    qsv tojsonl --help

Tojsonl optionns:
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.

Common options:
    -h, --help             Display this message
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -o, --output <file>    Write output to <file> instead of stdout.
"#;

use std::{env::temp_dir, fmt::Write, fs::File, path::Path, str::FromStr};

use serde::Deserialize;
use serde_json::{Map, Value};
use strum_macros::EnumString;
use uuid::Uuid;

use super::schema::infer_schema_from_stats;
use crate::{
    config::{Config, Delimiter},
    util, CliError, CliResult,
};

#[derive(Deserialize, Clone)]
struct Args {
    arg_input:      Option<String>,
    flag_jobs:      Option<usize>,
    flag_delimiter: Option<Delimiter>,
    flag_output:    Option<String>,
}

impl From<std::fmt::Error> for CliError {
    fn from(err: std::fmt::Error) -> CliError {
        CliError::Other(err.to_string())
    }
}

#[derive(PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
enum JsonlType {
    Boolean,
    String,
    Number,
    Integer,
    Null,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let preargs: Args = util::get_args(USAGE, argv)?;
    let mut args = preargs.clone();
    let conf = Config::new(&args.arg_input).delimiter(args.flag_delimiter);
    let mut is_stdin = false;

    let stdin_fpath = format!("{}/{}.csv", temp_dir().to_string_lossy(), Uuid::new_v4());
    let stdin_temp = stdin_fpath.clone();

    // if using stdin, we create a stdin.csv file as stdin is not seekable and we need to
    // open the file multiple times to compile stats/unique values, etc.
    let input_filename = if preargs.arg_input.is_none() {
        let mut stdin_file = File::create(stdin_fpath.clone())?;
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        std::io::copy(&mut stdin_handle, &mut stdin_file)?;
        args.arg_input = Some(stdin_fpath.clone());
        is_stdin = true;
        stdin_fpath
    } else {
        let filename = Path::new(args.arg_input.as_ref().unwrap())
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        filename
    };
    // we're calling the schema command to infer data types and enums
    let schema_args = crate::cmd::schema::Args {
        // we only do three, as we're only inferring boolean based on enum
        // i.e. we only inspect a field if its boolean if its domain
        // is just two values. if its more than 2, that's all we need know
        // for boolean inferencing
        flag_enum_threshold:  3,
        flag_strict_dates:    false,
        flag_pattern_columns: crate::select::SelectColumns::parse("")?,
        // json doesn't have a date type, so don't infer dates
        flag_dates_whitelist: "none".to_string(),
        flag_prefer_dmy:      false,
        flag_stdout:          false,
        flag_jobs:            Some(util::njobs(args.flag_jobs)),
        flag_no_headers:      false,
        flag_delimiter:       args.flag_delimiter,
        arg_input:            args.arg_input.clone(),
    };
    // build schema for each field by their inferred type, min/max value/length, and unique values
    let properties_map: Map<String, Value> =
        match infer_schema_from_stats(&schema_args, &input_filename) {
            Ok(map) => map,
            Err(e) => {
                return fail_clierror!("Failed to infer field types: {e}");
            }
        };

    let mut rdr = if is_stdin {
        Config::new(&Some(stdin_temp))
            .delimiter(args.flag_delimiter)
            .reader()?
    } else {
        conf.reader()?
    };

    // TODO: instead of abusing csv writer to write jsonl file
    // just use a normal buffered writer
    let mut wtr = Config::new(&args.flag_output)
        .flexible(true)
        .no_headers(true)
        .quote_style(csv::QuoteStyle::Never)
        .writer()?;

    let headers = rdr.headers()?.clone();

    // create a vec lookup about inferred field data types
    let mut field_type_vec: Vec<JsonlType> = Vec::with_capacity(headers.len());
    for (_field_name, field_def) in properties_map.iter() {
        let Some(field_map) = field_def.as_object() else { return fail!("Cannot create field map") };
        let prelim_type = field_map.get("type").unwrap();
        let field_values_enum = field_map.get("enum");

        // log::debug!("prelim_type: {prelim_type} field_values_enum: {field_values_enum:?}");

        // check if a field has a boolean data type
        // by checking its enum constraint
        if let Some(domain) = field_values_enum {
            if let Some(vals) = domain.as_array() {
                // if this field only has a domain of two values
                if vals.len() == 2 {
                    let val1 = if vals[0].is_null() {
                        '_'
                    } else {
                        // check the first domain value, if its a string
                        // get the first character of val1 lowercase
                        if let Some(str_val) = vals[0].as_str() {
                            first_lower_char(str_val)
                        } else if let Some(int_val) = vals[0].as_u64() {
                            // else, its an integer (as we only do enum constraints
                            // for string and integers), and see if its 1 or 0
                            match int_val {
                                1 => '1',
                                0 => '0',
                                _ => '*', // its something else
                            }
                        } else {
                            '*'
                        }
                    };
                    // same as above, but for the 2nd domain value
                    let val2 = if vals[1].is_null() {
                        '_'
                    } else if let Some(str_val) = vals[1].as_str() {
                        first_lower_char(str_val)
                    } else if let Some(int_val) = vals[1].as_u64() {
                        match int_val {
                            1 => '1',
                            0 => '0',
                            _ => '*',
                        }
                    } else {
                        '*'
                    };
                    // log::debug!("val1: {val1} val2: {val2}");

                    // check if the domain of two values is truthy or falsy
                    // i.e. if first character, case-insensitive is "t", "1" or "y" - truthy
                    // "f", "0", "n" or null - falsy
                    // if it is, infer a boolean field
                    if let ('t', 'f' | '_')
                    | ('f' | '_', 't')
                    | ('1', '0' | '_')
                    | ('0' | '_', '1')
                    | ('y', 'n' | '_')
                    | ('n' | '_', 'y') = (val1, val2)
                    {
                        field_type_vec.push(JsonlType::Boolean);
                        continue;
                    }
                }
            }
        }

        // ok to use index access and unwrap here as we know
        // we have at least one element in the prelim_type as_array
        field_type_vec.push(
            JsonlType::from_str(
                prelim_type.as_array().unwrap()[0]
                    .as_str()
                    .unwrap_or("null"),
            )
            .unwrap_or(JsonlType::String),
        );
    }

    // amortize allocs
    let mut record = csv::StringRecord::new();

    let mut temp_string = String::with_capacity(100);
    let mut temp_string2 = String::with_capacity(50);

    let mut header_key = Value::String(String::with_capacity(50));
    let mut temp_val = Value::String(String::with_capacity(50));

    // TODO: see if its worth it to do rayon here after benchmarking
    // with large files. We have --jobs option, but we only pass it
    // thru to stats/frequency to infer data types & enum constraints.

    // now that we have type mappings, iterate thru input csv
    // and write jsonl file
    while rdr.read_record(&mut record)? {
        temp_string.clear();
        record.trim();
        write!(temp_string, "{{")?;
        for (idx, field) in record.iter().enumerate() {
            let field_val = if let Some(field_type) = field_type_vec.get(idx) {
                match field_type {
                    JsonlType::String => {
                        if field.is_empty() {
                            "null"
                        } else {
                            // we round-trip thru serde_json to escape the str
                            // per json spec (https://www.json.org/json-en.html)
                            temp_val = field.into();
                            temp_string2 = temp_val.to_string();
                            &temp_string2
                        }
                    }
                    JsonlType::Null => "null",
                    JsonlType::Integer | JsonlType::Number => field,
                    JsonlType::Boolean => {
                        if let 't' | 'y' | '1' = first_lower_char(field) {
                            "true"
                        } else {
                            "false"
                        }
                    }
                }
            } else {
                "null"
            };
            header_key = headers[idx].into();
            if field_val.is_empty() {
                write!(temp_string, r#"{header_key}:null,"#)?;
            } else {
                write!(temp_string, r#"{header_key}:{field_val},"#)?;
            }
        }
        temp_string.pop(); // remove last comma
        temp_string.push('}');
        record.clear();
        record.push_field(&temp_string);
        wtr.write_record(&record)?;
    }

    Ok(wtr.flush()?)
}

#[inline]
fn first_lower_char(field_str: &str) -> char {
    field_str.chars().next().unwrap_or('_').to_ascii_lowercase()
}
