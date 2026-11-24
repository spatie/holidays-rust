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
                    name: "Labour day".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Madaraka Day".to_owned(),
                    date: parse_date(&(format!("{}-06-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: self.get_october10_holiday_name(year)?,
                    date: parse_date(&(format!("{}-10-10", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Mashujaa Day".to_owned(),
                    date: parse_date(&(format!("{}-10-20", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Jamhuri Day".to_owned(),
                    date: parse_date(&(format!("{}-12-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Christmas".to_owned(),
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
        Ok(vec![
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
        ])
    }
    pub(super) fn get_october10_holiday_name(&self, year: i32) -> Result<String, Error> {
        Ok(match () {
            _ if (year) >= (2024) => "Mazingira Day".to_owned(),
            _ if (year) >= (2022) => "Utamaduni Day".to_owned(),
            _ if (year) >= (2019) => "Huduma Day".to_owned(),
            _ => "Moi Day".to_owned(),
        })
    }
}
