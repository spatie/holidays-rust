use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "New Year's Day".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Canada Day".to_owned(),
                    date: parse_date(&(format!("{}-07-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Civic Holiday".to_owned(),
                    date: parse_date(&(format!("first monday of August {}", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Labour Day".to_owned(),
                    date: parse_date(&(format!("first monday of September {}", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "National Day for Truth and Reconciliation".to_owned(),
                    date: parse_date(&(format!("{}-09-30", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Remembrance Day".to_owned(),
                    date: parse_date(&(format!("{}-11-11", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Christmas Day".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Boxing Day".to_owned(),
                    date: parse_date(&(format!("{}-12-26", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        let victoria_day = date_previous(super::date(year, 5, 25)?, "Monday".to_owned())?;
        Ok(vec![
            Holiday {
                name: "Victoria Day".to_owned(),
                date: victoria_day,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Good Friday".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Easter Monday".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Thanksgiving".to_owned(),
                date: parse_date(&(format!("second monday of October {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
