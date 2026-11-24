---
title: Installation
weight: 3
---

Requires Rust 1.89 or newer. Install Rust using [rustup](https://rustup.rs/).

You can install the package with Cargo:

```sh
cargo add spatie-holidays
```

There are no runtime requirements for PHP, Composer or a system ICU installation. Dates use `chrono::NaiveDate`, re-exported as `spatie_holidays::Date`.
