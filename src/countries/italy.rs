use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            [
                vec![Holiday {
                    name: "Capodanno".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }],
                vec![Holiday {
                    name: "Epifania".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }],
                vec![Holiday {
                    name: "Festa della Liberazione".to_owned(),
                    date: parse_date(&(format!("{}-04-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }],
                vec![Holiday {
                    name: "Festa dei Lavoratori".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }],
                vec![Holiday {
                    name: "Festa della Repubblica".to_owned(),
                    date: parse_date(&(format!("{}-06-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }],
                vec![Holiday {
                    name: "Assunzione di Maria".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }],
                self.saint_francis(year)?,
                vec![Holiday {
                    name: "Ognissanti".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }],
                vec![Holiday {
                    name: "Immacolata Concezione".to_owned(),
                    date: parse_date(&(format!("{}-12-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }],
                vec![Holiday {
                    name: "Natale".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }],
                vec![Holiday {
                    name: "Santo Stefano".to_owned(),
                    date: parse_date(&(format!("{}-12-26", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }],
            ]
            .concat(),
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok(vec![Holiday {
            name: "Lunedì di Pasqua".to_owned(),
            date: (easter) + Duration::days(i64::from(1)),
            holiday_type: HolidayType::National,
            region: None,
        }])
    }
    pub(super) fn saint_francis(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        if (1939..=1976).contains(&(year)) || ((year) >= (2026)) {
            return Ok(vec![Holiday {
                name: "San Francesco".to_owned(),
                date: parse_date(&(format!("{}-10-04", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }]);
        }
        Ok(vec![])
    }
}
