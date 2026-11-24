use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Viti i Ri".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Krishtlindjet ortodokse".to_owned(),
                    date: parse_date(&(format!("{}-01-07", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dita Ndërkombëtare e Punës".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dita e Evropës".to_owned(),
                    date: parse_date(&(format!("{}-05-09", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Krishtlindjet Katolike".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Pashkët Katolike".to_owned(),
            date: super::easter(year, false)?,
            holiday_type: HolidayType::National,
            region: None,
        });
        holidays.push(Holiday {
            name: "Pashkët Ortodokse".to_owned(),
            date: super::easter(year, true)?,
            holiday_type: HolidayType::National,
            region: None,
        });
        if (year) >= (2008) {
            holidays.push(Holiday {
                name: "Dita e Pavarësisë së Republikës së Kosovës".to_owned(),
                date: parse_date(&(format!("{}-02-17", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
            holidays.push(Holiday {
                name: "Dita e Kushtetutës së Republikës së Kosovës".to_owned(),
                date: parse_date(&(format!("{}-04-09", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays.clone())
    }
}
