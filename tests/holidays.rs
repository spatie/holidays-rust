//! Public query, configuration, serialization and boundary contracts.
use spatie_holidays::{Date, Error, Holiday, HolidayType, Holidays};

fn date(value: &str) -> Date {
    Date::parse_from_str(value, "%Y-%m-%d").unwrap()
}

#[test]
fn exposes_all_registered_countries_and_regions() {
    assert_eq!(Holidays::countries().len(), 105);
    let region_count: usize = Holidays::countries()
        .iter()
        .map(|code| Holidays::for_country(code, 2024).unwrap().regions().len())
        .sum();
    assert_eq!(region_count, 435);
    assert_eq!(
        Holidays::for_country("pt", 2024).unwrap().regions().len(),
        308
    );
    assert_eq!(
        Holidays::for_country("GR", 2024).unwrap().get().unwrap(),
        Holidays::for_country("el", 2024).unwrap().get().unwrap()
    );
}

#[test]
fn validates_configuration_without_hiding_unavailable_years() {
    assert!(matches!(
        Holidays::for_country("unknown", 2024),
        Err(Error::InvalidCountry(_))
    ));
    for year in [0, 10_000] {
        assert!(matches!(
            Holidays::for_country("be", year),
            Err(Error::UnsupportedYear)
        ));
    }
    assert!(matches!(
        Holidays::for_country("de", 2024).unwrap().region("de-bw"),
        Err(Error::InvalidRegion(_))
    ));
    assert_eq!(
        Holidays::for_country("be", 2024)
            .unwrap()
            .region("ignored")
            .unwrap()
            .get()
            .unwrap()
            .len(),
        10
    );
    for year in [1969, 2038] {
        assert!(matches!(
            Holidays::for_country("tr", year).unwrap().get(),
            Err(Error::UnsupportedYear)
        ));
    }
    assert!(Holidays::for_country("tr", 1970).unwrap().get().is_ok());
    assert!(Holidays::for_country("tr", 2037).unwrap().get().is_ok());
}

#[test]
fn expands_partial_dates_and_swaps_inclusive_ranges_like_php() {
    let calendar = Holidays::for_country("be", 1980).unwrap();
    for (from, to, count, first, last) in [
        ("2020", "2024", 50, "Nieuwjaar", "Kerstmis"),
        (
            "2024-06",
            "2025-05",
            9,
            "Nationale Feestdag",
            "OLH Hemelvaart",
        ),
        (
            "2023-06-01",
            "2024-05-30",
            10,
            "Nationale Feestdag",
            "Pinkstermaandag",
        ),
        (
            "2024-05-30",
            "2023-06-01",
            10,
            "Nationale Feestdag",
            "Pinkstermaandag",
        ),
        ("2024-12-25", "2024-12-25", 1, "Kerstmis", "Kerstmis"),
        ("2025", "2024", 1, "Nieuwjaar", "Nieuwjaar"),
    ] {
        let holidays = calendar.get_in_range_str(from, to).unwrap();
        assert_eq!(
            (
                holidays.len(),
                holidays.first().unwrap().name.as_str(),
                holidays.last().unwrap().name.as_str()
            ),
            (count, first, last),
            "{from}..{to}"
        );
    }
    for invalid in ["2024-02-30", "2024-00", "tomorrow", "2024-13"] {
        assert!(matches!(
            calendar.get_in_range_str(invalid, "2025"),
            Err(Error::InvalidDate(_))
        ));
    }
    assert!(
        calendar
            .get_in_range_str("2024-01-02", "2024-01-03")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn translates_names_and_looks_up_the_dates_own_year() {
    let calendar = Holidays::for_country("be", 1980).unwrap().locale("fr");
    assert_eq!(
        calendar.get_name(date("2024-01-01")).unwrap().as_deref(),
        Some("Jour de l'An")
    );
    assert_eq!(calendar.get_name(date("2024-01-02")).unwrap(), None);
    assert!(calendar.is_holiday(date("2024-01-01")).unwrap());
    assert!(!calendar.is_holiday(date("2024-01-02")).unwrap());
    assert_eq!(
        Holidays::for_country("be", 2024)
            .unwrap()
            .locale("unknown")
            .get()
            .unwrap()[0]
            .name,
        "Nieuwjaar"
    );
}

#[test]
fn filters_classifications_without_dropping_duplicate_dates() {
    let calendar = Holidays::for_country("de", 2024)
        .unwrap()
        .region("DE-BW")
        .unwrap();
    let regional = calendar.get_by_type(HolidayType::Regional).unwrap();
    assert!(regional.iter().any(|h| h.name == "Heilige Drei Könige"));
    assert!(
        regional
            .iter()
            .all(|h| h.holiday_type == HolidayType::Regional)
    );
    let holidays = Holidays::for_country("tw", 2025).unwrap().get().unwrap();
    assert!(
        holidays
            .windows(2)
            .any(|pair| pair[0].date == pair[1].date && pair[0].name != pair[1].name)
    );
}

#[test]
fn includes_the_starting_holiday_and_crosses_years_for_upcoming_queries() {
    let calendar = Holidays::for_country("be", 1980).unwrap();
    let holidays = calendar.get_upcoming_from(date("2024-12-25"), 3).unwrap();
    assert_eq!(
        holidays.iter().map(|h| h.date).collect::<Vec<_>>(),
        [date("2024-12-25"), date("2025-01-01"), date("2025-04-21")]
    );
    assert!(
        calendar
            .get_upcoming_from(date("2024-12-25"), 0)
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        calendar.get_upcoming_from(date("9999-12-25"), 1).unwrap()[0].name,
        "Kerstmis"
    );
    assert!(matches!(
        calendar.get_upcoming_from(date("9999-12-26"), 1),
        Err(Error::UnsupportedYear)
    ));
}

#[test]
fn groups_long_weekends_and_serializes_php_field_names() {
    let weekends = Holidays::for_country("de", 2024)
        .unwrap()
        .get_long_weekends(4)
        .unwrap();
    assert_eq!(weekends.len(), 1);
    let weekend = &weekends[0];
    assert_eq!(
        (weekend.start_date, weekend.end_date, weekend.day_count),
        (date("2024-03-29"), date("2024-04-01"), 4)
    );
    assert_eq!(weekend.holidays.len(), 2);
    let json = serde_json::to_value(weekend).unwrap();
    assert_eq!(json["startDate"], "2024-03-29");
    assert_eq!(json["dayCount"], 4);
    assert!(json.get("start_date").is_none());
    let holiday = &weekend.holidays[0];
    assert_eq!(
        serde_json::from_value::<Holiday>(serde_json::to_value(holiday).unwrap()).unwrap(),
        *holiday
    );
}

#[test]
fn uses_date_only_lunar_rules_without_historical_timezone_database_disagreement() {
    let holidays = Holidays::for_country("my", 1900).unwrap().get().unwrap();
    let new_year = holidays
        .iter()
        .find(|h| h.name == "Tahun Baru Cina")
        .unwrap();
    assert_eq!(new_year.date, date("1900-01-31"));
}
