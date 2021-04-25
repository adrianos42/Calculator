use lazy_static::lazy_static;
use regex::Regex;

pub fn trim_trailing_zeros(value: &str) -> String {
    lazy_static! {
        static ref RE_TR: Regex = Regex::new(r"(\.?)0+$").unwrap();
    }

    if value.find('.').is_some() {
        RE_TR.replace_all(value, "").into_owned()
    } else {
        String::from(value)
    }
}

pub fn get_number_digits(value: &str) -> usize {
    let w = trim_trailing_zeros(value);
    let mut size = w.len();

    if w.find(".").is_some() {
        size -= 1;
    }

    if w.find("-").is_some() {
        size -= 1;
    }

    size
}

pub fn get_number_digits_whole_number_part(value: f64) -> usize {
    if value == 0.0 {
        1
    } else {
        let z: f64 = 0.0;
        1 + (z.max(value.abs().log10()) as usize)
    }
}