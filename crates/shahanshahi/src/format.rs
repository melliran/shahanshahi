//! strftime-style formatting for Shahanshahi dates.

const SHORT_NAMES_ASCII: [&str; 12] = [
    "Far", "Ord", "Kho", "Tir", "Mor", "Sha", "Meh", "Aba", "Aza", "Dey", "Bah", "Esf",
];

/// Output locale for [`ShahanshahiDate::format_localized`].
///
/// Controls whether month and weekday names are romanized ASCII or Persian script,
/// and whether digits are Western or Persian (U+06F0–U+06F9).
///
/// Mirrors the convention used by major Persian calendar libraries (jdatetime,
/// moment-jalaali): the same specifiers switch output based on locale rather than
/// using separate specifiers per script.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    /// ASCII romanized names, Western digits. This is the default for [`ShahanshahiDate::format`].
    En,
    /// Persian script names, Persian digits throughout the formatted output.
    Fa,
}

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
///
/// # Example
///
/// ```rust
/// use shahanshahi::to_persian_numerals;
/// assert_eq!(to_persian_numerals("2535/01/01"), "۲۵۳۵/۰۱/۰۱");
/// ```
pub fn to_persian_numerals(s: &str) -> String {
    s.chars().map(ascii_digit_to_persian).collect()
}

// TODO: when GregorianDate::format() is added, replace the flat params with a shared
// internal struct or trait rather than duplicating this signature.
/// Format a Shahanshahi date using a strftime-style format string.
///
/// Recognised specifiers:
///
/// | Specifier | `Locale::En` | `Locale::Fa` |
/// |-----------|-------------|-------------|
/// | `%Y` | `2535` | `۲۵۳۵` |
/// | `%m` | `01` | `۰۱` |
/// | `%d` | `01` | `۰۱` |
/// | `%B` | `Farvardin` | `فروردین` |
/// | `%b` | `Far` | `Far` |
/// | `%A` | `Shanbeh` | `شنبه` |
/// | `%%` | `%` | `%` |
///
/// Unknown specifiers are passed through unchanged. `Locale::Fa` converts all
/// output digits to Persian numerals after formatting.
pub(crate) fn format_shahanshahi(
    year: i32,
    month: u8,
    day: u8,
    weekday: crate::Weekday,
    fmt: &str,
    locale: Locale,
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
            Some('B') => match locale {
                Locale::En => {
                    out.push_str(crate::months::shahanshahi_month_name(month).unwrap_or(""))
                }
                Locale::Fa => {
                    out.push_str(crate::months::shahanshahi_month_name_persian(month).unwrap_or(""))
                }
            },
            Some('b') => {
                if (1..=12).contains(&month) {
                    out.push_str(SHORT_NAMES_ASCII[(month - 1) as usize]);
                }
            }
            Some('A') => match locale {
                Locale::En => out.push_str(weekday.name_ascii()),
                Locale::Fa => out.push_str(weekday.name_persian()),
            },
            Some('%') => out.push('%'),
            Some(other) => {
                out.push('%');
                out.push(other);
            }
            None => out.push('%'),
        }
    }

    if matches!(locale, Locale::Fa) {
        to_persian_numerals(&out)
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Weekday;

    #[test]
    fn iso_format_en() {
        assert_eq!(
            format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%Y-%m-%d", Locale::En),
            "2535-01-01"
        );
    }

    #[test]
    fn iso_format_fa() {
        assert_eq!(
            format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%Y-%m-%d", Locale::Fa),
            "۲۵۳۵-۰۱-۰۱"
        );
    }

    #[test]
    fn full_month_name_en() {
        assert_eq!(
            format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%d %B %Y", Locale::En),
            "01 Farvardin 2535"
        );
    }

    #[test]
    fn full_month_name_fa() {
        assert_eq!(
            format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%d %B %Y", Locale::Fa),
            "۰۱ فروردین ۲۵۳۵"
        );
    }

    #[test]
    fn short_month_name() {
        assert_eq!(
            format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%d %b %Y", Locale::En),
            "01 Far 2535"
        );
    }

    #[test]
    fn weekday_en() {
        assert_eq!(
            format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%A", Locale::En),
            "Yekshanbeh"
        );
    }

    #[test]
    fn weekday_fa() {
        assert_eq!(
            format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%A", Locale::Fa),
            "یکشنبه"
        );
    }

    #[test]
    fn full_persian_date() {
        assert_eq!(
            format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%A، %d %B %Y", Locale::Fa),
            "یکشنبه، ۰۱ فروردین ۲۵۳۵"
        );
    }

    #[test]
    fn percent_escape() {
        assert_eq!(
            format_shahanshahi(2535, 1, 1, Weekday::Sunday, "100%%", Locale::En),
            "100%"
        );
    }

    #[test]
    fn unknown_specifier_passthrough() {
        assert_eq!(
            format_shahanshahi(2535, 1, 1, Weekday::Sunday, "%z", Locale::En),
            "%z"
        );
    }

    #[test]
    fn to_persian_numerals_converts_digits() {
        assert_eq!(to_persian_numerals("2535/01/01"), "۲۵۳۵/۰۱/۰۱");
        assert_eq!(to_persian_numerals("abc"), "abc");
    }

    #[test]
    fn short_names_all_twelve() {
        let expected = [
            "Far", "Ord", "Kho", "Tir", "Mor", "Sha", "Meh", "Aba", "Aza", "Dey", "Bah", "Esf",
        ];
        for (i, name) in expected.iter().enumerate() {
            assert_eq!(
                format_shahanshahi(2535, i as u8 + 1, 1, Weekday::Saturday, "%b", Locale::En),
                *name
            );
        }
    }
}
