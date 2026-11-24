use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Año Nuevo".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de los Héroes".to_owned(),
                    date: parse_date(&(format!("{}-03-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día del Trabajador".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de la Independencia Nacional".to_owned(),
                    date: parse_date(&(format!("{}-05-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Fundación de Asunción".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Batalla de Boquerón".to_owned(),
                    date: parse_date(&(format!("{}-09-29", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Virgen de Caacupé".to_owned(),
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
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok(vec![
            Holiday {
                name: "Paz del Chaco".to_owned(),
                date: self.chaco_armistice(year)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Jueves Santo".to_owned(),
                date: (easter) + Duration::days(i64::from(-(3))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Viernes Santo".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn chaco_armistice(&self, year: i32) -> Result<Date, Error> {
        super::date(year, 6, if (year) == (2014) { 16 } else { 12 })
    }
}
