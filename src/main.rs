use std::str::FromStr;

use serde_json::Value;

fn main() {
    let samplejson = include_str!("./sample-json.json");
    let value = Value::from_str(samplejson).expect("parse json");
    println!("{}", value);
}
