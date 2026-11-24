use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Neujahr".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Heilige Drei Könige".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Tag der Arbeit".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Staatsfeiertag / Mariä Himmelfahrt".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Mariä Geburt".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Allerheiligen".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Mariä Empfängnis".to_owned(),
                    date: parse_date(&(format!("{}-12-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Weihnachten".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Stephanstag".to_owned(),
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
                name: "Ostermontag".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Auffahrt".to_owned(),
                date: (easter) + Duration::days(i64::from(39)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Pfingstmontag".to_owned(),
                date: (easter) + Duration::days(i64::from(50)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Fronleichnam".to_owned(),
                date: (easter) + Duration::days(i64::from(60)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
