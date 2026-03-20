# Shahanshahi calendar — implementation specification

This document **freezes** what the `shahanshahi` Rust crate implements. It is written for **deterministic software** (conversions, validation, tests), not for historiography beyond the cited civil regime.

**Document version:** `2` — matches [`SPEC_VERSION`](crates/shahanshahi/src/lib.rs) in the crate. This revision collates **statutes**, **contemporaneous parliamentary change**, **institutional reproductions** of official practice, and **peer‑reviewed astronomical / calendrical literature** for how a **true solar** New Year relates to the **vernal equinox** and to **software algorithms**. Gazette/yearbook gaps remain under [Known gaps](#known-gaps-pending-hardening-for-010).

## Scope

- **Intent:** Civil **Shahanshahi (Imperial Iranian, گاه‌شماری شاهنشاهی)** era as **briefly enacted** in Iran: epoch change approved **24 Esfand 1354** Hijri Shamsi, civil use from **1 Farvardin 2535** Shahanshahi (aligned with **1 Farvardin 1355** Hijri Shamsi), until the epoch was **revoked in Shahrivar 1357** (see [Era of applicability](#era-of-applicability)).
- **Model:** Shahanshahi is an **era overlay** on the **same twelve-month solar civil year** already defined by Iranian law (1925); only **year numbering** and **epoch** changed in 1976, not month names or month lengths.
- **Non-goals** (unless explicitly added later): religious calendars, lunar Hijri, localization strings beyond what the crate documents, claiming **proleptic** Shahanshahi dates as **legal** outside the era below.

## Evidence classes

| Class | Meaning in this spec |
|-------|----------------------|
| **Statute** | Text of law as published or transcribed from official publication. |
| **Near-primary** | Same-day or archival institutional summary that quotes or closely paraphrases official acts (not a substitute for Gazette when a rule is disputed). |
| **Scholarly** | Peer-reviewed or preprint literature on the astronomical Iranian / Solar Hijri calendar; **not** statute, but primary for **equinox‑aligned** operational definitions used in implementations. |
| **Informative** | Supporting encyclopedic or textbook material cited only for definitions (e.g. equinox as λ = 0°). |
| **Interpretation** | Logic required for software that statutes do not spell out (e.g. integer offset **+1180**), clearly labeled. |

---

## Primary references (legal and institutional)

Full citations for **civil law and epoch**. Prefer **Official Gazette (روزنامه رسمی)** facsimiles when publishing `0.1.0`.

| Topic | Citation | Notes |
|--------|-----------|--------|
| Month count, names, ordinary lengths, leap note (Esfand 30 in leap years), Hijra epoch, New Year = first day of spring, “true solar” year | **Law on the Conversion of the Zodiacal Months (قانون تبدیل برج‌ها)**, Fifth National Consultative Assembly, night of **11 Farvardin 1304** Hijri Shamsi (1925). Full Persian text transcribed: [Iranian Astronomical Society — «یکصدمین سال تصویب تقویم در ایران»](https://www.asi.ir/fa/news/13025/%DB%8C%DA%A8%D8%B5%D8%AF%D9%85%DB%8C%D9%86-%D8%B3%D8%A7%D9%84-%D8%AA%D8%B5%D9%88%DB%8C%D8%A8-%D8%AA%D9%82%D9%88%DB%8C%D9%85-%D8%AF%D8%B1-%D8%A7%DB%8C%D8%B1%D8%A7%D9%86). **Official Gazette** entry for this law should be cited when available. | Defines the **twelve Persian month names**, **31/30/29(+1)** pattern, and that leap years extend **Esfand** to 30 days. Does **not** encode a numeric leap cycle in statute. |
| Joint resolution: Cyrus’s reign as calendar epoch; civil transition to Shahanshahi year at Nowruz 1355 | **Joint session** of National Consultative Assembly and Senate, **24 Esfand 1354** Hijri Shamsi, presidency **Jaʿfar Sharif-Emami**. Contemporaneous report: ***Ettelāʿāt***, **no. 14959**, **Sunday 24 Esfand 1354**, pp. 1 & 5 (headline and resolution text). Reproduced in e.g. [Motaghin — related publication page](https://motaghin.com/fa_booksPage_4556.aspx?gid=4451); [History Documents — post on Torabti](https://www.historydocuments.ir/?page=post&id=4193). | **Near-primary** witness for wording that the **beginning of Cyrus the Great’s reign** is the **starting point of the calendar and epoch of national history**; **next Nowruz = 2535 Shahanshahi**. |
| Institutional summary: 559 BCE coronation as epoch; **1 Farvardin 1355 H.Sh. = 1 Farvardin 2535** Shahanshahi; later annulment | **Political Studies and Research Institute (PSRI)** — «تقویم شاهنشاهی», [psri.ir special](https://psri.ir/?id=240&page=special); «تاریخ شاهنشاهی», [psri.ir](https://psri.ir/?id=oshllx94). | **Near-primary**; aligns epoch with **Cyrus’s coronation in 559 BCE** and the **1355/2535** mapping; notes **annulment** of Shahanshahi in **1357**. |
| Same epoch change (chronology entry) | **Institute for Compilation and Publication of Imam Khomeini’s Works** — event log **24/12/1354**, [NewsPrint ID=11925](http://www.imam-khomeini.ir/fa/NewsPrint.aspx?ID=11925). | Confirms **official** switch and **1355 → 2535** labeling. |
| Repeal (effect: return to Hijri Shamsi epoch) | Multiple Iranian “this day in history” chronologies cite **5 Shahrivar 1357** as abrogation of the **“law on the epoch of Iran”** (e.g. [Tebyan](https://www.tebyan.net/news/255826/%D9%BE%D9%86%D8%AC%D9%85-%D8%B4%D9%87%D8%B1%DB%8C%D9%88%D8%B1-%D8%AF%D8%B1-%D8%A2%DB%8C%D9%86%D9%87-%D8%AA%D8%A7%D8%B1%DB%8C%D8%AE-%D9%85%D8%B9%D8%A7%D8%B5%D8%B1), [Titre Azad](https://www.titreazad.ir/1540-%D9%BE%D9%86%D8%AC%D9%85-%D8%B4%D9%87%D8%B1%DB%8C%D9%88%D8%B1-%D8%AF%D8%B1-%D8%A2%DB%8C%D9%86%D9%87-%D8%AA%D8%A7%D8%B1%DB%8C%D8%AE-%D9%85%D8%B9%D8%A7%D8%B5%D8%B1)). **PSRI** (same «تقویم شاهنشاهی» page) notes annulment in **1357**. | **Medium** confidence on **calendar effect**; **full repealing instrument** (Gazette text) **not yet** anchored in this repo. |
| Technical concordance (not statute) | Encyclopedic articles (e.g. [HandWiki — Shahanshahi calendar](https://handwiki.org/wiki/History:Shahanshahi_calendar), [Wikipedia — Shahanshahi calendar](https://en.wikipedia.org/wiki/Shahanshahi_calendar)) | Useful for **cross-checks** only; **do not** treat as legal primary. |

---

## Astronomical and operational references

These sources **do not replace** statute; they operationalize **“first day of spring” / true solar year** for **implementations** and justify **arithmetic** vs **ephemeris** modes.

| Topic | Citation | Notes |
|--------|-----------|--------|
| Vernal‑equinox year, official calendar tied to Sun crossing vernal equinox, Esfand 30 in leap years, discussion of intercalation cycles (33, 128, 161, 2820 years) as **mathematical** proposals | **Akrami, M.** (2011). *On the astronomical basis of the Iranian calendar and precision of its leap years.* [arXiv:1111.4926](https://arxiv.org/abs/1111.4926) — [PDF](https://arxiv.org/pdf/1111.4926.pdf). | **Scholarly.** Emphasizes **vernal‑equinox (tropical) year** and **natural/astronomical** (vs purely conventional) reading of the civil calendar. Large cycles are **not** statutory. |
| Interval between successive vernal equinoxes; **Nowruz** as **midnight closest to equinox** (Tehran); before/after **local noon** rule for assigning **Farvardin 1**; IMCCE ephemerides; **33‑year** cycle (8 leap years per 33); residues **1, 5, 9, 13, 17, 22, 26, 30 mod 33** | **Heydari-Malayeri, M.** (2004). *A concise review of the Iranian calendar.* [arXiv:astro-ph/0409620](https://arxiv.org/abs/astro-ph/0409620) — [PDF](https://arxiv.org/pdf/astro-ph/0409620.pdf). | **Scholarly.** Primary spec support for **operational Nowruz** and for the **mod‑33 leap pattern** used in Mode A. |
| Vernal equinox: Sun’s **apparent geocentric ecliptic longitude** λ = **0°** (mod 360°) | [Wikipedia — *Equinox*](https://en.wikipedia.org/wiki/Equinox) | **Informative** only, for the **λ = 0°** definition in Mode B. |
| 33‑year rule vs equinox tables over a long Gregorian span | **Borkowski, K. M.**; calendar-software literature (e.g. **Reingold & Dershowitz**, *Calendrical Calculations*) as cited in Heydari‑Malayeri and follow-on work | **Scholarly / technical.** Cited to bound **Mode A** validity (roughly **Gregorian years ~1799–2256** / related Solar Hijri range per Borkowski’s comparison to IMCCE-style data). |

---

## Calendar rules

### Epoch

**Year 1** of the Shahanshahi era is anchored historically to the **coronation of Cyrus the Great**, taken in official narrative as **559 BCE** (see PSRI citations above).

**Normative mapping during legal civil use:** For any civil date in the [era of applicability](#era-of-applicability), the **same** month index and day-of-month as **Hijri Shamsi** apply; only the **year number** differs:

- Let **Y_H** be the Hijri Shamsi civil year and **Y_S** the Shahanshahi civil year on the **same physical day**.
- **Interpretation (software):** **Y_S = Y_H + 1180** for that period. This integer is **not** quoted verbatim in the joint resolution; it is the **unique** offset consistent with **1355 ↔ 2535** stated in PSRI, Imam Khomeini Institute chronology, and widespread technical descriptions.

**Anchor pair (required invariant for tests):**

- **1 Farvardin 1355 Hijri Shamsi** ≡ **1 Farvardin 2535 Shahanshahi** (same civil day).

### Months

The year has **12** months in fixed order. **Month index** for implementers: **1 = Farvardin … 12 = Esfand**.

| Index | Name (Persian) | Days (ordinary year) |
|------:|----------------|----------------------:|
| 1 | Farvardin | 31 |
| 2 | Ordibehesht | 31 |
| 3 | Khordad | 31 |
| 4 | Tir | 31 |
| 5 | Mordad | 31 |
| 6 | Shahrivar | 31 |
| 7 | Mehr | 30 |
| 8 | Aban | 30 |
| 9 | Azar | 30 |
| 10 | Dey | 30 |
| 11 | Bahman | 30 |
| 12 | Esfand | 29 |

**In leap years, Esfand has 30 days** (1925 law, note to Article 1(d)).

Month **names** are **in scope** for this spec: they are **fixed by law** (1925) and unchanged by the 1976 epoch resolution.

### Nowruz and the vernal equinox (operational model for software)

**Statute (1925)** requires the year to begin on the **first day of spring** and to follow a **true solar** year, with **Esfand** extended in leap years — without defining λ(t) or a numeric cycle.

**Scholarly operational definition (implementations SHOULD align with Heydari‑Malayeri 2004):**

- The civil year is the interval between **two successive vernal equinoxes**.
- **Farvardin 1** is the civil day determined by the **midnight closest to the vernal equinox instant**, evaluated in **Tehran legal time** (see Heydari‑Malayeri for the **before/after local noon** rule):
  - If the equinox instant falls **before local noon** on civil day *D*, **Farvardin 1** is *D*.
  - If it falls **after local noon**, **Farvardin 1** is the **next** civil day.

**Conceptual definition of the equinox instant (Mode B):** The **vernal equinox** is the time when the Sun’s **apparent geocentric ecliptic longitude** is **0°** (mod 360°), i.e. the northward crossing of the celestial equator — see [Wikipedia — *Equinox*](https://en.wikipedia.org/wiki/Equinox) (**informative**) and Akrami (2011) for astronomical framing.

**Akrami (2011)** supports treating the calendar as tied to the **vernal‑equinox tropical year** and explains **why** long arithmetic cycles (e.g. 2820 years) are discussed as **approximations**, not law.

### Leap years

**Legal rule (statute):** The civil year is a **true solar year**; the year **begins on the first day of spring**; **in leap years, Esfand has 30 days**. Statute does **not** define a closed-form arithmetic cycle.

**Normative consequence for Shahanshahi:** During the Shahanshahi era, **leap years are exactly those Solar Hijri leap years** for the same solar year (same Esfand length as under pre-1976 civil practice).

**Leap status in the Shahanshahi legal window (Mode A + public tables):** Under the **mod‑33** rule in [§ Implementation modes](#implementation-modes-informative-defaults-for-this-repository), **1354 H.Sh.** is **leap** and **1355–1357 H.Sh.** are **common** (residues **1** vs **2–4** mod 33). The English Wikipedia **Solar Hijri calendar** comparison table likewise marks **1354\*** as leap and **1355–1357** without a leap mark — see [`data/reference-dates.json`](data/reference-dates.json) rows citing that table.

---

### Implementation modes (informative defaults for this repository)

Implementations **MUST** document which mode they use. **Recommended default** for a **deterministic, offline** library: **Mode A**. **Mode B** is **optional** (feature-gated or separate tool) when ephemeris or network access is acceptable.

#### Mode A — Arithmetic 33-year rule (deterministic)

**Basis:** Heydari‑Malayeri (2004) derives the **33-year, 8-leap** pattern from IMCCE equinox data; **Borkowski** and calendar literature report agreement with equinox-based Nowruz over a broad range (order of **Gregorian ~1799–2256**).

Let **Y_H** be the **Solar Hijri** civil year. Compute **r = Y_H mod 33** with **r ∈ {0,…,32}** (use nonnegative remainder for negative proleptic years). Then **Y_H** is a **leap year** iff:

**r ∈ {1, 5, 9, 13, 17, 22, 26, 30}**.

For **Shahanshahi** year **Y_S**, first map to the underlying Hijri Shamsi year **Y_H = Y_S − 1180** (see [proleptic policy](#proleptic-use)); then apply the same predicate to **Y_H**.

**Rust (reference):**

```rust
/// Solar Hijri leap year per the 33-year arithmetic rule (Heydari-Malayeri / Borkowski range).
pub fn is_solar_hijri_leap_arithmetic(y_h: i32) -> bool {
    let r = y_h.rem_euclid(33);
    matches!(r, 1 | 5 | 9 | 13 | 17 | 22 | 26 | 30)
}

/// Underlying Solar Hijri year from Shahanshahi year (proleptic offset).
pub fn shahanshahi_to_hijri_shamsi_year(y_s: i32) -> i32 {
    y_s - 1180
}

/// Shahanshahi leap year using Mode A (same solar year as underlying Hijri Shamsi).
pub fn is_shahanshahi_leap_arithmetic(y_s: i32) -> bool {
    is_solar_hijri_leap_arithmetic(shahanshahi_to_hijri_shamsi_year(y_s))
}
```

**Caveats:** Outside the validated range, Mode A is **extrapolation**. It is **not** a statute; it **approximates** the equinox rule.

#### Mode B — Ephemeris “true equinox” (JPL / NASA)

**Basis:** Same **Heydari‑Malayeri** midnight / noon assignment of **Farvardin 1**, but the equinox instant **t_eq** is obtained from **high-precision solar ephemerides** (e.g. **NASA JPL Horizons** or **DE430** + **SPICE** / equivalent Rust libraries).

**Informative algorithm:**

1. For the target solar year, choose a UTC window around **19–22 March** (adjust for year).
2. Obtain the Sun’s **apparent geocentric ecliptic longitude** λ(*t*) (geocenter **500@399** is typical; topocentric Tehran is optional refinement).
3. **Bracket** *t_eq* where λ crosses **0°** northward (handle 360°/0° wrap).
4. Refine (bisection, interpolation, or root-finding on λ(*t*) − 0°).
5. Convert *t_eq* to **Asia/Tehran** civil time and apply **before/after local noon** per Heydari‑Malayeri.
6. A year is **leap** iff there are **366** civil days between **successive** Nowruz-starting midnights (i.e. prior **Esfand** has 30 days).

**Horizons (HTTP):** Query ephemeris tables for the Sun with ecliptic longitude, coarse time steps, then refine — suitable for **build-time table generation** or **online** tools (crate policy may forbid network in core library).

**Offline:** Load **DE** files and evaluate λ(*t*) via **SPICE** or another ephemeris backend; same bracketing/refinement in **Rust**.

**Rust (illustrative fragment — longitude samples from your ephemeris backend):**

```rust
/// Given ordered pairs (time_seconds_tai_or_tt, ecliptic_longitude_deg),
/// find index `i` such that λ crosses 0° northward between sample `i` and `i+1`.
pub fn bracket_vernal_longitude_crossing(samples: &[(f64, f64)]) -> Option<usize> {
    for i in 0..samples.len().saturating_sub(1) {
        let (_, lam0) = samples[i];
        let (_, lam1) = samples[i + 1];
        let d0 = ((lam0 % 360.0) + 360.0) % 360.0;
        let d1 = ((lam1 % 360.0) + 360.0) % 360.0;
        // Northward crossing: just before 360/0 wrap (e.g. > 300° then < 60°)
        if d0 > 300.0 && d1 < 60.0 {
            return Some(i);
        }
    }
    None
}

/// Linear interpolation of time when λ crosses 0° (degrees), between two samples.
pub fn interpolate_equinox_time(t0: f64, l0: f64, t1: f64, l1: f64) -> f64 {
    let l0n = ((l0 % 360.0) + 360.0) % 360.0;
    let l1n = ((l1 % 360.0) + 360.0) % 360.0;
    let y0 = if l0n > 180.0 { l0n - 360.0 } else { l0n };
    let y1 = if l1n > 180.0 { l1n - 360.0 } else { l1n };
    t0 + (t1 - t0) * (-y0) / (y1 - y0)
}
```

Integrate with **chrono-tz** (or fixed **UTC+3:30** offset where historically acceptable) for Tehran civil time before applying the **noon** rule.

### Era of applicability

**Start (high confidence):** Civil Shahanshahi numbering is in force from:

- **1 Farvardin 2535 Shahanshahi** = **1 Farvardin 1355 Hijri Shamsi**  
  Decision dated **24 Esfand 1354**; compulsory use from **start of 1355** per PSRI «تاریخ شاهنشاهی» and related sources.

**End (medium confidence; effective-day ambiguity):** Sources agree the Shahanshahi epoch was **revoked in Shahrivar 1357** (often **5 Shahrivar 1357** as the political/legal date of abrogation). Some technical summaries use **2 September 1978** / **11 Shahrivar** for “reversion,” yielding a **few-day** ambiguity for the **last** civil date stamped Shahanshahi.

**Conservative inclusive window for “legal Shahanshahi civil era” (spec default until Gazette clarifies):**

- **From** **1 Farvardin 2535** Shahanshahi (same day as **1 Farvardin 1355** Hijri Shamsi).  
- **Through** a **last civil day** that secondary sources dispute by a few days (e.g. some technical summaries treat **10 Shahrivar 2537** / **10 Shahrivar 1357** as the last Shahanshahi‑numbered civil day; chronologies often emphasize **5 Shahrivar 1357** as the **abrogation** date). Implementations **must** document the chosen **last day** until the **repealing instrument** is cited.

### Proleptic use

**Legal stance:** Primary materials located for this spec **do not** authorize Shahanshahi year numbers for **civil** use **before** 1 Farvardin 2535 or **after** repeal. Extending **Y_S = Y_H + 1180** outside the legal era is **conventional only** (cultural or diaspora practice).

**Crate policy (normative for this repository):**

1. **Default:** **Reject** Shahanshahi dates outside the [era of applicability](#era-of-applicability) (or outside a **narrower** window if the implementation fixes the end-date ambiguity more conservatively).
2. **Optional:** A **feature-gated** module or API may implement **proleptic** arithmetic (**same month grid + offset**) with documentation that such dates are **not** historically legal civil dates.

---

## Relationship to Hijri Shamsi (Solar Hijri)

For dates **within** the Shahanshahi civil era, conversion between Shahanshahi and Hijri Shamsi civil dates is **year translation only** with **identical** month and day. Gregorian (or Julian) conversion **inherits** the same ephemeris or national tables as for Hijri Shamsi for those civil dates.

---

## Known gaps (pending hardening for 0.1.0)

1. **Official Gazette** issue number, date, and page for the **1976 joint resolution** and for the **1978 repeal** (full Persian text).
2. **Scanned calendar pages** from official **سالنامه** (e.g. *سالنامه کشور ایران*, سال سی‌و‌یکم ۲۵۳۵) proving **Esfand** length for **2535** vs **2536**.
3. **Final word** on **last civil day** under Shahanshahi numbering vs first day **only** Hijri Shamsi year labels in official use.
4. **Bibliographic pin** for Borkowski’s published comparison used alongside Heydari‑Malayeri (for citation in golden-date rows).

---

## Versioning

- **`SPEC.md`:** When rules change, add an entry to [Changelog](#changelog) with date and rationale.
- **Crate semver:** Bugfixes that align code with this spec are patch bumps. Intentional spec changes require minor/major per semver and a migration note.

## Relationship to `data/reference-dates.json`

Every row must cite a **traceable source** independent of this codebase. The implementation must pass all golden tests derived from those rows. Until leap/end-date rows are filled from inspected tables, tests may cover only **uncontroversial anchors** (e.g. **1 Farvardin 2535** pairing).

## Changelog

| Date | Change |
|------|--------|
| 2026-03-21 | **Spec version 2:** Astronomical references (Heydari‑Malayeri arXiv:astro-ph/0409620, Akrami arXiv:1111.4926, Wikipedia *Equinox*); operational Nowruz/equinox model; **Mode A** (33‑year arithmetic + Rust reference) and **Mode B** (JPL Horizons / DE+SPICE + illustrative Rust). |
| 2026-03-20 | **Spec version 1:** Primary reference table; epoch (+1180 mapping); twelve months; leap as statutory true-solar with same leap set as Hijri Shamsi; era bounds; proleptic policy; known gaps. |
| _prior_ | Initial draft template (`0-unstable`). |
