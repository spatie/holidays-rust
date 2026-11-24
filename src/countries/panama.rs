use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let fixed_holidays = vec![
            Holiday {
                name: "Año Nuevo".to_owned(),
                date: parse_date(&(format!("{}-01-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de los Mártires".to_owned(),
                date: parse_date(&(format!("{}-01-09", year)))?,
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
                name: "Separación de Panamá de Colombia".to_owned(),
                date: parse_date(&(format!("{}-11-03", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de los Símbolos Patrios".to_owned(),
                date: parse_date(&(format!("{}-11-04", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de Consolidación de la Separación de Panamá con Colombia en Colón"
                    .to_owned(),
                date: parse_date(&(format!("{}-11-05", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Grito de la Independencia".to_owned(),
                date: parse_date(&(format!("{}-11-10", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Independencia de Panamá de España".to_owned(),
                date: parse_date(&(format!("{}-11-28", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de las Madres".to_owned(),
                date: parse_date(&(format!("{}-12-08", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de los Caídos por la invasión de Estados Unidos a Panamá".to_owned(),
                date: parse_date(&(format!("{}-12-20", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Navidad".to_owned(),
                date: parse_date(&(format!("{}-12-25", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ];
        Ok([
            fixed_holidays.clone(),
            self.variable_holidays(year)?,
            self.calculate_bridge_days(fixed_holidays.clone())?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok(vec![
            Holiday {
                name: "Carnaval (Día 1)".to_owned(),
                date: (easter) + Duration::days(i64::from(-(50))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Carnaval (Día 2)".to_owned(),
                date: (easter) + Duration::days(i64::from(-(49))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Carnaval (Día 3)".to_owned(),
                date: (easter) + Duration::days(i64::from(-(48))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Carnaval (Día 4)".to_owned(),
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
            Holiday {
                name: "Sábado de Gloria".to_owned(),
                date: (easter) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn calculate_bridge_days(
        &self,
        fixed_holidays: Vec<Holiday>,
    ) -> Result<Vec<Holiday>, Error> {
        let mut bridge_days = vec![];
        for holiday in fixed_holidays.clone() {
            let date = (holiday.clone()).date;
            if date_is(date, "sunday") {
                bridge_days.push(Holiday {
                    name: ((holiday.clone()).name.clone()) + " (Puente)",
                    date: (date) + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::National,
                    region: None,
                });
            }
        }
        Ok(bridge_days.clone())
    }
}
