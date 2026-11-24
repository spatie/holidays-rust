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
                    name: "Día del Trabajo".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de la independencia".to_owned(),
                    date: parse_date(&(format!("{}-07-20", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Batalla de Boyacá".to_owned(),
                    date: parse_date(&(format!("{}-08-07", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Inmaculada Concepción".to_owned(),
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
        let mut holidays = vec![
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
            Holiday {
                name: "Ascención de Jesús".to_owned(),
                date: (easter) + Duration::days(i64::from(43)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Corpus Christi".to_owned(),
                date: (easter) + Duration::days(i64::from(64)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Sagrado corazón de Jesús".to_owned(),
                date: (easter) + Duration::days(i64::from(71)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Reyes Magos".to_owned(),
                date: self.emiliani_holiday(year, 1, 6)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de San José".to_owned(),
                date: self.emiliani_holiday(year, 3, 19)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "San Pedro y San Pablo".to_owned(),
                date: self.emiliani_holiday(year, 6, 29)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Asunción de la Virgen".to_owned(),
                date: self.emiliani_holiday(year, 8, 15)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de la raza".to_owned(),
                date: self.emiliani_holiday(year, 10, 12)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Todos los santos".to_owned(),
                date: self.emiliani_holiday(year, 11, 1)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Independencia de Cartagena".to_owned(),
                date: self.emiliani_holiday(year, 11, 11)?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ];
        if (year) >= (2026) {
            holidays.push(Holiday {
                name: "Día de Nuestra Señora del Rosario de Chiquinquirá".to_owned(),
                date: self.emiliani_holiday(year, 7, 9)?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn emiliani_holiday(&self, year: i32, month: i32, day: i32) -> Result<Date, Error> {
        let date_obj = super::date(year, month, day)?;
        if date_is(date_obj, "Monday") {
            return Ok(date_obj);
        }
        date_next(date_obj, "Monday".to_owned())
    }
}
