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
                    name: "Declaración de la Independencia".to_owned(),
                    date: parse_date(&(format!("{}-04-19", year)))?,
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
                    name: "Aniversario de la Batalla de Carabobo".to_owned(),
                    date: parse_date(&(format!("{}-06-24", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de la Independencia".to_owned(),
                    date: parse_date(&(format!("{}-07-05", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Natalicio de Simón Bolívar".to_owned(),
                    date: parse_date(&(format!("{}-07-24", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de la Resistencia Indígena".to_owned(),
                    date: parse_date(&(format!("{}-10-12", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Víspera de Navidad".to_owned(),
                    date: parse_date(&(format!("{}-12-24", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Navidad".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de Fin de Año".to_owned(),
                    date: parse_date(&(format!("{}-12-31", year)))?,
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
                date: (easter) + Duration::days(i64::from(-(48))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Martes de Carnaval".to_owned(),
                date: (easter) + Duration::days(i64::from(-(47))),
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
}
