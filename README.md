# Calculate public holidays for a country

[![Latest version on crates.io](https://img.shields.io/crates/v/spatie-holidays.svg?style=flat-square)](https://crates.io/crates/spatie-holidays)
[![Tests](https://img.shields.io/github/actions/workflow/status/spatie/holidays-rust/ci.yml?branch=main&style=flat-square)](https://github.com/spatie/holidays-rust/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](https://github.com/spatie/holidays-rust/blob/main/LICENSE.md)
[![Total downloads](https://img.shields.io/crates/d/spatie-holidays.svg?style=flat-square)](https://crates.io/crates/spatie-holidays)

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

This is a native Rust port of [spatie/holidays](https://github.com/spatie/holidays). It contains all 104 country implementations, 105 registered codes, regional rules and 50 translation files from the PHP reference. Calculated dates remain calculations; bounded source calendars retain their original data and supported-year limits. PHP and system ICU are not runtime requirements.

## Support us

[<img src="https://spatie.be/github-ad-image/holidays-rust.jpg" alt="Support Spatie's open source work" width="419px" />](https://spatie.be/github-ad-click/holidays-rust)

We invest a lot of resources into creating [best in class open source packages](https://spatie.be/open-source). You can support us by [buying one of our paid products](https://spatie.be/open-source/support-us).

We highly appreciate you sending us a postcard from your hometown, mentioning which of our package(s) you are using. You'll find our address on [our contact page](https://spatie.be/about-us). We publish all received postcards on [our virtual postcard wall](https://spatie.be/open-source/postcards).

## Installation

Requires Rust 1.89 or newer.

```sh
cargo add spatie-holidays
```

## Supported countries

We support the countries listed in [this directory](https://github.com/spatie/holidays-rust/tree/main/src/countries). `Holidays::countries()` lists every accepted code. Greece accepts both `gr` and `el`.

Countries that support regions: Australia, Bosnia and Herzegovina, Brazil, France, Germany, Malaysia, Portugal, Spain, Switzerland. Portugal supports all 308 municipalities.

Some calendars are limited to particular years or omit categories of leave. Read the [calendar coverage and compatibility notes](https://spatie.be/docs/holidays-rust/v1/compatibility) before relying on results. The port preserves upstream behavior, including documented questionable rules; it is not an independent legal authority.

## Usage

```rust
# use spatie_holidays::{Error, Holidays};
# fn main() -> Result<(), Error> {
let holidays = Holidays::for_country("de", 2024)?
    .region("DE-BW")?
    .locale("en")
    .get()?;

for holiday in holidays {
    println!("{}: {}", holiday.date, holiday.name);
}
# Ok(())
# }
```

See the [usage guide](https://spatie.be/docs/holidays-rust/v1/usage) for date ranges, translations, holiday types, upcoming holidays and long weekends.

## Testing

```sh
cargo test --locked --all-targets --all-features
cargo test --locked --doc --all-features
```

See [all quality checks](https://github.com/spatie/holidays-rust/blob/main/docs/testing.md). The README and Markdown guides are tested by rustdoc, including setup on hidden `# ` lines.

## Changelog

See [CHANGELOG.md](https://github.com/spatie/holidays-rust/blob/main/CHANGELOG.md).

## Contributing

See [CONTRIBUTING.md](https://github.com/spatie/holidays-rust/blob/main/CONTRIBUTING.md) and the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

Using an AI coding agent? Load Apollo's [rust-best-practices skill](https://github.com/apollographql/skills/tree/main/skills/rust-best-practices) when writing or reviewing code in this package.

## Security Vulnerabilities

Report vulnerabilities privately using the instructions in [SECURITY.md](https://github.com/spatie/holidays-rust/blob/main/SECURITY.md).

## Credits

- [Niels Vanpachtenbeke](https://github.com/Nielsvanpach)
- [Freek Van der Herten](https://github.com/freekmurze)
- [PHP package contributors](https://github.com/spatie/holidays/graphs/contributors)
- [All contributors](https://github.com/spatie/holidays-rust/graphs/contributors)

## License

The MIT License (MIT). See [LICENSE.md](https://github.com/spatie/holidays-rust/blob/main/LICENSE.md).

The Chinese calendar calculation adapts ICU's astronomical algorithms under the [Unicode License](https://github.com/spatie/holidays-rust/blob/main/UNICODE-LICENSE.txt).
