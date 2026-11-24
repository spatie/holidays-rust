---
title: Usage
weight: 4
---

You can get all holidays for a country by using the `get` method. It returns a vector of `Holiday` values.

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Error> {
let holidays = Holidays::for_current_year("be")?.get()?;

for holiday in holidays {
    println!("{}: {} ({:?})", holiday.date, holiday.name, holiday.holiday_type);
}
# Ok(())
# }
```

Pass an [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2#Officially_assigned_code_elements) code to select a country. For Greece, use `gr`. The EU abbreviation `el` remains supported as an alias, and both codes are case-insensitive. `Holidays::countries()` returns all registered codes.

`Holiday` implements Serde's `Serialize` and `Deserialize`:

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let holidays = Holidays::for_country("be", 2024)?.get()?;
let json = serde_json::to_string(&holidays[0])?;
# assert_eq!(json, r#"{"name":"Nieuwjaar","date":"2024-01-01","type":"national","region":null}"#);
// {"name":"Nieuwjaar","date":"2024-01-01","type":"national","region":null}
# Ok(())
# }
```

### Getting holidays for a specific year

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Error> {
let holidays = Holidays::for_country("be", 2024)?.get()?;
# assert_eq!(holidays.len(), 10);
# Ok(())
# }
```

Years use the Gregorian calendar. Each country retains its PHP supported-year limits; unavailable data returns an error rather than an incomplete substitute calendar.

### Getting holidays between two dates

The `get_in_range` method returns all holidays between two dates (inclusive). Dates are swappable; the lower date is always used as the start. Use `get_in_range_str` for the PHP shorthand formats:

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Error> {
let holidays = Holidays::for_country("be", 2024)?
    .get_in_range_str("2024-01-01", "2024-12-31")?;
# assert_eq!(holidays.len(), 10);

let calendar = Holidays::for_country("be", 2024)?;
let full_years = calendar.get_in_range_str("2024", "2025")?;
let year_month = calendar.get_in_range_str("2024-06", "2025-05")?;
# assert_eq!(full_years.len(), 20);
# assert_eq!(year_month.len(), 9);
# Ok(())
# }
```

`get_in_range(from, to)` accepts `Date` values. A range can cross years, independently of the year used to construct the calendar. Partial starts expand to the first day; partial ends expand to the last day, before reversed endpoints are swapped.

### Getting holidays in a specific language

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Error> {
let holidays = Holidays::for_country("be", 2024)?.locale("fr").get()?;
# assert_eq!(holidays[0].name, "Jour de l'An");
# Ok(())
# }
```

If no translation file exists for the given locale, the original holiday names are returned. Omitting the locale preserves the country's default locale, where the PHP package defines one. All 50 original translation files are included.

### Regional holidays

Some countries have region-specific holidays. You can pass a region code to the `region` method:

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Error> {
let holidays = Holidays::for_country("de", 2024)?.region("DE-BW")?.get()?;
# assert!(holidays.iter().any(|h| h.name == "Heilige Drei Könige"));
# Ok(())
# }
```

To discover which regions a country supports:

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Error> {
let regions = Holidays::for_country("de", 2024)?.regions();
# assert_eq!(&regions[..3], ["DE-BW", "DE-BY", "DE-BE"]);
// ["DE-BW", "DE-BY", "DE-BE", ...]
# Ok(())
# }
```

Portugal uses municipal holidays. Its regions are the municipalities, identified by `PT-` followed by their DICO code (distrito + concelho) as published by the INE:

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Error> {
let holidays = Holidays::for_country("pt", 2024)?.region("PT-1106")?.get()?;
# assert!(holidays.iter().any(|h| h.name == "Santo António"));
// Lisboa
# Ok(())
# }
```

All 308 municipalities are supported. These are municipal DICO codes, not ISO 3166-2 district codes. Without a region, only national holidays are returned. Holidays applying to the whole autonomous region of Madeira or the Azores are not currently included.

Countries that support regions: Australia, Bosnia and Herzegovina, Brazil, France, Germany, Malaysia, Portugal, Spain, Switzerland. Region codes are case-sensitive, as in PHP. Countries without regional calendars ignore a supplied region. Malaysia without a region includes and deduplicates all its regional holidays, matching PHP.

### Determining if a date is a holiday

```rust
# use spatie_holidays::{Date, Error, Holidays};
# fn main() -> Result<(), Error> {
let calendar = Holidays::for_country("be", 2024)?;
# let date = Date::from_ymd_opt(2024, 1, 1).unwrap();
assert!(calendar.is_holiday(date)?);
# Ok(())
# }
```

### Determining if today is a holiday

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Error> {
let is_holiday = Holidays::for_current_year("be")?.is_today_holiday()?;
# let _ = is_holiday;
# Ok(())
# }
```

Today and the current year use the system's local timezone.

### Getting upcoming holidays

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Error> {
let holidays = Holidays::for_current_year("be")?.get_upcoming(3)?;

for holiday in holidays {
    println!("{}: {}", holiday.date, holiday.name);
}
# Ok(())
# }
```

This returns the next three holidays, including today if it is a holiday. `get_upcoming_from(date, count)` uses an explicit starting date. Both operations can cross years and ignore the configured year, as PHP does. A count of zero returns an empty vector.

### Getting long weekends

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Error> {
let calendar = Holidays::for_country("de", 2024)?;
let long_weekends = calendar.get_long_weekends(4)?;
let shorter_weekends = calendar.get_long_weekends(3)?;

for weekend in long_weekends {
    println!("{} to {}: {} days", weekend.start_date, weekend.end_date, weekend.day_count);
    println!("{:?}", weekend.holidays);
}
# let _ = shorter_weekends;
# Ok(())
# }
```

This preserves PHP's grouping algorithm. It groups successive holidays separated by no more than the supplied minimum days, then keeps groups with at least two holidays whose inclusive span meets that minimum. It does not calculate which weekends or working days require time off.

### Getting the name of a holiday

```rust
# use spatie_holidays::{Date, Error, Holidays};
# fn main() -> Result<(), Error> {
# let date = Date::from_ymd_opt(2024, 1, 1).unwrap();
let name = Holidays::for_country("be", 2024)?.get_name(date)?;
assert_eq!(name.as_deref(), Some("Nieuwjaar"));
# Ok(())
# }
```

Date lookup uses the date's own year, independently of the configured year. An ordinary non-holiday returns `None`, not an error.

### Determining whether a country is supported

```rust
# use spatie_holidays::Holidays;
assert!(Holidays::has("be"));
assert!(!Holidays::has("unknown"));
```

### Holiday types

The holiday types are `National`, `Regional`, `Observed`, `Religious` and `Bank`:

```rust
# use spatie_holidays::{Error, HolidayType, Holidays};
# fn main() -> Result<(), Error> {
let holidays = Holidays::for_country("de", 2024)?
    .region("DE-BW")?
    .get_by_type(HolidayType::Regional)?;
# assert!(holidays.iter().all(|h| h.holiday_type == HolidayType::Regional));
# Ok(())
# }
```

### Errors and Rust API differences

Configuration uses consuming builders. Keep the returned `Holidays` value or chain the calls. Unknown countries and invalid regions return typed errors. Supported-year checks happen when holidays are calculated, since different countries have different limits. Date inputs are typed `Date` values; string range endpoints accept ISO year, month or date formats rather than Carbon's natural-language parser.

PHP optional arguments map to explicit Rust operations: `for_current_year` or `for_country`, `get` or `get_by_type`, and `get_upcoming` or `get_upcoming_from`. `get_long_weekends` takes its minimum explicitly; pass `4` for PHP's default. `Holiday` fields are owned values rather than shared mutable objects. Custom PHP country subclasses and runtime registry mutation are not Rust extension mechanisms; see [adding a country](adding-a-country.md) for repository contributions.
