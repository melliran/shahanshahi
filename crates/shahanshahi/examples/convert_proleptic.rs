//! Shahanshahi labelled dates **outside** the legal civil era, using the same 1925 grid + Mode A leap.
//!
//! Requires Cargo feature **`proleptic`**. Run from the repository root:
//! ```text
//! cargo run -p shahanshahi --example convert_proleptic --features proleptic
//! ```

use shahanshahi::ShahanshahiDate;

fn main() {
    // Golden-aligned: last day of Shahanshahi 2554 ≡ 1996-03-19 Gregorian (see reference_dates tests).
    let sh = ShahanshahiDate::try_new_proleptic(2554, 12, 29).expect("valid proleptic YMD");
    let g = sh.to_gregorian();
    println!(
        "Proleptic Shahanshahi {:04}-{:02}-{:02} → Gregorian {:04}-{:02}-{:02}",
        sh.year(),
        sh.month(),
        sh.day(),
        g.year(),
        g.month(),
        g.day(),
    );

    let back = ShahanshahiDate::try_from_gregorian_proleptic(g).expect("round-trip (proleptic)");
    assert_eq!(back, sh);
    println!("Round-trip via try_from_gregorian_proleptic: OK");
}
