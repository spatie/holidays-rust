//! Differential fingerprints produced by the real, pinned PHP implementation.
use serde::Deserialize;
use sha2::{Digest, Sha256};
use spatie_holidays::Holidays;

#[derive(Deserialize)]
struct Fixture {
    reference: String,
    years: Vec<i32>,
    rows: Vec<Row>,
}

#[derive(Debug, Deserialize)]
struct Row {
    country: String,
    region: Option<String>,
    locale: Option<String>,
    successful_years: usize,
    sha256: String,
}

#[derive(Deserialize)]
struct Correction {
    country: String,
    region: Option<String>,
    locale: Option<String>,
    sha256: String,
    reason: String,
}

#[test]
fn checks_every_calendar_against_the_reference_and_sourced_corrections() {
    let fixture: Fixture = serde_json::from_str(include_str!("php-parity.json")).unwrap();
    let corrections: Vec<Correction> =
        serde_json::from_str(include_str!("calendar-corrections.json")).unwrap();
    assert_eq!(
        fixture.reference,
        "9557c35f121e27f473bcd6c8958826a205ffa449"
    );
    let mut cases = 0;
    let mut failures = Vec::new();
    let mut corrected_groups = 0;
    for row in &fixture.rows {
        let correction = corrections.iter().find(|correction| {
            correction.country == row.country
                && correction.region == row.region
                && correction.locale == row.locale
        });
        let expected = if let Some(correction) = correction {
            assert!(!correction.reason.is_empty());
            assert_ne!(correction.sha256, row.sha256);
            corrected_groups += 1;
            &correction.sha256
        } else {
            &row.sha256
        };
        let mut hash = Sha256::new();
        let mut successful_years = 0;
        for year in &fixture.years {
            let mut calendar = Holidays::for_country(&row.country, *year).unwrap();
            if let Some(region) = &row.region {
                calendar = calendar.region(region).unwrap();
            }
            if let Some(locale) = &row.locale {
                calendar = calendar.locale(locale);
            }
            match calendar.get() {
                Ok(mut holidays) => {
                    holidays.sort_by(|a, b| {
                        (a.date, &a.name, format!("{:?}", a.holiday_type), &a.region).cmp(&(
                            b.date,
                            &b.name,
                            format!("{:?}", b.holiday_type),
                            &b.region,
                        ))
                    });
                    let value = serde_json::to_value(holidays).unwrap();
                    hash.update(serde_json::to_vec(&value).unwrap());
                    successful_years += 1;
                }
                Err(_) => hash.update(b"error"),
            }
            hash.update(b"\n");
            cases += 1;
        }
        let digest = format!("{:x}", hash.finalize());
        if &digest != expected || successful_years != row.successful_years {
            failures.push(format!(
                "{row:?}: got {digest}, {successful_years} successful years"
            ));
        }
    }
    assert_eq!(corrected_groups, corrections.len(), "unused correction");
    assert!(
        failures.is_empty(),
        "{} groups differ across {cases} cases:\n{}",
        failures.len(),
        failures.join("\n")
    );
    eprintln!(
        "Checked {cases} cases across {} configurations, including {corrected_groups} sourced correction groups",
        fixture.rows.len(),
    );
}
