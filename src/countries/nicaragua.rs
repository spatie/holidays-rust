use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Año nuevo".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día internacional de los trabajadores".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de las madres".to_owned(),
                    date: parse_date(&(format!("{}-05-30", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Aniversario de la revolución".to_owned(),
                    date: parse_date(&(format!("{}-07-19", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Aniversario de la batalla de san jacinto".to_owned(),
                    date: parse_date(&(format!("{}-09-14", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Aniversario de la independencia".to_owned(),
                    date: parse_date(&(format!("{}-09-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de la inmaculada concepción".to_owned(),
                    date: parse_date(&(format!("{}-12-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Navidad".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.holidays_introduced_in2026(year)?,
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn holidays_introduced_in2026(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        if (year) < (2026) {
            return Ok(vec![]);
        }
        Ok(vec![
            Holiday {
                name: "Día de Rubén Darío".to_owned(),
                date: parse_date(&(format!("{}-01-18", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día nacional de la reconciliación y la paz".to_owned(),
                date: parse_date(&(format!("{}-02-02", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de Augusto C. Sandino".to_owned(),
                date: parse_date(&(format!("{}-02-21", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de Carlos Fonseca Amador".to_owned(),
                date: parse_date(&(format!("{}-11-08", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok(vec![
            Holiday {
                name: "Jueves santo".to_owned(),
                date: (easter) + Duration::days(i64::from(-(3))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Viernes santo".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
