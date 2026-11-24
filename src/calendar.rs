use crate::{Date, Error, Holiday, HolidayType};
use chrono::{Datelike, Duration};
use indexmap::IndexMap;

pub(crate) type Table = IndexMap<i32, Vec<String>>;

pub(crate) fn date(year: i32, month: i32, day: i32) -> Result<Date, Error> {
    let base = Date::from_ymd_opt(year, month as u32, 1)
        .ok_or_else(|| Error::InvalidDate(format!("{year}-{month}-{day}")))?;
    base.checked_add_signed(Duration::days(i64::from(day) - 1))
        .ok_or(Error::UnsupportedYear)
}

pub(crate) fn range_date(input: &str, end: bool) -> Result<Date, Error> {
    let full = match input.len() {
        4 => format!("{input}-{}", if end { "12-31" } else { "01-01" }),
        7 => {
            let first = Date::parse_from_str(&format!("{input}-01"), "%Y-%m-%d")
                .map_err(|_| Error::InvalidDate(input.into()))?;
            if end {
                let (y, m) = if first.month() == 12 {
                    (first.year() + 1, 1)
                } else {
                    (first.year(), first.month() + 1)
                };
                return Ok(date(y, m as i32, 1)? - Duration::days(1));
            }
            return Ok(first);
        }
        10 => input.into(),
        _ => return Err(Error::InvalidDate(input.into())),
    };
    Date::parse_from_str(&full, "%Y-%m-%d").map_err(|_| Error::InvalidDate(input.into()))
}

fn weekday(input: &str) -> Result<i32, Error> {
    [
        "sunday",
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
    ]
    .iter()
    .position(|d| *d == input.to_ascii_lowercase())
    .map(|d| d as i32)
    .ok_or_else(|| Error::InvalidDate(input.into()))
}

pub(crate) fn parse_date(input: &str) -> Result<Date, Error> {
    let lower = input.to_ascii_lowercase();
    let words: Vec<_> = lower.split_whitespace().collect();
    if words.len() == 5 && words[2] == "of" {
        let year = words[4]
            .parse()
            .map_err(|_| Error::InvalidDate(input.into()))?;
        let month = [
            "january",
            "february",
            "march",
            "april",
            "may",
            "june",
            "july",
            "august",
            "september",
            "october",
            "november",
            "december",
        ]
        .iter()
        .position(|m| *m == words[3])
        .ok_or_else(|| Error::InvalidDate(input.into()))? as i32
            + 1;
        let target = weekday(words[1])?;
        if words[0] == "last" {
            let next = if month == 12 {
                date(year + 1, 1, 1)?
            } else {
                date(year, month + 1, 1)?
            };
            return date_previous(next, target);
        }
        let n = match words[0] {
            "first" => 0,
            "second" => 1,
            "third" => 2,
            "fourth" => 3,
            "fifth" => 4,
            _ => return Err(Error::InvalidDate(input.into())),
        };
        return Ok(date_first_of_month(date(year, month, 1)?, target)? + Duration::days(n * 7));
    }
    if words.len() == 3 && matches!(words[0], "next" | "previous" | "last") {
        let base = parse_date(words[2])?;
        return if words[0] == "next" {
            date_next(base, words[1])
        } else {
            date_previous(base, words[1])
        };
    }
    let parts: Vec<_> = input.split('-').collect();
    if parts.len() == 3
        && let (Ok(y), Ok(m), Ok(d)) = (parts[0].parse(), parts[1].parse(), parts[2].parse())
    {
        return date(y, m, d);
    }
    Err(Error::InvalidDate(input.into()))
}

pub(crate) trait WeekdayInput {
    fn weekday_number(self) -> Result<i32, Error>;
}
impl WeekdayInput for i32 {
    fn weekday_number(self) -> Result<i32, Error> {
        Ok(self)
    }
}
impl WeekdayInput for String {
    fn weekday_number(self) -> Result<i32, Error> {
        weekday(&self)
    }
}
impl WeekdayInput for &str {
    fn weekday_number(self) -> Result<i32, Error> {
        weekday(self)
    }
}
pub(crate) fn date_next(day: Date, target: impl WeekdayInput) -> Result<Date, Error> {
    let delta =
        (target.weekday_number()? - day.weekday().num_days_from_sunday() as i32).rem_euclid(7);
    Ok(day + Duration::days(i64::from(if delta == 0 { 7 } else { delta })))
}
pub(crate) fn date_previous(day: Date, target: impl WeekdayInput) -> Result<Date, Error> {
    let delta =
        (day.weekday().num_days_from_sunday() as i32 - target.weekday_number()?).rem_euclid(7);
    Ok(day - Duration::days(i64::from(if delta == 0 { 7 } else { delta })))
}
pub(crate) fn date_modify(day: Date, target: String) -> Result<Date, Error> {
    let target = weekday(&target)?;
    Ok(day
        + Duration::days(i64::from(
            (target - day.weekday().num_days_from_sunday() as i32).rem_euclid(7),
        )))
}
pub(crate) fn date_first_of_month(day: Date, target: impl WeekdayInput) -> Result<Date, Error> {
    date_next(
        date(day.year(), day.month() as i32, 1)? - Duration::days(1),
        target,
    )
}
pub(crate) fn date_start_of_week(day: Date) -> Result<Date, Error> {
    Ok(day - Duration::days(i64::from(day.weekday().num_days_from_monday())))
}
pub(crate) fn date_end_of_week(day: Date) -> Result<Date, Error> {
    Ok(date_start_of_week(day)? + Duration::days(6))
}
pub(crate) fn date_is(day: Date, target: &str) -> bool {
    let dow = day.weekday().num_days_from_sunday() as i32;
    match target {
        "weekend" => dow == 0 || dow == 6,
        "weekday" => dow != 0 && dow != 6,
        _ => weekday(target).is_ok_and(|w| w == dow),
    }
}
pub(crate) fn easter(year: i32, orthodox: bool) -> Result<Date, Error> {
    // Convert Julian Easter to the public Gregorian date, including century changes.
    if orthodox || year <= 1752 {
        let a = year.rem_euclid(4);
        let b = year.rem_euclid(7);
        let c = year.rem_euclid(19);
        let d = (19 * c + 15).rem_euclid(30);
        let e = (2 * a + 4 * b - d + 34).rem_euclid(7);
        let offset = if orthodox {
            year / 100 - year / 400 - 2
        } else {
            0
        };
        return Ok(date(year, 3, 22)? + Duration::days(i64::from(d + e + offset)));
    }
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let x = h + l - 7 * m + 114;
    date(year, x / 31, x % 31 + 1)
}

pub(crate) fn table_value(table: &Table, year: i32) -> Result<String, Error> {
    table
        .get(&year)
        .and_then(|v| v.first())
        .cloned()
        .ok_or(Error::UnsupportedYear)
}
pub(crate) fn get_single_day_holiday(table: &Table, year: i32) -> Result<Date, Error> {
    parse_date(&format!("{year}-{}", table_value(table, year)?))
}
pub(crate) fn get_multi_day_holiday(
    table: &Table,
    year: i32,
    total_days: i32,
) -> Result<Vec<Vec<Date>>, Error> {
    let Some(current) = table.get(&year) else {
        return Ok(Vec::new());
    };
    let mut starts = Vec::new();
    if table.keys().min().is_some_and(|min| year > *min) {
        let previous = table
            .get(&(year - 1))
            .and_then(|v| v.last())
            .ok_or(Error::UnsupportedYear)?;
        let same_year = parse_date(&format!("{year}-{previous}"))?;
        if (same_year + Duration::days(i64::from(total_days - 1))).year() != year {
            starts.push(parse_date(&format!("{}-{previous}", year - 1))?);
        }
    }
    for md in current {
        starts.push(parse_date(&format!("{year}-{md}"))?);
    }
    Ok(starts
        .into_iter()
        .map(|start| {
            (0..total_days)
                .map(|i| start + Duration::days(i64::from(i)))
                .collect()
        })
        .collect())
}
pub(crate) fn convert_periods(
    name: String,
    year: i32,
    period: Vec<Date>,
    holiday_type: HolidayType,
    suffix: String,
    prefix: String,
    include_eve: bool,
) -> Vec<Holiday> {
    let mut result = Vec::new();
    if include_eve && let Some(first) = period.first() {
        let eve = *first - Duration::days(1);
        if eve.year() == year {
            result.push(Holiday {
                name: format!("{name} Eve"),
                date: eve,
                holiday_type,
                region: None,
            });
        }
    }
    for (i, date) in period.into_iter().enumerate() {
        if date.year() != year {
            continue;
        }
        let tail = if i == 0 {
            String::new()
        } else {
            format!(" {suffix} {}", i + 1)
        };
        result.push(Holiday {
            name: format!("{prefix}{name}{tail}"),
            date,
            holiday_type,
            region: None,
        });
    }
    result
}

pub(crate) fn weekend_to_next_monday(day: Date) -> Option<Date> {
    date_is(day, "weekend")
        .then(|| date_next(day, 1).ok())
        .flatten()
}
pub(crate) fn sunday_to_next_monday(day: Date) -> Option<Date> {
    date_is(day, "sunday").then_some(day + Duration::days(1))
}
pub(crate) fn observed_christmas_day(day: Date) -> Option<Date> {
    match day.weekday().num_days_from_sunday() {
        6 | 0 => Some(day + Duration::days(2)),
        _ => None,
    }
}
pub(crate) fn observed_boxing_day(day: Date) -> Option<Date> {
    observed_christmas_day(day)
}
pub(crate) fn union<K: Eq + std::hash::Hash, V>(
    mut first: IndexMap<K, V>,
    second: IndexMap<K, V>,
) -> IndexMap<K, V> {
    for (key, value) in second {
        first.entry(key).or_insert(value);
    }
    first
}

pub(crate) fn chinese_to_gregorian_date(input: &str, year: i32) -> Result<Date, Error> {
    let (month, day) = input
        .split_once('-')
        .ok_or_else(|| Error::InvalidDate(input.into()))?;
    let month = month
        .parse()
        .map_err(|_| Error::InvalidDate(input.into()))?;
    let day = day.parse().map_err(|_| Error::InvalidDate(input.into()))?;
    let converted = icu_calendar::Date::try_new_chinese_traditional(
        year,
        icu_calendar::types::Month::new(month),
        day,
    )
    .map_err(|_| Error::Calculation(input.into()))?;
    Date::from_num_days_from_ce_opt(converted.to_rata_die().to_i64_date() as i32)
        .ok_or(Error::UnsupportedYear)
}

pub(crate) fn islamic_to_gregorian_date(
    input: &str,
    year: i32,
    next_year: bool,
) -> Result<Date, Error> {
    let iso = date(year + i32::from(next_year), 1, 1)?;
    let civil = icu_calendar::cal::Hijri::new_tabular(
        icu_calendar::cal::hijri::TabularAlgorithmLeapYears::TypeII,
        icu_calendar::cal::hijri::TabularAlgorithmEpoch::Friday,
    );
    let start = icu_calendar::Date::try_new_iso(iso.year(), 1, 1)
        .map_err(|_| Error::UnsupportedYear)?
        .to_calendar(civil);
    let hijri_year = start.era_year().year;
    let (month, day) = input
        .split_once('-')
        .ok_or_else(|| Error::InvalidDate(input.into()))?;
    let converted = icu_calendar::Date::try_new_hijri_with_calendar(
        hijri_year,
        month
            .parse()
            .map_err(|_| Error::InvalidDate(input.into()))?,
        day.parse().map_err(|_| Error::InvalidDate(input.into()))?,
        civil,
    )
    .map_err(|_| Error::Calculation(input.into()))?;
    Date::from_num_days_from_ce_opt(converted.to_rata_die().to_i64_date() as i32)
        .ok_or(Error::UnsupportedYear)
}
