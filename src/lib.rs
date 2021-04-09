extern crate cpc;
use crate::conversions::d128_to_radix_string;
use crate::fractions::float_to_fraction_string;
use cpc::eval;
use cpc::units::Unit;
use decimal::*;
use wasm_bindgen::prelude::*;

mod conversions;
mod fractions;
mod tests;

#[wasm_bindgen]
pub struct NumOutput {
    num: String,
    unit: String,
    radix: u8,
}

impl NumOutput {
    fn new(num: String, unit: String, radix: u8) -> Self {
        NumOutput {
            num: num,
            unit: unit,
            radix: radix,
        }
    }
}

impl ToString for NumOutput {
    fn to_string(&self) -> String {
        format!("{}; {}; {:?}", self.num, self.unit, self.radix)
    }
}

#[wasm_bindgen]
pub fn eval_expression_radix(expr: String, radix: u8) -> NumOutput {
    match eval(expr.as_str(), false, Unit::Celsius, false) {
        Ok(answer) => NumOutput::new(
            d128_to_radix_string(answer.value, radix),
            matched_unit(answer.unit),
            radix,
        ),
        Err(_) => NumOutput::new("".to_string(), "".to_string(), radix),
    }
}

#[wasm_bindgen]
pub fn eval_as_fraction_radix(expr: String, radix: u8, precision: i32) -> NumOutput {
    match eval(expr.as_str(), false, Unit::Celsius, false) {
        Ok(answer) => NumOutput::new(
            float_to_fraction_string(answer.value, radix, precision),
            "fraction".to_string(),
            radix,
        ),
        Err(_) => NumOutput::new("".to_string(), "".to_string(), radix),
    }
}

#[wasm_bindgen]
pub fn eval_expression(expr: String) -> NumOutput {
    match eval(expr.as_str(), false, Unit::Celsius, false) {
        Ok(answer) => NumOutput::new(answer.value.to_string(), matched_unit(answer.unit), 10),
        Err(_) => NumOutput::new("".to_string(), "".to_string(), 10),
    }
}

pub fn eval_input(expr: String) -> (d128, String) {
    match eval(expr.as_str(), false, Unit::Celsius, false) {
        Ok(answer) => (answer.value, matched_unit(answer.unit)),
        Err(_) => (d128!(0), "".to_string()),
    }
}

fn matched_unit(unit: Unit) -> String {
    match unit {
        Unit::Foot => "ft".to_string(),
        Unit::Mile => "mi".to_string(),
        Unit::Kilometer => "km".to_string(),
        Unit::Centimeter => "cm".to_string(),
        Unit::Millimeter => "mm".to_string(),
        Unit::Microsecond => "ms".to_string(),
        Unit::Celsius => "ºC".to_string(),
        Unit::Kelvin => "ºK".to_string(),
        Unit::Fahrenheit => "ºF".to_string(),
        Unit::Pound => "lb".to_string(),
        Unit::Ounce => "oz".to_string(),
        Unit::Inch => "in".to_string(),
        _ => unit_to_string(unit),
    }
}

fn unit_to_string(unit: Unit) -> String {
    format!("{:?}", unit).to_lowercase()
}
