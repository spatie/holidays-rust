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
                    name: "Carnaval".to_owned(),
                    date: parse_date(&(format!("{}-02-12", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Carnaval Día 2".to_owned(),
                    date: parse_date(&(format!("{}-02-13", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día Nacional de la Memoria por la Verdad y la Justicia".to_owned(),
                    date: parse_date(&(format!("{}-03-24", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día del Veterano y de los Caídos en la Guerra de Malvinas".to_owned(),
                    date: parse_date(&(format!("{}-04-02", year)))?,
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
                    name: "Revolución de Mayo".to_owned(),
                    date: parse_date(&(format!("{}-05-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Paso a la Inmortalidad del General Manuel Belgrano".to_owned(),
                    date: parse_date(&(format!("{}-06-20", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de la Independencia".to_owned(),
                    date: parse_date(&(format!("{}-07-09", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de la Inmaculada Concepción de María".to_owned(),
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
        Ok(vec![Holiday {
            name: "Viernes Santo".to_owned(),
            date: (easter) + Duration::days(i64::from(-(2))),
            holiday_type: HolidayType::National,
            region: None,
        }])
    }
}
