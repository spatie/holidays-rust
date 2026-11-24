---
title: Adding a country
weight: 6
---

This is a community driven package. If you find any errors, please create a pull request with the fix, or at least open an issue.

## Basic country

Add a module under `src/countries`. Follow a neighboring country's rules, using the shared date, Easter and observed-day calculations. Return `Holiday` values with the original names, classifications and region codes. Keep date rules as calculations; do not replace a rule with a list of dates calculated for a few selected years.

```rust
# use spatie_holidays::{Date, Holiday, HolidayType};
# let year = 2024;
let holiday = Holiday {
    name: "New Year's Day".into(),
    date: Date::from_ymd_opt(year, 1, 1).unwrap(),
    holiday_type: HolidayType::National,
    region: None,
};
# assert_eq!(holiday.name, "New Year's Day");
```

The internal `calendar` module owns Easter, Orthodox Easter, relative weekdays, multi-day periods and substitute-day helpers. Chinese calendar conversion uses ICU4X's traditional Chinese calendar. Islamic civil conversion uses the tabular Islamic calendar. Some holidays depend on official announcements; retain those dated source records and reject years where the required data is unavailable.

## Register your country

Add the ISO country code to `COUNTRIES` in `src/countries/mod.rs` and add its calculation branch. Keep aliases separate from the country's canonical code. Unknown codes must still return `Error::InvalidCountry`.

## Regional holidays

Add accepted region codes to the same registry's `regions` function. Keep exact spelling and case. Specify whether selecting no region returns national holidays only or combines regional calendars; existing countries differ. Attach the applicable region to regional `Holiday` values, without inferring it from the active query when the rule specifies another value.

## Observed holidays

Use the existing observed-day helpers where the legal rule agrees with them. A Saturday, a Sunday and a collision with another holiday can require different substitutions. Include all of these in tests rather than checking only a year with weekday holidays.

## Holiday types

The types are `National`, `Regional`, `Observed`, `Religious` and `Bank`. Preserve the source's classification. Do not turn every religious date into a public holiday without checking the country's calendar.

## Translations

Translations are in `src/translations.json`, grouped by canonical country code and locale. Use the original holiday name as the key. Missing translations keep the original name. Update the default-locale selection only when that is the country's documented behavior.

## Testing

Add public API cases for the rule's boundaries. A new rule needs an official source and a test that distinguishes it from the previous rule. Run the quality checks in [testing](testing.md) on both stable Rust and the minimum supported version.
