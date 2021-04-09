use decimal::*;
use num::bigint::BigInt;
use num::pow;
use num::FromPrimitive;
use std::char;
use std::str::FromStr;

#[derive(Debug)]
pub struct NumberTemplate {
  is_negative: bool,
  float: d128,
  integer: i128,
  decimals: BigInt,
  scale: u32,
}

impl NumberTemplate {
  pub fn new(num: d128) -> Self {
    let is_neg = num.is_negative();
    let (whole, diff) = if is_neg {
      ceil_d128(num)
    } else {
      floor_d128(num)
    };
    let size: u32 = d128_to_u32(whole.logb()) + 1;
    let ref_scale = 34 - size + diff.digits() as u32;
    let scale = if ref_scale > 32 { 32 } else { ref_scale };
    NumberTemplate {
      is_negative: is_neg,
      float: num,
      integer: d128_to_i128(whole),
      decimals: d128_to_big_number(diff, scale),
      scale: scale,
    }
  }
}

impl ToString for NumberTemplate {
  fn to_string(&self) -> String {
    let neg_str = if self.is_negative { "-" } else { "" };
    let dec_as_str = self.decimals.to_string();
    let dec_str = dec_as_str.trim_end_matches('0');
    let sep = if dec_str.len() > 0 { "." } else { "" };
    return format!("{}{}{}{}", neg_str, self.integer, sep, dec_str);
  }
}

pub fn d128_to_i128(num: d128) -> i128 {
  match i128::from_str(&first_d128_part(num)) {
    Ok(result) => result,
    Err(_) => 0i128,
  }
}

pub fn d128_to_u32(num: d128) -> u32 {
  match u32::from_str(&first_d128_part(num)) {
    Ok(result) => result,
    Err(_) => 0u32,
  }
}

pub fn f64_to_d128(num: f64) -> d128 {
  if let Ok(result) = d128::from_str(&num.to_string()) {
    result
  } else {
    d128!(0)
  }
}

pub fn d128_to_big_int(num: d128) -> BigInt {
  match BigInt::from_str(&first_d128_part(num)) {
    Ok(result) => result,
    Err(_) => BigInt::from_i32(0i32).unwrap(),
  }
}

pub fn big_int_to_d128(num: BigInt) -> d128 {
  match d128::from_str(&num.to_string()) {
    Ok(result) => result,
    Err(_) => d128!(0),
  }
}

pub fn d128_to_big_number(num: d128, scale: u32) -> BigInt {
  let base = num * f64_to_d128(10f64.powf(scale as f64));
  println!("base: {}, {}", base, scale);
  let big_base = d128_to_big_int(base);
  println!("bb: {},{}", num, base);
  big_base * pow(BigInt::from_i32(10).unwrap(), scale as usize)
}

pub fn d128_to_parts(num: d128) -> (bool, String, String) {
  let is_neg = num.is_negative();
  let (whole, diff) = if is_neg {
    ceil_d128(num)
  } else {
    floor_d128(num)
  };
  (is_neg, first_d128_part(whole.abs()), last_d128_part(diff))
}

fn split_d128_string(num: d128, is_last: bool) -> String {
  let index = if is_last { 1 } else { 0 };
  let parts: Vec<String> = num.to_string().split(".").map(|s| s.to_string()).collect();
  if let Some(sub_str) = parts.get(index) {
    sub_str.to_string()
  } else {
    "".to_string()
  }
}

pub fn first_d128_part(num: d128) -> String {
  split_d128_string(num, false)
}

pub fn last_d128_part(num: d128) -> String {
  split_d128_string(num, true)
}

pub fn floor_d128_to_string(num: d128) -> String {
  let (floor_num, _) = floor_d128(num);
  first_d128_part(floor_num)
}

pub fn ceil_d128_to_string(num: d128) -> String {
  let (ceil_num, _) = ceil_d128(num);
  first_d128_part(ceil_num)
}

pub fn d128_to_radix_string(num: d128, radix: u8) -> String {
  let template = NumberTemplate::new(num);
  let radval = integer_to_radix_string(template.integer, radix, template.is_negative);
  let rad_int_len = radval.len();
  let rad_float = float_to_radix_string(template.float, radix, false);

  let rad_parts = rad_float.split_at(rad_int_len);
  let pv_str = if rad_parts.1.len() > 0 {
    if radix <= 36 {
      rad_parts.1.trim_end_matches('0').to_string()
    } else if rad_parts.1.to_string().starts_with(":") {
      clean_radix_pv_string(rad_parts.1.to_string())
    } else {
      rad_parts.1.to_string()
    }
  } else {
    "".to_string()
  };
  let place_value_str = if radix > 36 {
    pv_str[1..].to_string()
  } else {
    pv_str
  };
  let suffix = if place_value_str.len() > 0 {
    format!(".{}", place_value_str)
  } else {
    "".to_string()
  };
  format!("{}{}", rad_parts.0, suffix)
}

pub fn bigint_to_radix_string(num: BigInt, radix: u8, is_negative: bool) -> String {
  let (_, vec_nums) = num.to_radix_be(radix as u32);
  let mut num_chars: Vec<String> = vec_nums
    .iter()
    .map(|c| integer_to_radix_char(*c, radix))
    .collect();
  if is_negative {
    num_chars.insert(0, "-".to_string());
  }
  let separator: &str = if radix > 36 { ":" } else { "" };
  num_chars.join(separator)
}

pub fn float_to_radix_string(num: d128, radix: u8, is_neg: bool) -> String {
  let max_len_10 = 34 - d128_to_u32(num.logb());
  let radix_log: f64 = if radix < 10 {
    0.25
  } else if radix < 12 {
    0.75
  } else {
    0.5
  };
  let rel_scale = (1f64 / (radix as f64 / 10f64).powf(radix_log) * max_len_10 as f64) as usize;
  println!(
    "rel scale: {}, {}, {}",
    rel_scale,
    radix,
    d128_to_u32(num.logb())
  );
  let multiple = pow(BigInt::from_u8(radix).unwrap(), rel_scale as usize);
  let bg = d128_to_big_int(num * big_int_to_d128(multiple));
  bigint_to_radix_string(bg, radix, is_neg)
}

pub fn integer_to_radix_string(num: i128, radix: u8, is_negative: bool) -> String {
  let bg = BigInt::from_i128(num).unwrap();
  bigint_to_radix_string(bg, radix, is_negative)
}

pub fn integer_to_radix_char(num: u8, radix: u8) -> String {
  let mut str_val = "".to_string();
  if radix > 10 {
    let mut char_num: u8 = num;
    if radix > 36 {
      if radix >= 100 {
        let hundreds = num / 100;
        str_val.push_str(hundreds.to_string().as_str());
      }
      let tens = num / 10;
      str_val.push_str(tens.to_string().as_str());
      char_num = num - (tens * 10);
    }
    if char_num >= 10 {
      if let Some(n) = char::from_u32(87 + char_num as u32) {
        str_val.push_str(n.to_string().as_str());
      }
    } else {
      str_val.push_str(char_num.to_string().as_str());
    }
  } else {
    str_val = num.to_string();
  }
  str_val
}

pub fn clean_radix_pv_string(num_str: String) -> String {
  let mut parts: Vec<&str> = num_str.split(":").collect::<Vec<&str>>();
  let num = parts.len();
  let mut stop = false;
  for n in (0..num).rev() {
    if !stop && parts[n] == "00" {
      parts.remove(n);
    } else {
      stop = true
    }
  }
  parts.join(":")
}

fn floor_d128(num: d128) -> (d128, d128) {
  let diff = num % d128!(1);
  (num - diff, diff)
}

pub fn d128_to_f64(num: d128) -> f64 {
  if let Ok(result) = f64::from_str(&num.to_string()) {
    result
  } else {
    0f64
  }
}

pub fn i32_to_d128(num: i32) -> d128 {
  d128::from(num)
}

pub fn i128_to_d128(num: i128) -> d128 {
  match d128::from_str(&num.to_string()) {
    Ok(result) => result,
    Err(_) => d128!(0.0),
  }
}

fn ceil_d128(num: d128) -> (d128, d128) {
  let (num, diff) = floor_d128(num);
  let is_integer = diff == d128!(0);
  let ceil_offset = if num.is_negative() {
    d128!(0)
  } else {
    d128!(1)
  };
  let ceil_num = if is_integer { num } else { num + ceil_offset };
  let ceil_diff = if is_integer { diff } else { d128!(1) - diff };
  (ceil_num, ceil_diff)
}
