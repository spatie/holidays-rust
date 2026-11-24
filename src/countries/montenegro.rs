use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Nova godina - prvi dan".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Nova godina - drugi dan".to_owned(),
                    date: parse_date(&(format!("{}-01-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Badnji dan".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Božić - prvi dan".to_owned(),
                    date: parse_date(&(format!("{}-01-07", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Božić - drugi dan".to_owned(),
                    date: parse_date(&(format!("{}-01-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Praznik rada - prvi dan".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Praznik rada - drugi dan".to_owned(),
                    date: parse_date(&(format!("{}-05-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dan nezavisnosti - prvi dan".to_owned(),
                    date: parse_date(&(format!("{}-05-21", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dan nezavisnosti - drugi dan".to_owned(),
                    date: parse_date(&(format!("{}-05-22", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dan državnosti - prvi dan".to_owned(),
                    date: parse_date(&(format!("{}-07-13", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dan državnosti - drugi dan".to_owned(),
                    date: parse_date(&(format!("{}-07-14", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Njegošev dan - prvi dan".to_owned(),
                    date: parse_date(&(format!("{}-11-13", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Njegošev dan - drugi dan".to_owned(),
                    date: parse_date(&(format!("{}-11-14", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let orthodox_easter = super::easter(year, true)?;
        Ok(vec![
            Holiday {
                name: "Vaskrs".to_owned(),
                date: orthodox_easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Vaskršnji ponedjeljak".to_owned(),
                date: (orthodox_easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Veliki petak".to_owned(),
                date: (orthodox_easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
