//! Persian month names — ASCII romanization and Persian script.

const NAMES_ASCII: [&str; 12] = [
    "Farvardin",
    "Ordibehesht",
    "Khordad",
    "Tir",
    "Mordad",
    "Shahrivar",
    "Mehr",
    "Aban",
    "Azar",
    "Dey",
    "Bahman",
    "Esfand",
];

const NAMES_PERSIAN: [&str; 12] = [
    "فروردین",
    "اردیبهشت",
    "خرداد",
    "تیر",
    "مرداد",
    "شهریور",
    "مهر",
    "آبان",
    "آذر",
    "دی",
    "بهمن",
    "اسفند",
];

/// Returns the ASCII romanization of the Shahanshahi month name for a
/// 1-based month index (1 = `"Farvardin"` … 12 = `"Esfand"`), or `None`
/// for any value outside `1..=12`.
pub fn shahanshahi_month_name(month: u8) -> Option<&'static str> {
    match month {
        1..=12 => Some(NAMES_ASCII[(month - 1) as usize]),
        _ => None,
    }
}

/// Returns the Persian script name of the Shahanshahi month for a
/// 1-based month index (1 = `"فروردین"` … 12 = `"اسفند"`), or `None`
/// for any value outside `1..=12`.
pub fn shahanshahi_month_name_persian(month: u8) -> Option<&'static str> {
    match month {
        1..=12 => Some(NAMES_PERSIAN[(month - 1) as usize]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_all_twelve() {
        let expected = [
            "Farvardin",
            "Ordibehesht",
            "Khordad",
            "Tir",
            "Mordad",
            "Shahrivar",
            "Mehr",
            "Aban",
            "Azar",
            "Dey",
            "Bahman",
            "Esfand",
        ];
        for (i, name) in expected.iter().enumerate() {
            assert_eq!(shahanshahi_month_name(i as u8 + 1), Some(*name));
        }
    }

    #[test]
    fn persian_all_twelve() {
        let expected = [
            "فروردین",
            "اردیبهشت",
            "خرداد",
            "تیر",
            "مرداد",
            "شهریور",
            "مهر",
            "آبان",
            "آذر",
            "دی",
            "بهمن",
            "اسفند",
        ];
        for (i, name) in expected.iter().enumerate() {
            assert_eq!(shahanshahi_month_name_persian(i as u8 + 1), Some(*name));
        }
    }

    #[test]
    fn out_of_range_returns_none() {
        assert_eq!(shahanshahi_month_name(0), None);
        assert_eq!(shahanshahi_month_name(13), None);
        assert_eq!(shahanshahi_month_name_persian(0), None);
        assert_eq!(shahanshahi_month_name_persian(13), None);
    }
}
