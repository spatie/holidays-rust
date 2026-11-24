use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Nyårsdagen".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Trettondedag jul".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Första maj".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Nationaldagen".to_owned(),
                    date: parse_date(&(format!("{}-06-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Juldagen".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Annandag jul".to_owned(),
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
        let mut midsummer_day = parse_date(&(format!("{}-06-20", year)))?;
        if !(date_is(midsummer_day, "saturday")) {
            midsummer_day = date_next(midsummer_day, 6)?;
        }
        let mut halloween = parse_date(&(format!("{}-10-31", year)))?;
        if !(date_is(halloween, "saturday")) {
            halloween = date_next(halloween, 6)?;
        }
        Ok(vec![
            Holiday {
                name: "Långfredagen".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Påskdagen".to_owned(),
                date: easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Annandag påsk".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Kristi himmelsfärdsdag".to_owned(),
                date: (easter) + Duration::days(i64::from(39)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Pingstdagen".to_owned(),
                date: (easter) + Duration::days(i64::from(49)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Midsommardagen".to_owned(),
                date: if ((midsummer_day).day() as i32) > (26) {
                    (midsummer_day) + Duration::days(i64::from(-7))
                } else {
                    midsummer_day
                },
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Alla helgons dag".to_owned(),
                date: halloween,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
