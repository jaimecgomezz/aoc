// cargo-deps: json = "0.11.13"

extern crate json;

use json::object::Object;
use json::JsonValue;
use std::fs;

pub fn main() {
    println!("{}", sum(parse("input.txt")));
}

fn parse(path: &str) -> JsonValue {
    let content = fs::read_to_string(path).unwrap();

    json::parse(content.as_str()).unwrap()
}

fn sum(item: JsonValue) -> isize {
    match item {
        JsonValue::Number(n) => n.into(),
        JsonValue::Array(array) => sum_array(array),
        JsonValue::Object(object) => sum_object(object),
        _ => 0,
    }
}

fn sum_object(object: Object) -> isize {
    let mut result: isize = 0;

    let mut red_found = false;

    for (_, value) in object.iter() {
        match value {
            JsonValue::Short(string) => {
                if string == "red" {
                    red_found = true;
                    break;
                }
            }
            _ => {
                result += sum(value.to_owned());
            }
        }
    }

    if red_found {
        return 0;
    }

    return result;
}

fn sum_array(array: Vec<JsonValue>) -> isize {
    array.into_iter().map(|a| sum(a)).sum()
}
