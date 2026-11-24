#![doc = include_str!("../README.md")]

mod calendar;
mod chinese;
mod countries;

pub use chrono::NaiveDate as Date;
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};

/// A failure to select a calendar or calculate its holidays.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The country code is not registered.
    #[error("Unknown country: {0}")]
    InvalidCountry(String),
    /// The country does not recognize this region code.
    #[error("Unknown region: {0}")]
    InvalidRegion(String),
    /// The selected calendar cannot calculate this year.
    #[error("The year is outside this calendar's supported range")]
    UnsupportedYear,
    /// The date is not a valid ISO calendar date.
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    /// A rule requires calendar data that is unavailable.
    #[error("Unable to calculate holiday: {0}")]
    Calculation(String),
}

/// The classification supplied by the country calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HolidayType {
    /// A national public holiday.
    National,
    /// A regional public holiday.
    Regional,
    /// A substitute or observed holiday.
    Observed,
    /// A religious holiday.
    Religious,
    /// A bank holiday.
    Bank,
}

/// A named holiday, including its original classification and region.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Holiday {
    /// The translated name, or the original name when no translation exists.
    pub name: String,
    /// The calendar date, without a time of day.
    pub date: Date,
    /// The holiday's classification.
    #[serde(rename = "type")]
    pub holiday_type: HolidayType,
    /// The region attached by the rule, if any.
    pub region: Option<String>,
}

/// A group of nearby holidays, following the PHP package's grouping semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongWeekend {
    /// The first holiday in the group.
    pub start_date: Date,
    /// The last holiday in the group.
    pub end_date: Date,
    /// The inclusive span between those holidays, including intervening days.
    pub day_count: i64,
    /// The holidays in the group.
    pub holidays: Vec<Holiday>,
}

/// A country's public holiday calendar.
#[derive(Debug, Clone)]
#[must_use]
pub struct Holidays {
    country: String,
    year: i32,
    locale: Option<String>,
    region: Option<String>,
}

impl Holidays {
    /// Select a country and year. Country codes are case-insensitive.
    ///
    /// # Errors
    /// Returns [`Error::InvalidCountry`] for an unknown country and
    /// [`Error::UnsupportedYear`] for years outside 1–9999.
    pub fn for_country(country: &str, year: i32) -> Result<Self, Error> {
        let country = country.to_ascii_lowercase();
        if !Self::has(&country) {
            return Err(Error::InvalidCountry(country));
        }
        if !(1..=9999).contains(&year) {
            return Err(Error::UnsupportedYear);
        }
        Ok(Self {
            country,
            year,
            locale: None,
            region: None,
        })
    }

    /// Select a country for the current local year.
    pub fn for_current_year(country: &str) -> Result<Self, Error> {
        Self::for_country(country, Local::now().year())
    }

    /// Whether a country code is registered.
    #[must_use]
    pub fn has(country: &str) -> bool {
        countries::COUNTRIES.contains(&country.to_ascii_lowercase().as_str())
    }

    /// All registered codes, including the `el` alias for Greece.
    #[must_use]
    pub fn countries() -> &'static [&'static str] {
        countries::COUNTRIES
    }

    /// Choose a translation. Missing locales retain the original holiday names.
    pub fn locale(mut self, locale: &str) -> Self {
        self.locale = Some(locale.to_owned());
        self
    }

    /// Choose a region using the country's exact, case-sensitive region code.
    ///
    /// # Errors
    /// Returns [`Error::InvalidRegion`] when a regional calendar rejects the code.
    /// Countries without regional calendars ignore this setting, like PHP.
    pub fn region(mut self, region: &str) -> Result<Self, Error> {
        let regions = self.regions();
        if !regions.is_empty() && !regions.iter().any(|r| r == region) {
            return Err(Error::InvalidRegion(region.to_owned()));
        }
        self.region = Some(region.to_owned());
        Ok(self)
    }

    /// Region codes accepted by this calendar.
    #[must_use]
    pub fn regions(&self) -> Vec<String> {
        countries::regions(&self.country)
    }

    /// Calculate holidays in chronological order, retaining same-day duplicates.
    ///
    /// # Errors
    /// Returns an error if a country rule does not support the selected year.
    pub fn get(&self) -> Result<Vec<Holiday>, Error> {
        self.holidays_for_year(self.year)
    }

    /// Calculate only holidays of a given classification.
    pub fn get_by_type(&self, holiday_type: HolidayType) -> Result<Vec<Holiday>, Error> {
        Ok(self
            .get()?
            .into_iter()
            .filter(|h| h.holiday_type == holiday_type)
            .collect())
    }

    fn holidays_for_year(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        if !(1..=9999).contains(&year) {
            return Err(Error::UnsupportedYear);
        }
        let mut holidays =
            countries::calculate(&self.country, self.region.as_deref().unwrap_or(""), year)?;
        countries::translate(&self.country, self.locale.as_deref(), &mut holidays);
        holidays.sort_by_key(|h| h.date);
        Ok(holidays)
    }

    /// Get holidays between inclusive dates; reversed endpoints are swapped.
    pub fn get_in_range(&self, from: Date, to: Date) -> Result<Vec<Holiday>, Error> {
        let (from, to) = if from > to { (to, from) } else { (from, to) };
        let mut holidays = Vec::new();
        for year in from.year()..=to.year() {
            holidays.extend(
                self.holidays_for_year(year)?
                    .into_iter()
                    .filter(|h| h.date >= from && h.date <= to),
            );
        }
        holidays.sort_by_key(|h| h.date);
        Ok(holidays)
    }

    /// Get holidays in a range written as `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`.
    /// Partial starts expand to the first day; partial ends to the last day.
    pub fn get_in_range_str(&self, from: &str, to: &str) -> Result<Vec<Holiday>, Error> {
        self.get_in_range(
            calendar::range_date(from, false)?,
            calendar::range_date(to, true)?,
        )
    }

    /// Whether the given date is a holiday, independently of the selected year.
    pub fn is_holiday(&self, date: Date) -> Result<bool, Error> {
        Ok(self.get_name(date)?.is_some())
    }

    /// Whether today is a holiday in the system's local timezone.
    pub fn is_today_holiday(&self) -> Result<bool, Error> {
        self.is_holiday(Local::now().date_naive())
    }

    /// The first holiday name on a date, or `None` when it is not a holiday.
    pub fn get_name(&self, date: Date) -> Result<Option<String>, Error> {
        Ok(self
            .holidays_for_year(date.year())?
            .into_iter()
            .find(|h| h.date == date)
            .map(|h| h.name))
    }

    /// Get the next `count` holidays, including today.
    pub fn get_upcoming(&self, count: usize) -> Result<Vec<Holiday>, Error> {
        self.get_upcoming_from(Local::now().date_naive(), count)
    }

    /// Get upcoming holidays from an explicit inclusive date.
    pub fn get_upcoming_from(&self, from: Date, count: usize) -> Result<Vec<Holiday>, Error> {
        let mut holidays = Vec::new();
        if count == 0 {
            return Ok(holidays);
        }
        for year in from.year()..=9999 {
            for holiday in self.holidays_for_year(year)? {
                if holiday.date >= from {
                    holidays.push(holiday);
                }
                if holidays.len() >= count {
                    return Ok(holidays);
                }
            }
        }
        Err(Error::UnsupportedYear)
    }

    /// Group consecutive holidays separated by at most `minimum_days` days.
    /// Only groups with at least two holidays and a sufficient inclusive span remain.
    /// This reproduces PHP; it does not calculate intervening working days off.
    pub fn get_long_weekends(&self, minimum_days: i64) -> Result<Vec<LongWeekend>, Error> {
        let mut groups: Vec<Vec<Holiday>> = Vec::new();
        for holiday in self.get()? {
            if let Some(group) = groups.last_mut()
                && let Some(last) = group.last()
                && (holiday.date - last.date).num_days() <= minimum_days
            {
                group.push(holiday);
                continue;
            }
            groups.push(vec![holiday]);
        }
        Ok(groups
            .into_iter()
            .filter_map(|holidays| {
                let start_date = holidays.first()?.date;
                let end_date = holidays.last()?.date;
                let day_count = (end_date - start_date).num_days() + 1;
                (holidays.len() >= 2 && day_count >= minimum_days).then_some(LongWeekend {
                    start_date,
                    end_date,
                    day_count,
                    holidays,
                })
            })
            .collect())
    }
}

#[cfg(doctest)]
mod guide {
    include!(concat!(env!("OUT_DIR"), "/documentation.rs"));
}
