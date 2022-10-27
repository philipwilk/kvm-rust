use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env::{args, Args};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone, Hash)]
/*
  Make value set to 0 for boolean parameters (no value) and 1 for keys with values
*/
pub enum Parameters {
    NoPreflights = 0,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone, Hash)]
enum KeyFormat {
    Equals,
    Space,
    Boolean,
}

// tryfrom implementation, almost definitely not mine iirc
impl FromStr for Parameters {
    type Err = ();

    fn from_str(input: &str) -> Result<Parameters, Self::Err> {
        match input {
            "NoPreflights" => Ok(Parameters::NoPreflights),
            _ => Err(()),
        }
    }
}

pub async fn get_parameters() -> HashMap<Parameters, String> {
    let _parameters: Result<HashMap<Parameters, String>, String> = parse_parameters(args()).await;
    let parameters: HashMap<Parameters, String>;
    if _parameters.is_err() {
        eprintln!("--- FATAL ERROR --- {}", _parameters.unwrap_err());
        std::process::exit(exitcode::DATAERR);
    } else {
        parameters = _parameters.unwrap();
    }
    parameters
}

// return vec<(Key, Val)> of parameters. key and val are same for boolean params
async fn parse_parameters(arguments: Args) -> Result<HashMap<Parameters, String>, String> {
    let mut raw_arg_vecd: VecDeque<String> = VecDeque::new();
    for arg in arguments {
        raw_arg_vecd.push_back(arg);
    }

    // args[0] is usually the path of the executable, if it is a file, pop it fron the front
    if Path::new(&raw_arg_vecd[0]).is_file() {
        raw_arg_vecd.pop_front();
    }

    /*
      Formats I want to work
        --key=value
        --key value
        -key
        command parameter --key value -key
    */
    lazy_static! {
        static ref REGEX_EQUALS: Regex = Regex::new(r"^--[A-z]+=\S+$").unwrap();
        static ref REGEX_SPACE: Regex = Regex::new(r"^--[A-z]+$").unwrap();
        static ref REGEX_BOOLEAN: Regex = Regex::new(r"^-[A-z]+$").unwrap();
    }

    let mut args_hashmap: HashMap<Parameters, String> = HashMap::new();
    while raw_arg_vecd.len() > 0 {
        let front = raw_arg_vecd.pop_front().unwrap();
        let original_front = front.clone();
        let key_format: KeyFormat;
        // Need to isolate the key from the value (if a value is present)
        // check if --key=value format
        if REGEX_EQUALS.is_match(&front) {
            key_format = KeyFormat::Equals;
        }
        // Check if --key format
        else if REGEX_SPACE.is_match(&front) {
            key_format = KeyFormat::Space;
        }
        // Check if -key format
        else if REGEX_BOOLEAN.is_match(&front) {
            key_format = KeyFormat::Boolean;
        } else {
            return Err("Invalid parameter format ".to_owned() + &front);
        }
        let mut equals_val: String;
        // Check key is real parameter
        let parsed_front_key: String = (if key_format == KeyFormat::Equals {
            let vals = front.split("=").collect::<Vec<&str>>();
            #[allow(unused_assignments)]
            equals_val = vals[1].to_owned();
            vals[0].clone().to_owned()
        } else {
            front
        })
        .replace("-", "");

        let front_key_res = Parameters::from_str(&parsed_front_key);
        let front_key: Parameters;
        match front_key_res {
            Ok(_) => front_key = front_key_res.unwrap(),
            Err(_) => return Err("Invalid parameter: ".to_owned() + &original_front),
        }

        if key_format == KeyFormat::Space && raw_arg_vecd.len() < 1 {
            return Err("Space formatted parameter without value: ".to_owned() + &original_front);
        } else if key_format == KeyFormat::Space {
            equals_val = raw_arg_vecd.pop_front().unwrap();
        } else {
            equals_val = "".to_owned();
        }

        if key_format == KeyFormat::Boolean && front_key as u8 == 1 {
            return Err(
                "Use of key+value parameter as boolean parameter: ".to_owned() + &original_front,
            );
        }
        if key_format != KeyFormat::Boolean && front_key as u8 == 0 {
            return Err(
                "Use of boolean parameter as a valued parameter: ".to_owned() + &original_front,
            );
        }

        match key_format {
            KeyFormat::Boolean => args_hashmap.insert(front_key, parsed_front_key),
            KeyFormat::Equals | KeyFormat::Space => args_hashmap.insert(front_key, equals_val),
        };
    }

    Ok(args_hashmap)
}
