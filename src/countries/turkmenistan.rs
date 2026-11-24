use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Täze ýyl".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Halkara zenanlar güni".to_owned(),
                    date: parse_date(&(format!("{}-03-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Milli bahar baýramy 1-nji güni".to_owned(),
                    date: parse_date(&(format!("{}-03-21", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Milli bahar baýramy 2-nji güni".to_owned(),
                    date: parse_date(&(format!("{}-03-22", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name:
                        "Türkmenistanyň Konstitusiýasynyň we Türkmenistanyň Döwlet baýdagynyň güni"
                            .to_owned(),
                    date: parse_date(&(format!("{}-05-18", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Türkmenistanyň Garaşsyzlyk güni".to_owned(),
                    date: parse_date(&(format!("{}-09-27", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Hatyra güni".to_owned(),
                    date: parse_date(&(format!("{}-10-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Halkara Bitaraplyk güni".to_owned(),
                    date: parse_date(&(format!("{}-12-12", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "Oraza baýramy".to_owned(),
                date: self.islamic_calendar("01-10".to_owned(), year, false)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Gurban baýramy 1-nji güni".to_owned(),
                date: self.islamic_calendar("10-12".to_owned(), year, false)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Gurban baýramy 2-nji güni".to_owned(),
                date: self.islamic_calendar("11-12".to_owned(), year, false)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Gurban baýramy 3-nji güni".to_owned(),
                date: self.islamic_calendar("12-12".to_owned(), year, false)?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn islamic_calendar(
        &self,
        input: String,
        year: i32,
        next_year: bool,
    ) -> Result<Date, Error> {
        let (day, month) = input
            .split_once('-')
            .ok_or_else(|| Error::InvalidDate(input.clone()))?;
        islamic_to_gregorian_date(&format!("{month}-{day}"), year, next_year)
    }
}
