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
                    name: "Saint Patrick's Day".to_owned(),
                    date: parse_date(&(format!("{}-03-17", year)))?,
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
                    name: "Saint Stephen's Day".to_owned(),
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
        let mut variable_holidays = vec![
            Holiday {
                name: "Easter Monday".to_owned(),
                date: (super::easter(year, false)?) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "May Public Holiday".to_owned(),
                date: parse_date(&(format!("first monday of May {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "June Public Holiday".to_owned(),
                date: parse_date(&(format!("first monday of June {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "August Public Holiday".to_owned(),
                date: parse_date(&(format!("first monday of August {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "October Public Holiday".to_owned(),
                date: parse_date(&(format!("last monday of October {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ];
        if (year) >= (2023) {
            let mut st_brigids_day = parse_date(&(format!("{}-02-01", year)))?;
            if !(date_is(st_brigids_day, "friday")) {
                st_brigids_day = parse_date(&(format!("first monday of February {}", year)))?;
            }
            variable_holidays.push(Holiday {
                name: "St Brigid's Day".to_owned(),
                date: st_brigids_day,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(variable_holidays.clone())
    }
}
