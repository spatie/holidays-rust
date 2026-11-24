use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Día de Año Nuevo".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día del Estado Plurinacional".to_owned(),
                    date: parse_date(&(format!("{}-01-22", year)))?,
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
                    name: "Año Nuevo Aymara".to_owned(),
                    date: parse_date(&(format!("{}-06-21", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de la Independencia".to_owned(),
                    date: parse_date(&(format!("{}-08-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de Todos los Santos".to_owned(),
                    date: parse_date(&(format!("{}-11-02", year)))?,
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
                name: "Lunes de Carnaval".to_owned(),
                date: ((easter) + Duration::days(i64::from(-((6) * 7))))
                    + Duration::days(i64::from(-(6))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Martes de Carnaval".to_owned(),
                date: ((easter) + Duration::days(i64::from(-((6) * 7))))
                    + Duration::days(i64::from(-(5))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Viernes Santo".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Corpus Christi".to_owned(),
                date: ((easter) + Duration::days(i64::from((8) * 7)))
                    + Duration::days(i64::from(4)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
