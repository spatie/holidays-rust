---
title: Coverage and compatibility
weight: 5
---

## PHP reference and verification

The port follows [spatie/holidays at 9557c35](https://github.com/spatie/holidays/tree/9557c35f121e27f473bcd6c8958826a205ffa449). It contains all 104 country implementations, all 105 registered country codes, all 435 region codes and all 50 translation files. Greece's `el` alias is retained.

The differential test compares 247,400 cases from the real PHP package across 1901–2100. It covers 1,237 combinations of country, region and applicable locale, including each country's default names and an unknown locale. Of these cases, 231,884 return holidays and 15,516 reject unavailable years. Comparisons retain duplicate holidays and check names, dates, types and regions. Same-day ordering is normalized because PHP sorts timestamps whose time-of-day and timezone may differ even when their calendar dates are identical.

PHP runs in UTC with Carbon's clock fixed to midnight on 2024-01-01, using ICU 76.1. The Rust runtime does not invoke PHP or load these test fingerprints. The fingerprints are SHA-256 digests of independently generated PHP results, not date tables used by the implementation. See [testing](testing.md) to reproduce them.

The public API tests also cover inclusive and reversed date ranges, partial dates, translated lookup, type filtering, duplicate dates, upcoming holidays across years, long-weekend grouping and Serde output. Matching the PHP package does not establish that a calendar is legally correct.

## Calendar coverage

Saudi Arabia (`sa`) supports 2020–2037 and returns the statutory private-sector holidays under [Article 24 of the Saudi Labor Law's implementing regulations](https://www.hrsd.gov.sa/en/knowledge-centre/articles/322). Eid dates follow the Umm al-Qura calendar. Eid al-Fitr leave begins the day after 29 Ramadan, which may be before 1 Shawwal. Public-sector and banking holiday periods, and employer-specific compensatory days off, are not included.

Taiwan (`tw`) follows the [DGPA government-office calendar](https://data.gov.tw/dataset/14718) for 2020–2027, including compensatory and adjusted days off as `HolidayType::Observed`. Ordinary weekends and make-up working Saturdays are not listed. Holidays sharing a date are returned separately. This is not a private-sector employment calendar; for example, Labor Day was already a workers' holiday before it became a government-office holiday in 2026.

Outside 2020–2027, Taiwan retains its legacy calculation for compatibility. It returns only New Year's Day, Peace Memorial Day, Children's Day, National Day, the first three days of Lunar New Year, Dragon Boat Festival and Mid-Autumn Festival. It omits Lunar New Year's Eve, Tomb Sweeping Day, compensatory days and the holidays introduced by the [2025 reform](https://law.moj.gov.tw/LawClass/LawAll.aspx?pcode=D0020095). Those years are not complete official calendars.

Other countries retain the PHP package's original tables where the dates depend on announcements or bounded calendar data. For example, India's dated calendars cover 1970–2037, Nepal supports 2024–2025, and Spain's regional data covers 2022–2027. An unsupported year returns an error. The library does not invent missing announcements or extrapolate official data into later years.

## Questionable upstream results retained for compatibility

These are observable PHP results, not corrections made by this port:

- **Chile.** `Viernes Santo` and `Sábado Santo` are fixed to March 29 and March 30 for every year. For 2025 the former is a Saturday. The PHP class contains an Easter-based method, but `allHolidays` never calls it. Several other Chilean movable holidays also use fixed dates. The Rust port retains the active rules, not the unused method.
- **Mexico.** In a UTC PHP process, the 2024 Constitution Day is February 4, a Sunday. PHP first constructs the first Monday in February at UTC midnight, then converts that instant to Mexico City time, moving the calendar date back one day. The port preserves this UTC-reference result, including the analogous March and November shifts. The 2024 government-change date is September 30 with a midnight clock and October 1 with a midday clock; Rust follows the midnight reference.
- **Malaysia.** The 2024 Agong birthday is returned as October 5, and Deepavali as November 1. The [Malaysian government's 2024 schedule](https://www.kabinet.gov.my/storage/2024/11/hka_2024.pdf) lists June 3 and October 31 respectively. The birthday rule uses the first Saturday in October for 2001–2026. The original rules and bounded festival dates are retained.
- **Chinese-calendar countries.** PHP's ICU 76.1 approximation gives Chinese New Year as February 7 in 2027 rather than February 6. This affects Hong Kong, Indonesia, Korea, Malaysia and Vietnam; Taiwan uses official data for 2027 but uses the approximation outside its official-data window. For Hong Kong, PHP returns February 8, 9 and 10 after substitution, while the [gazetted 2027 calendar](https://www.gov.hk/en/about/abouthk/holiday/2027.htm) lists February 6, 8 and 9. The Rust port reproduces ICU's calculation algorithm instead of inserting corrected date tables. A newer ICU algorithm can produce different answers, so this is not delegated to the installed ICU version.
- **Bulgaria.** The source describes its substitute-holiday rule as applying since 2017 but applies it to all years. PHP returns substitute days on May 2, December 27 and December 28 in 2016. The port retains that behavior; historical results before 2017 need review.
- **Orthodox Easter countries.** The shared source helper adds the Julian Easter offset to Gregorian April 3 every year. It does not adjust that base for the changing Julian/Gregorian offset across centuries. Historical and far-future results inherit that limitation.

These issues should be resolved as separately reviewed changes to the PHP and Rust packages, with official sources and explicit compatibility notes.

## Deliberate Rust differences

- Holiday dates are date-only values. They do not carry Carbon's incidental time-of-day, default PHP timezone or mutable calendar object. Results match the documented UTC/midnight reference; they cannot reproduce every ambient-time-dependent PHP outcome. For example, PHP returns Greece's 2024 May Day on May 7 with a midnight clock but May 1 with a midday clock: one Carbon value is at midnight and another retains the current clock time, so their equality comparison changes. Rust compares dates and returns May 7.
- Same-day holidays retain their source rule order. Their order can differ from PHP's timestamp ordering. All duplicate holidays remain present.
- In Malaysia for 1900, PHP's ICU formatter and PHP's timezone database disagree on Kuala Lumpur's offset. The formatter creates 1900-01-30 17:04:35 UTC for lunar New Year; PHP converts it to January 30 at 23:51:21, while the date-only rule yields January 31. A separate 248,637-case run covering 1900–2100 found 34 mismatches, all this one historical year across Malaysia's default/unknown locale and its 17 regional configurations. No date-table override was added to reproduce the timezone disagreement.
- Recoverable failures are typed `Result` values. Missing holiday names are `Option` values. Gregorian query years are limited to 1–9999, with country-specific limits checked when calculating.
- Upcoming queries stop at year 9999 and return an error if they cannot collect enough holidays. PHP has no finite termination bound for an empty calendar. A requested count of zero returns an empty vector.
- PHP inheritance, runtime registry mutation and Carbon's natural-language date parsing are not emulated. Use country codes and typed `Date` inputs, or the explicit ISO string range method.
