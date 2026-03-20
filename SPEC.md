# Shahanshahi calendar — implementation specification

This document **freezes** what the `shahanshahi` Rust crate implements. It is written for **deterministic software** (conversions, validation, tests), not for historiography beyond the cited civil regime.

**Document version:** `1` — matches [`SPEC_VERSION`](crates/shahanshahi/src/lib.rs) in the crate. This revision collates **statutes, contemporaneous parliamentary change, and institutional reproductions** of official practice during the late Pahlavi period. A few items remain **open** (Official Gazette pagination, full text of the 1978 repeal, scanned yearbook tables); those are listed under [Known gaps](#known-gaps-pending-hardening-for-010) and do not block stating the **normative rules** below.

## Scope

- **Intent:** Civil **Shahanshahi (Imperial Iranian, گاه‌شماری شاهنشاهی)** era as **briefly enacted** in Iran: epoch change approved **24 Esfand 1354** Hijri Shamsi, civil use from **1 Farvardin 2535** Shahanshahi (aligned with **1 Farvardin 1355** Hijri Shamsi), until the epoch was **revoked in Shahrivar 1357** (see [Era of applicability](#era-of-applicability)).
- **Model:** Shahanshahi is an **era overlay** on the **same twelve-month solar civil year** already defined by Iranian law (1925); only **year numbering** and **epoch** changed in 1976, not month names or month lengths.
- **Non-goals** (unless explicitly added later): religious calendars, lunar Hijri, localization strings beyond what the crate documents, claiming **proleptic** Shahanshahi dates as **legal** outside the era below.

## Evidence classes

| Class | Meaning in this spec |
|-------|----------------------|
| **Statute** | Text of law as published or transcribed from official publication. |
| **Near-primary** | Same-day or archival institutional summary that quotes or closely paraphrases official acts (not a substitute for Gazette when a rule is disputed). |
| **Interpretation** | Logic required for software that statutes do not spell out (e.g. integer offset **+1180**), clearly labeled. |

---

## Primary references

Full citations for the rules below. Prefer retrieving **Official Gazette (روزنامه رسمی)** facsimiles when publishing `0.1.0`; issue numbers for the 1976 resolution and 1978 repeal are still to be pinned in this repository.

| Topic | Citation | Notes |
|--------|-----------|--------|
| Month count, names, ordinary lengths, leap note (Esfand 30 in leap years), Hijra epoch, New Year = first day of spring, “true solar” year | **Law on the Conversion of the Zodiacal Months (قانون تبدیل برج‌ها)**, Fifth National Consultative Assembly, night of **11 Farvardin 1304** Hijri Shamsi (1925). Full Persian text transcribed: [Iranian Astronomical Society — «یکصدمین سال تصویب تقویم در ایران»](https://www.asi.ir/fa/news/13025/%DB%8C%DA%A8%D8%B5%D8%AF%D9%85%DB%8C%D9%86-%D8%B3%D8%A7%D9%84-%D8%AA%D8%B5%D9%88%DB%8C%D8%A8-%D8%AA%D9%82%D9%88%DB%8C%D9%85-%D8%AF%D8%B1-%D8%A7%DB%8C%D8%B1%D8%A7%D9%86). **Official Gazette** entry for this law should be cited when available. | Defines the **twelve Persian month names**, **31/30/29(+1)** pattern, and that leap years extend **Esfand** to 30 days. Does **not** encode a numeric leap cycle in statute. |
| Joint resolution: Cyrus’s reign as calendar epoch; civil transition to Shahanshahi year at Nowruz 1355 | **Joint session** of National Consultative Assembly and Senate, **24 Esfand 1354** Hijri Shamsi, presidency **Jaʿfar Sharif-Emami**. Contemporaneous report: ***Ettelāʿāt***, **no. 14959**, **Sunday 24 Esfand 1354**, pp. 1 & 5 (headline and resolution text). Reproduced in e.g. [Motaghin — related publication page](https://motaghin.com/fa_booksPage_4556.aspx?gid=4451); [History Documents — post on Torabti](https://www.historydocuments.ir/?page=post&id=4193). | **Near-primary** witness for wording that the **beginning of Cyrus the Great’s reign** is the **starting point of the calendar and epoch of national history**; **next Nowruz = 2535 Shahanshahi**. |
| Institutional summary: 559 BCE coronation as epoch; **1 Farvardin 1355 H.Sh. = 1 Farvardin 2535** Shahanshahi; later annulment | **Political Studies and Research Institute (PSRI)** — «تقویم شاهنشاهی», [psri.ir special](https://psri.ir/?id=240&page=special); «تاریخ شاهنشاهی», [psri.ir](https://psri.ir/?id=oshllx94). | **Near-primary**; aligns epoch with **Cyrus’s coronation in 559 BCE** and the **1355/2535** mapping; notes **annulment** of Shahanshahi in **1357**. |
| Same epoch change (chronology entry) | **Institute for Compilation and Publication of Imam Khomeini’s Works** — event log **24/12/1354**, [NewsPrint ID=11925](http://www.imam-khomeini.ir/fa/NewsPrint.aspx?ID=11925). | Confirms **official** switch and **1355 → 2535** labeling. |
| Repeal (effect: return to Hijri Shamsi epoch) | Multiple Iranian “this day in history” chronologies cite **5 Shahrivar 1357** as abrogation of the **“law on the epoch of Iran”** (e.g. [Tebyan](https://www.tebyan.net/news/255826/%D9%BE%D9%86%D8%AC%D9%85-%D8%B4%D9%87%D8%B1%DB%8C%D9%88%D8%B1-%D8%AF%D8%B1-%D8%A2%DB%8C%D9%86%D9%87-%D8%AA%D8%A7%D8%B1%DB%8C%D8%AE-%D9%85%D8%B9%D8%A7%D8%B5%D8%B1), [Titre Azad](https://www.titreazad.ir/1540-%D9%BE%D9%86%D8%AC%D9%85-%D8%B4%D9%87%D8%B1%DB%8C%D9%88%D8%B1-%D8%AF%D8%B1-%D8%A2%DB%8C%D9%86%D9%87-%D8%AA%D8%A7%D8%B1%DB%8C%D8%AE-%D9%85%D8%B9%D8%A7%D8%B5%D8%B1)). **PSRI** (same «تقویم شاهنشاهی» page) notes annulment in **1357**. | **Medium** confidence on **calendar effect**; **full repealing instrument** (Gazette text) **not yet** anchored in this repo. |
| Technical concordance (not statute) | Encyclopedic articles (e.g. [HandWiki — Shahanshahi calendar](https://handwiki.org/wiki/History:Shahanshahi_calendar), [Wikipedia — Shahanshahi calendar](https://en.wikipedia.org/wiki/Shahanshahi_calendar)) | Useful for **cross-checks** only; **do not** treat as legal primary. |

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

### Leap years

**Legal rule (statute):** The civil year is a **true solar year**; the year **begins on the first day of spring**; **in leap years, Esfand has 30 days**. Statute does **not** define a closed-form arithmetic cycle (e.g. 33-year or 2820-year).

**Normative consequence for Shahanshahi:** During the Shahanshahi era, **leap years are exactly those Hijri Shamsi leap years** for the same solar year (same Esfand length as under pre-1976 civil practice).

**Worked examples (pending table verification):** Calendar literature commonly treats **1355 H.Sh. (2535 Shahanshahi)** as **leap** and **1356 H.Sh. (2536 Shahanshahi)** as **common**. This spec **accepts** that pairing for **implementation planning** until **official yearbook or Gazette table** pages are cited in [`data/reference-dates.json`](data/reference-dates.json). **Golden tests** must not rely on those years until a row cites an inspected primary table.

**Implementation note:** A **deterministic algorithm** (e.g. published 33-year or 2820-year approximation to the astronomical rule) is an **implementation choice**, not statute. Any crate using such an algorithm **must** document it as **approximating** the legal “true solar” rule.

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

---

## Versioning

- **`SPEC.md`:** When rules change, add an entry to [Changelog](#changelog) with date and rationale.
- **Crate semver:** Bugfixes that align code with this spec are patch bumps. Intentional spec changes require minor/major per semver and a migration note.

## Relationship to `data/reference-dates.json`

Every row must cite a **traceable source** independent of this codebase. The implementation must pass all golden tests derived from those rows. Until leap/end-date rows are filled from inspected tables, tests may cover only **uncontroversial anchors** (e.g. **1 Farvardin 2535** pairing).

## Changelog

| Date | Change |
|------|--------|
| 2026-03-20 | **Spec version 1:** Primary reference table; epoch (+1180 mapping); twelve months; leap as statutory true-solar with same leap set as Hijri Shamsi; era bounds; proleptic policy; known gaps. |
| _prior_ | Initial draft template (`0-unstable`). |
