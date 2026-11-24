---
title: Testing
weight: 7
---

Run these checks from the package directory:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo test --locked --doc --all-features
cargo doc --locked --no-deps --all-features
cargo package --locked
```

Set `RUSTDOCFLAGS` to `-D warnings` when running the documentation commands. In POSIX shells, prefix each command with `RUSTDOCFLAGS='-D warnings'`. In PowerShell, first run `$env:RUSTDOCFLAGS = '-D warnings'`.

README.md and all Markdown pages in docs/ participate in rustdoc tests. Setup lines prefixed with `# ` compile and execute but are hidden in rendered rustdoc. Assertions check the examples' results.

Check the minimum supported Rust version:

```sh
rustup toolchain install 1.89.0 --profile minimal
cargo +1.89.0 test --locked --all-targets --all-features
cargo +1.89.0 test --locked --doc --all-features
```

Check security advisories:

```sh
cargo install cargo-audit --locked
cargo audit
```

CI runs stable checks on Linux, macOS and Windows, and the MSRV and audit checks on Linux. An audit requires access to the RustSec advisory database.

The initial audit found no vulnerabilities and one informational warning: [`RUSTSEC-2026-0543`](https://rustsec.org/advisories/RUSTSEC-2026-0543) marks `chrono` as unmaintained, with no patched version available. `cargo audit` exits successfully with this warning. The library currently exposes `chrono::NaiveDate` as `Date`; changing the date library would change that public contract.

## Differential PHP tests

The checked-in `tests/php-parity.json` was produced by the real PHP package, not by the Rust implementation. It covers every registered code and region, each applicable locale, default and missing locales, and every year from 1901 through 2100. The test compares complete holiday records after normalizing same-day order. SHA-256 fingerprints keep the fixture small without shipping replacement date tables.

To reproduce the fixture, install PHP 8.4 with `intl`, `calendar` and Composer. Clone the reference outside this package directory:

```sh
git clone https://github.com/spatie/holidays.git ../holidays-php
git -C ../holidays-php checkout 9557c35f121e27f473bcd6c8958826a205ffa449
composer install --working-dir=../holidays-php --no-dev
php tools/php-parity.php ../holidays-php > tests/php-parity.json
cargo test --locked --test php_parity -- --nocapture
```

The reference fixture used ICU 76.1. A different ICU version may change lunar-calendar results; inspect such differences instead of accepting the new fixture automatically. The fixture records its upstream commit, clock, timezone and ICU version. Errors are compared as rejected calculations, not as identical PHP exception messages and Rust error text. The fixture also records successful-year counts so rejected years cannot silently replace working calendars.

The [compatibility notes](compatibility.md) describe the separate 1900 comparison, historical timezone discrepancy and questionable upstream rules. A passing parity test means compatibility with this reference, not legal validation of every country's calendar.
