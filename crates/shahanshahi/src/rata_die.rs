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
    let y = i64::from(year);
    let m = i64::from(month);
    let d = i64::from(day);
    let a = (14 - m) / 12;
    let y2 = y + 4800 - a;
    let m2 = m + 12 * a - 3;
    d + (153 * m2 + 2) / 5 + 365 * y2 + y2 / 4 - y2 / 100 + y2 / 400 - 32045
}

/// Inverse of [`gregorian_to_jdn`], for positive JDN in the usual software range.
fn jdn_to_gregorian(mut j: i64) -> (i32, u8, u8) {
    j += 32044;
    let g = j / 146097;
    let dg = j % 146097;
    let c = ((dg / 36524 + 1) * 3) / 4;
    let dc = dg - c * 36524;
    let b = dc / 1461;
    let db = dc % 1461;
    let a = ((db / 365 + 1) * 3) / 4;
    let da = db - a * 365;
    let y = g * 400 + c * 100 + b * 4 + a;
    let m = (5 * da + 308) / 153 - 2;
    let d = da - (153 * m + 2) / 5 + 1;
    let year = (y - 4800 + (m + 2) / 12) as i32;
    let month = ((m + 2).rem_euclid(12) + 1) as u8;
    let day = d as u8;
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
