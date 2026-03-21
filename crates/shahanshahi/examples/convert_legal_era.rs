//! Legal-era Shahanshahi ↔ Gregorian conversion (default crate policy).
//!
//! Run from the repository root:
//! ```text
//! cargo run -p shahanshahi --example convert_legal_era
//! ```

use shahanshahi::ShahanshahiDate;

fn main() {
    let sh = ShahanshahiDate::try_new(2535, 1, 1).expect("1 Farvardin 2535 (era start)");
    let g = sh.to_gregorian();
    println!(
        "Shahanshahi {:04}-{:02}-{:02} → Gregorian {:04}-{:02}-{:02}",
        sh.year(),
        sh.month(),
        sh.day(),
        g.year(),
        g.month(),
        g.day(),
    );

    let back = ShahanshahiDate::try_from_gregorian(g).expect("round-trip in legal era");
    assert_eq!(back, sh);
    println!("Round-trip via try_from_gregorian: OK");
}
