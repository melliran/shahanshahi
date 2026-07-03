//! Day of week, with Saturday as the first day of the Iranian civil week.

use core::fmt;

/// A day of the week, ordered with **Saturday** (the first day of the Iranian civil week) first.
///
/// The Iranian week runs Saturday → Sunday → … → Friday, with Friday as the day of rest.
/// Derived `Ord` reflects this ordering: `Weekday::Saturday < Weekday::Friday`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Weekday {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl Weekday {
    /// 0-based index from **Saturday** (Iranian week start): Saturday = 0, …, Friday = 6.
    #[inline]
    pub fn number_from_saturday(self) -> u8 {
        self as u8
    }

    /// English transliteration of the Persian weekday name.
    pub fn name_ascii(self) -> &'static str {
        match self {
            Weekday::Saturday => "Shanbeh",
            Weekday::Sunday => "Yekshanbeh",
            Weekday::Monday => "Doshanbeh",
            Weekday::Tuesday => "Seshanbeh",
            Weekday::Wednesday => "Chaharshanbeh",
            Weekday::Thursday => "Panjshanbeh",
            Weekday::Friday => "Jom'eh",
        }
    }

    /// Persian-script weekday name (Unicode).
    pub fn name_persian(self) -> &'static str {
        match self {
            Weekday::Saturday => "شنبه",
            Weekday::Sunday => "یکشنبه",
            Weekday::Monday => "دوشنبه",
            Weekday::Tuesday => "سه‌شنبه",
            Weekday::Wednesday => "چهارشنبه",
            Weekday::Thursday => "پنجشنبه",
            Weekday::Friday => "جمعه",
        }
    }
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name_ascii())
    }
}

/// Maps a Rata Die to the corresponding [`Weekday`].
///
/// RD 1 = Monday (Gregorian 1 Jan 1 CE). Uses `rem_euclid` so negative RDs are handled correctly.
#[inline]
pub(crate) fn weekday_from_rata_die(rd: i64) -> Weekday {
    match rd.rem_euclid(7) {
        0 => Weekday::Sunday,
        1 => Weekday::Monday,
        2 => Weekday::Tuesday,
        3 => Weekday::Wednesday,
        4 => Weekday::Thursday,
        5 => Weekday::Friday,
        _ => Weekday::Saturday, // 6
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Anchor: RD 721_434 = 1976-03-21 (Gregorian) = Sunday.
    // 721_434 mod 7 = 0, and rem_euclid(7) == 0 maps to Sunday.
    #[test]
    fn anchor_rd_is_sunday() {
        assert_eq!(weekday_from_rata_die(721_434), Weekday::Sunday);
    }

    #[test]
    fn week_walk_from_anchor() {
        let expected = [
            Weekday::Sunday,
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
        ];
        for (i, &day) in expected.iter().enumerate() {
            assert_eq!(weekday_from_rata_die(721_434 + i as i64), day, "offset {i}");
        }
    }

    #[test]
    fn negative_rd_does_not_panic() {
        // RD 0 = Sunday (0 rem_euclid 7 = 0); RD -1 = Saturday ((-1).rem_euclid(7) = 6).
        assert_eq!(weekday_from_rata_die(0), Weekday::Sunday);
        assert_eq!(weekday_from_rata_die(-1), Weekday::Saturday);
    }

    #[test]
    fn number_from_saturday_ordering() {
        assert_eq!(Weekday::Saturday.number_from_saturday(), 0);
        assert_eq!(Weekday::Friday.number_from_saturday(), 6);
    }

    #[test]
    fn name_ascii_all_seven() {
        let expected = [
            (Weekday::Saturday, "Shanbeh"),
            (Weekday::Sunday, "Yekshanbeh"),
            (Weekday::Monday, "Doshanbeh"),
            (Weekday::Tuesday, "Seshanbeh"),
            (Weekday::Wednesday, "Chaharshanbeh"),
            (Weekday::Thursday, "Panjshanbeh"),
            (Weekday::Friday, "Jom'eh"),
        ];
        for (day, name) in expected {
            assert_eq!(day.name_ascii(), name);
        }
    }

    #[test]
    fn name_persian_spot_check() {
        assert_eq!(Weekday::Saturday.name_persian(), "شنبه");
        assert_eq!(Weekday::Friday.name_persian(), "جمعه");
    }

    #[test]
    fn display_uses_ascii_name() {
        assert_eq!(Weekday::Sunday.to_string(), "Yekshanbeh");
        assert_eq!(Weekday::Saturday.to_string(), "Shanbeh");
    }

    #[test]
    fn ord_follows_iranian_week_order() {
        assert!(Weekday::Saturday < Weekday::Sunday);
        assert!(Weekday::Sunday < Weekday::Friday);
        assert!(Weekday::Thursday < Weekday::Friday);
    }
}
