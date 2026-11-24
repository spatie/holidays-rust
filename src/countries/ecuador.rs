use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![Holiday {
                name: "New Year's Day".to_owned(),
                date: parse_date(&(format!("{}-01-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn nearest_day(&self, year: i32, month: i32, day: i32) -> Result<Date, Error> {
        let date = super::date(year, month, day)?;
        if (date_is(date, "Tuesday")) || (date_is(date, "Saturday")) {
            return Ok((date) + Duration::days(i64::from(-(1))));
        }
        if date_is(date, "Sunday") {
            return Ok((date) + Duration::days(i64::from(1)));
        }
        if (date_is(date, "Wednesday")) || (date_is(date, "Thursday")) {
            return date_next(date, 5);
        }
        Ok(date)
    }
    pub(super) fn get_christmas_holiday(&self, year: i32) -> Result<Date, Error> {
        if (year) == (2022) {
            let observed_christmas_day = sunday_to_next_monday(super::date(year, 12, 25)?);
            if (observed_christmas_day).is_some() {
                return (observed_christmas_day)
                    .ok_or_else(|| Error::Calculation("Ecuador".into()));
            }
        }
        super::date(year, 12, 25)
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        let ash_wednesday = (easter) + Duration::days(i64::from(-(46)));
        Ok(vec![
            Holiday {
                name: "Holy Friday".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Carnival Monday".to_owned(),
                date: (ash_wednesday) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Carnival Tuesday".to_owned(),
                date: (ash_wednesday) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Labor Day".to_owned(),
                date: self.nearest_day(year, 5, 1)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Battle of Pichincha".to_owned(),
                date: self.nearest_day(year, 5, 24)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Independence Day".to_owned(),
                date: self.nearest_day(year, 8, 10)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Independence Of Guayaquil".to_owned(),
                date: self.nearest_day(year, 10, 9)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "All Souls' Day".to_owned(),
                date: self.nearest_day(year, 11, 2)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Independence Of Cuenca".to_owned(),
                date: self.nearest_day(year, 11, 3)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Christmas".to_owned(),
                date: self.get_christmas_holiday(year)?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
