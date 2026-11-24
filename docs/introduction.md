---
title: Introduction
weight: 1
---

This package can calculate public holidays for a country.

```rust
# use spatie_holidays::{Date, Error, HolidayType, Holidays};
# fn main() -> Result<(), Error> {
let holidays = Holidays::for_country("be", 2024)?.get()?;

assert_eq!(holidays[0].name, "Nieuwjaar");
assert_eq!(holidays[0].date, Date::from_ymd_opt(2024, 1, 1).unwrap());
assert_eq!(holidays[0].holiday_type, HolidayType::National);
# Ok(())
# }
```

This package supports 104 countries, regional calendars and 50 translation files. It calculates relative dates, Easter, lunar holidays and observed days using rules. Calendars that need official year-specific data have supported-year limits.

Continue with [installation](installation-and-setup.md) and [usage](usage.md), including calendar coverage and supported-year limits.
