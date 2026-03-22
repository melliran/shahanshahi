//! JSON round-trip for `serde` on public date types (feature **`serde`**).

use shahanshahi::{GregorianDate, ShahanshahiDate};

#[test]
fn shahanshahi_json_round_trip() {
    let d = ShahanshahiDate::try_new(2535, 1, 1).unwrap();
    let v = serde_json::to_value(d).unwrap();
    let back: ShahanshahiDate = serde_json::from_value(v).unwrap();
    assert_eq!(d, back);
}

#[test]
fn gregorian_json_round_trip() {
    let d = GregorianDate::try_new(1976, 3, 21).unwrap();
    let v = serde_json::to_value(d).unwrap();
    let back: GregorianDate = serde_json::from_value(v).unwrap();
    assert_eq!(d, back);
}
