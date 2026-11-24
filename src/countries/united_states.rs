use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = [self.fixed_holidays(year)?, self.variable_holidays(year)?].concat();
        if (year) >= (2021) {
            holidays.push(Holiday {
                name: "Juneteenth National Independence Day".to_owned(),
                date: self.observed(format!("{}-06-19", year))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn fixed_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "New Year's Day".to_owned(),
                date: self.observed(format!("{}-01-01", year))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Independence Day".to_owned(),
                date: self.observed(format!("{}-07-04", year))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Veterans Day".to_owned(),
                date: self.observed(format!("{}-11-11", year))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Christmas".to_owned(),
                date: self.observed(format!("{}-12-25", year))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "Martin Luther King Day".to_owned(),
                date: parse_date(&(format!("third monday of January {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Presidents' Day".to_owned(),
                date: parse_date(&(format!("third monday of February {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Memorial Day".to_owned(),
                date: parse_date(&(format!("last monday of May {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Labor Day".to_owned(),
                date: parse_date(&(format!("first monday of September {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Columbus Day".to_owned(),
                date: parse_date(&(format!("second monday of October {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Thanksgiving".to_owned(),
                date: parse_date(&(format!("fourth thursday of November {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn observed(&self, date_string: String) -> Result<Date, Error> {
        let date = parse_date(&(date_string.clone()))?;
        Ok(match () {
            _ if ((date).weekday().num_days_from_sunday() as i32) == (6) => {
                (date) + Duration::days(i64::from(-(1)))
            }
            _ if ((date).weekday().num_days_from_sunday() as i32) == (0) => {
                (date) + Duration::days(i64::from(1))
            }
            _ => date,
        })
    }
}
