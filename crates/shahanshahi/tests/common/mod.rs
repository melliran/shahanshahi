//! Helpers shared by integration tests.

/// Days since 1970-01-01 for a proleptic Gregorian date (Howard Hinnant's `days_from_civil`:
/// March-based year, then era of 400 years, year-of-era, day-of-year, day-of-era).
pub fn days_from_civil(year: i32, month: u8, day: u8) -> i64 {
    let march_year = i64::from(year) - i64::from(month <= 2);
    let era = march_year.div_euclid(400);
    let year_of_era = march_year - era * 400;
    let shifted_month = (i64::from(month) + 9) % 12;
    let day_of_year = (153 * shifted_month + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}
