//! strftime-style formatting for Shahanshahi dates.

const SHORT_NAMES_ASCII: [&str; 12] = [
    "Far", "Ord", "Kho", "Tir", "Mor", "Sha", "Meh", "Aba", "Aza", "Dey", "Bah", "Esf",
];

fn ascii_digit_to_persian(ch: char) -> char {
    match ch {
        '0' => '۰',
        '1' => '۱',
        '2' => '۲',
        '3' => '۳',
        '4' => '۴',
        '5' => '۵',
        '6' => '۶',
        '7' => '۷',
        '8' => '۸',
        '9' => '۹',
        other => other,
    }
}

/// Replaces every ASCII decimal digit in `s` with its Persian numeral equivalent.
///
/// Persian numerals: ۰۱۲۳۴۵۶۷۸۹ (U+06F0–U+06F9).
pub fn to_persian_numerals(s: &str) -> String {
    s.chars().map(ascii_digit_to_persian).collect()
}

/// Format a Shahanshahi date using a strftime-style format string.
///
/// Recognised specifiers:
///
/// | Specifier | Output |
/// |-----------|--------|
/// | `%Y` | 4-digit year |
/// | `%m` | 2-digit month, zero-padded (`01`–`12`) |
/// | `%d` | 2-digit day, zero-padded (`01`–`31`) |
/// | `%B` | Full ASCII month name (`"Farvardin"`) |
/// | `%b` | Short ASCII month name (`"Far"`) |
/// | `%A` | Full ASCII weekday name (`"Shanbeh"`) |
/// | `%e` | Day in Persian numerals, unpadded (`"۱"`) |
/// | `%%` | Literal `%` |
///
/// Unknown specifiers are passed through unchanged (e.g. `%z` → `%z`).
pub(crate) fn format_shahanshahi(
    year: i32,
    month: u8,
    day: u8,
    weekday: crate::Weekday,
    fmt: &str,
) -> String {
    let mut out = String::with_capacity(fmt.len() + 16);
    let mut chars = fmt.chars();

    while let Some(ch) = chars.next() {
        if ch != '%' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('Y') => {
                use core::fmt::Write;
                write!(out, "{:04}", year).unwrap();
            }
            Some('m') => {
                use core::fmt::Write;
                write!(out, "{:02}", month).unwrap();
            }
            Some('d') => {
                use core::fmt::Write;
                write!(out, "{:02}", day).unwrap();
            }
            Some('B') => {
                out.push_str(crate::months::shahanshahi_month_name(month).unwrap_or(""));
            }
            Some('b') => {
                if (1..=12).contains(&month) {
                    out.push_str(SHORT_NAMES_ASCII[(month - 1) as usize]);
                }
            }
            Some('A') => {
                out.push_str(weekday.name_ascii());
            }
            Some('e') => {
                for ch in to_persian_numerals(&day.to_string()).chars() {
                    out.push(ch);
                }
            }
            Some('%') => out.push('%'),
            Some(other) => {
                out.push('%');
                out.push(other);
            }
            None => out.push('%'),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Weekday;

    #[test]
    fn iso_format() {
        let result = format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%Y-%m-%d");
        assert_eq!(result, "2535-01-01");
    }

    #[test]
    fn full_names() {
        let result = format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%d %B %Y");
        assert_eq!(result, "01 Farvardin 2535");
    }

    #[test]
    fn short_month_name() {
        let result = format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%d %b %Y");
        assert_eq!(result, "01 Far 2535");
    }

    #[test]
    fn weekday_name() {
        let result = format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%A");
        assert_eq!(result, "Yekshanbeh");
    }

    #[test]
    fn persian_day() {
        let result = format_shahanshahi(2535, 1, 12, Weekday::Sunday, "%e %B %Y");
        assert_eq!(result, "۱۲ Farvardin 2535");
    }

    #[test]
    fn percent_escape() {
        let result = format_shahanshahi(2535, 1, 1, Weekday::Sunday, "100%%");
        assert_eq!(result, "100%");
    }

    #[test]
    fn unknown_specifier_passthrough() {
        let result = format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%z");
        assert_eq!(result, "%z");
    }

    #[test]
    fn to_persian_numerals_converts_digits() {
        assert_eq!(to_persian_numerals("2535"), "۲۵۳۵");
        assert_eq!(to_persian_numerals("01"), "۰۱");
        assert_eq!(to_persian_numerals("abc"), "abc");
    }

    #[test]
    fn short_names_all_twelve() {
        let expected = [
            "Far", "Ord", "Kho", "Tir", "Mor", "Sha", "Meh", "Aba", "Aza", "Dey", "Bah", "Esf",
        ];
        for (i, name) in expected.iter().enumerate() {
            let result = format_shahanshahi(2535, i as u8 + 1, 1, Weekday::Saturday, "%b");
            assert_eq!(result, *name);
        }
    }
}
