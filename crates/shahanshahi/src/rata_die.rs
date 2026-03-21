//! **Rata Die** (integer day count) for **proleptic Gregorian** dates: `RD 1` = Monday **1 January 1** CE.
//! Internal conversions use Julian day number `J = RD + 1_721_425` (SPEC.md anchor calendar).

/// `JDN` such that `gregorian_to_jdn(1, 1, 1) == 1_721_426` and `RD = JDN - JDN_RD_OFFSET` yields `RD == 1`.
const JDN_RD_OFFSET: i64 = 1_721_425;

/// Proleptic Gregorian civil date → Rata Die (same epoch as above).
#[inline]
pub(crate) fn gregorian_to_rata_die(year: i32, month: u8, day: u8) -> i64 {
    gregorian_to_jdn(year, month, day) - JDN_RD_OFFSET
}

#[inline]
pub(crate) fn rata_die_to_gregorian(rd: i64) -> (i32, u8, u8) {
    jdn_to_gregorian(rd + JDN_RD_OFFSET)
}

fn gregorian_to_jdn(year: i32, month: u8, day: u8) -> i64 {
    // Integer Gregorian→JDN: shift to a March-based month index so February is last (`shifted_month`).
    let greg_year = i64::from(year);
    let greg_month = i64::from(month);
    let greg_day = i64::from(day);
    let march_offset = (14 - greg_month) / 12;
    let shifted_year = greg_year + 4800 - march_offset;
    let shifted_month = greg_month + 12 * march_offset - 3;
    greg_day + (153 * shifted_month + 2) / 5 + 365 * shifted_year + shifted_year / 4
        - shifted_year / 100
        + shifted_year / 400
        - 32045
}

/// Inverse of [`gregorian_to_jdn`], for positive JDN in the usual software range.
///
/// This is the standard decomposition (often cited from Fliegel & Van Flandern / calendar FAQs):
/// peel **400-year**, then **century**, then **4-year** blocks from a shifted day count, then map the
/// remaining “day-of-year” chunk to month/day with the classic `153`-term identity.
fn jdn_to_gregorian(mut jdn: i64) -> (i32, u8, u8) {
    jdn += 32044;

    let cycles_400y = jdn / 146097;
    let day_in_400y_cycle = jdn % 146097;

    let century_leap_correction = ((day_in_400y_cycle / 36524 + 1) * 3) / 4;
    let day_after_century = day_in_400y_cycle - century_leap_correction * 36524;

    let quadrennia_4y = day_after_century / 1461;
    let day_in_quadrennium = day_after_century % 1461;

    let year_leap_correction = ((day_in_quadrennium / 365 + 1) * 3) / 4;
    let day_within_year = day_in_quadrennium - year_leap_correction * 365;

    let year_packed = cycles_400y * 400
        + century_leap_correction * 100
        + quadrennia_4y * 4
        + year_leap_correction;

    // `month_packed` is not a civil month yet; the next two lines are the standard month/day recovery.
    let month_packed = (5 * day_within_year + 308) / 153 - 2;
    let day_of_month = day_within_year - (153 * month_packed + 2) / 5 + 1;

    let year = (year_packed - 4800 + (month_packed + 2) / 12) as i32;
    let month = ((month_packed + 2).rem_euclid(12) + 1) as u8;
    let day = day_of_month as u8;
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_rd_matches_gregorian_1976_03_21() {
        let rd = gregorian_to_rata_die(1976, 3, 21);
        assert_eq!(rd, 721_434);
        assert_eq!(rata_die_to_gregorian(rd), (1976, 3, 21));
    }

    #[test]
    fn rd_one_is_jan_1_year_1() {
        assert_eq!(gregorian_to_rata_die(1, 1, 1), 1);
        assert_eq!(rata_die_to_gregorian(1), (1, 1, 1));
    }
}
