use std::str::FromStr;

use serde_json::Value;

use xilem::{WindowOptions, Xilem, core::View};

fn main() {
    let samplejson = include_str!("../sample-json.json");
    let value = Value::from_str(samplejson).expect("parse json");

    Xilem::new_simple(value, logic, WindowOptions::new("json viewer"));
}

impl View for Value {}

fn logic(value: &mut Value) {}
