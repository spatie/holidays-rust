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

This is a native Rust port of [spatie/holidays](https://github.com/spatie/holidays), including every country, regional rule and translation in the pinned PHP reference. It calculates relative dates, Easter, lunar holidays and observed days using rules. Calendars whose PHP implementations use official year-specific data retain those same limits.

Continue with [installation](installation-and-setup.md) and [usage](usage.md). The [compatibility notes](compatibility.md) record the PHP reference, verification, supported-year limits and questionable upstream behavior.
