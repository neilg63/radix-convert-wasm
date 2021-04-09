use crate::conversions::d128_to_f64;
use crate::conversions::d128_to_i128;
use crate::conversions::integer_to_radix_string;
use decimal::*;
use num::bigint::BigInt;
use num::rational::BigRational;
use num::FromPrimitive;
use num::ToPrimitive;

#[derive(Debug)]
pub struct RationalNumber128 {
	integer: i128,
	numer: i128,
	denom: i32,
	difference: f64,
}

impl RationalNumber128 {
	pub fn new(integer: i128, big: BigRational, difference: f64) -> Self {
		let numer = match big.numer().to_i128() {
			Some(result) => result,
			None => 0,
		};
		let denom = match big.denom().to_i32() {
			Some(result) => result,
			None => 0,
		};
		RationalNumber128 {
			integer: integer,
			numer: numer,
			denom: denom,
			difference: difference,
		}
	}

	pub fn to_string_radix(&self, radix: u8) -> String {
		let mut parts: Vec<String> = vec![];
		if self.integer > 0 {
			parts.push(integer_to_radix_string(
				self.integer,
				radix,
				self.integer < 0,
			));
		}
		if self.numer > 0 && self.denom > 0 {
			let frac_parts: Vec<String> = vec![
				integer_to_radix_string(self.numer, radix, true),
				integer_to_radix_string(self.denom as i128, radix, true),
			];
			parts.push(frac_parts.join("/"));
		}
		parts.join(" ")
	}
}

impl ToString for RationalNumber128 {
	fn to_string(&self) -> String {
		let mut parts: Vec<String> = vec![];
		if self.integer > 0 {
			parts.push(format!("{:?}", self.integer));
		}
		if self.numer > 0 && self.denom > 0 {
			parts.push(format!("{:?}/{:?}", self.numer, self.denom));
		}
		parts.join(" ")
	}
}

pub fn float_to_fraction(num: d128, precision: i32) -> RationalNumber128 {
	let is_negative = num < d128!(0.0);
	let abs_num = d128_to_f64(num.abs());
	let remainder = d128_to_f64(num.abs() % d128!(1));
	let integer = d128_to_i128(num.abs());
	let mut numer: i128 = abs_num as i128;
	let mut demon: i32 = 1;
	let max = precision + 1;
	let max_dec: f64 = 1.0_f64 / max as f64;
	let mut difference: f64 = 0.0;
	for i in 1..max {
		if let Some((n, diff)) = is_divisable(remainder, i, max_dec) {
			numer = (n as f64 * remainder).round() as i128;
			demon = n as i32;
			difference = diff;
			break;
		}
	}
	if is_negative {
		numer = 0 - numer;
	}
	RationalNumber128::new(integer, build_big_rational(numer, demon), difference)
}

pub fn float_to_fraction_string(num: d128, radix: u8, precision: i32) -> String {
	let rn = float_to_fraction(num, precision);
	if radix == 10 {
		rn.to_string()
	} else {
		rn.to_string_radix(radix)
	}
}

pub fn is_divisable(num: f64, i: i32, tolerance: f64) -> Option<(i32, f64)> {
	let diff = num * i as f64 % 1.0;
	if diff <= tolerance {
		return Some((i, diff));
	} else if diff >= (1.0 - tolerance) {
		return Some((i, 1.0 - diff));
	} else {
		return None;
	}
}

pub fn build_big_rational(numer: i128, denom: i32) -> BigRational {
	BigRational::new(build_bigint_128(numer), build_bigint(denom)).reduced()
}

pub fn build_bigint(integer: i32) -> BigInt {
	BigInt::from_i32(integer).unwrap()
}

pub fn build_bigint_128(integer: i128) -> BigInt {
	BigInt::from_i128(integer).unwrap()
}
