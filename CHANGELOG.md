# Changelog

All notable changes to `spatie-holidays` will be documented in this file.

## 1.0.1 - 2026-11-24

- Correct Chile's Easter holidays, weekday shifts, conditional extra days and winter-solstice holiday.
- Correct Mexico's civil dates, reform boundaries and presidential transition date.
- Correct Malaysia's Agong birthday and 2023/2024 Deepavali dates.
- Use ICU4X's traditional Chinese calendar and account for century changes in Orthodox Easter.
- Apply Bulgaria's recurring substitute-day rule from 2017 and retain Greece's 2024 May Day regression.
- Keep source-backed corrections in internal tests and coverage limits in the usage guide.

## 1.0.0 - 2026-11-24

- Native Rust port of all 104 PHP country implementations, 435 region codes and 50 translation files.
- Date ranges, translated lookup, holiday classifications, upcoming holidays and long-weekend grouping.
- Differential verification against the PHP reference across 247,400 cases from 1901 through 2100.
- Documented upstream calendar anomalies and date-only compatibility differences.
