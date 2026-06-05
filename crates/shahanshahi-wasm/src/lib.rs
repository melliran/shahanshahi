use serde::Serialize;
use shahanshahi::{is_shahanshahi_leap_arithmetic, GregorianDate, ShahanshahiDate};
use wasm_bindgen::prelude::*;

/// Plain data returned across the JS boundary — serializes to `{ year, month, day }`.
#[derive(Serialize)]
struct DateResult {
    year: i32,
    month: u8,
    day: u8,
}

/// Convert a Gregorian date to Shahanshahi.
///
/// Returns a JS object `{ year, month, day }` or throws a JS `Error` on invalid input.
#[wasm_bindgen(js_name = gregorianToShahanshahi)]
pub fn gregorian_to_shahanshahi(year: i32, month: u8, day: u8) -> Result<JsValue, JsError> {
    let greg =
        GregorianDate::try_new(year, month, day).map_err(|e| JsError::new(&e.to_string()))?;
    let sh = ShahanshahiDate::try_from_gregorian(greg).map_err(|e| JsError::new(&e.to_string()))?;
    let result = DateResult {
        year: sh.year(),
        month: sh.month(),
        day: sh.day(),
    };
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

/// Convert a Shahanshahi date to Gregorian.
///
/// Returns a JS object `{ year, month, day }` or throws a JS `Error` on invalid input.
#[wasm_bindgen(js_name = shahanshahiToGregorian)]
pub fn shahanshahi_to_gregorian(year: i32, month: u8, day: u8) -> Result<JsValue, JsError> {
    let sh =
        ShahanshahiDate::try_new(year, month, day).map_err(|e| JsError::new(&e.to_string()))?;
    let greg = sh.to_gregorian();
    let result = DateResult {
        year: greg.year(),
        month: greg.month(),
        day: greg.day(),
    };
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

/// Return `true` if the given Shahanshahi year is a leap year (Mode A arithmetic rule).
#[wasm_bindgen(js_name = isShahanshahiLeapYear)]
pub fn is_shahanshahi_leap_year(year: i32) -> bool {
    is_shahanshahi_leap_arithmetic(year)
}
