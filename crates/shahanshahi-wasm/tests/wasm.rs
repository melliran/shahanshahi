use js_sys::Reflect;
use shahanshahi_wasm::{
    gregorian_to_shahanshahi, is_shahanshahi_leap_year, shahanshahi_to_gregorian,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

fn get_i32(obj: &JsValue, key: &str) -> i32 {
    Reflect::get(obj, &JsValue::from_str(key))
        .unwrap()
        .as_f64()
        .unwrap() as i32
}

fn get_u8(obj: &JsValue, key: &str) -> u8 {
    Reflect::get(obj, &JsValue::from_str(key))
        .unwrap()
        .as_f64()
        .unwrap() as u8
}

// Anchor: 1 Farvardin 2535 SH == 1976-03-21 Gregorian (from SPEC.md and golden corpus)

#[wasm_bindgen_test]
fn gregorian_to_shahanshahi_anchor() {
    let result = gregorian_to_shahanshahi(1976, 3, 21).unwrap();
    assert_eq!(get_i32(&result, "year"), 2535);
    assert_eq!(get_u8(&result, "month"), 1);
    assert_eq!(get_u8(&result, "day"), 1);
}

#[wasm_bindgen_test]
fn shahanshahi_to_gregorian_anchor() {
    let result = shahanshahi_to_gregorian(2535, 1, 1).unwrap();
    assert_eq!(get_i32(&result, "year"), 1976);
    assert_eq!(get_u8(&result, "month"), 3);
    assert_eq!(get_u8(&result, "day"), 21);
}

#[wasm_bindgen_test]
fn leap_year_known_values() {
    // 2534: (2534 - 1180) = 1354; 1354 mod 33 = 1 → leap
    assert!(is_shahanshahi_leap_year(2534));
    // 2535: (2535 - 1180) = 1355; 1355 mod 33 = 2 → not leap
    assert!(!is_shahanshahi_leap_year(2535));
}

#[wasm_bindgen_test]
fn invalid_gregorian_month_throws() {
    assert!(gregorian_to_shahanshahi(1976, 13, 1).is_err());
}

#[wasm_bindgen_test]
fn invalid_shahanshahi_month_throws() {
    assert!(shahanshahi_to_gregorian(2535, 13, 1).is_err());
}
