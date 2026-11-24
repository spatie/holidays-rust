use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([self.fixed_holidays(year)?, self.variable_holidays(year)?].concat())
    }
    pub(super) fn fixed_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "Uudenvuodenpäivä".to_owned(),
                date: parse_date(&(format!("{}-01-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Loppiainen".to_owned(),
                date: parse_date(&(format!("{}-01-06", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Vappu".to_owned(),
                date: parse_date(&(format!("{}-05-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Itsenäisyyspäivä".to_owned(),
                date: parse_date(&(format!("{}-12-06", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Joulupäivä".to_owned(),
                date: parse_date(&(format!("{}-12-25", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Tapaninpäivä".to_owned(),
                date: parse_date(&(format!("{}-12-26", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        let midsummer_day = date_next(super::date(year, 6, 20)?, 6)?;
        Ok(vec![
            Holiday {
                name: "Pitkäperjantai".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Pääsiäispäivä".to_owned(),
                date: easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Toinen pääsiäispäivä".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Helatorstai".to_owned(),
                date: (easter) + Duration::days(i64::from(39)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Helluntaipäivä".to_owned(),
                date: (easter) + Duration::days(i64::from(49)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Juhannuspäivä".to_owned(),
                date: if ((midsummer_day).day() as i32) > (26) {
                    (midsummer_day) + Duration::days(i64::from(-7))
                } else {
                    midsummer_day
                },
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Pyhäinpäivä".to_owned(),
                date: date_next(super::date(year, 10, 31)?, 6)?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
