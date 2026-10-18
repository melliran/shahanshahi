use serde::Serialize;
use shahanshahi::{is_shahanshahi_leap_arithmetic, GregorianDate, Locale, ShahanshahiDate};
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

/// Format a Shahanshahi date using a strftime-style format string.
///
/// Specifiers: `%Y` `%m` `%d` `%B` `%b` `%A` `%%`; unknown specifiers pass through unchanged.
/// `locale` is `"en"` (default — romanized names, Western digits) or `"fa"` (Persian names
/// and digits). Throws a JS `Error` on an invalid date or unknown locale.
#[wasm_bindgen(js_name = formatShahanshahiDate)]
pub fn format_shahanshahi_date(
    year: i32,
    month: u8,
    day: u8,
    fmt: &str,
    locale: Option<String>,
) -> Result<String, JsError> {
    let locale = match locale.as_deref() {
        None | Some("en") => Locale::En,
        Some("fa") => Locale::Fa,
        Some(other) => {
            return Err(JsError::new(&format!(
                "unknown locale {other:?} (expected \"en\" or \"fa\")"
            )))
        }
    };
    let sh =
        ShahanshahiDate::try_new(year, month, day).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(sh.format_localized(fmt, locale))
}

/// Replace every ASCII digit in `s` with its Persian numeral (U+06F0–U+06F9).
#[wasm_bindgen(js_name = toPersianNumerals)]
pub fn to_persian_numerals(s: &str) -> String {
    shahanshahi::to_persian_numerals(s)
}
