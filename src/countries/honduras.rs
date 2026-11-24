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
                    name: "Día de la Independencia".to_owned(),
                    date: parse_date(&(format!("{}-09-15", year)))?,
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
        let morazanic_week = self.get_morazanic_week(year)?;
        let day_of_the_americas = self.get_day_of_the_americas(year, easter)?;
        Ok([vec![
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
                name: "Sábado Santo".to_owned(),
                date: (easter) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de las Américas".to_owned(),
                date: day_of_the_americas,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Feriado Morazánico (3 Octubre)".to_owned(),
                date: (morazanic_week.clone())
                    .get(&("miercoles".to_owned()))
                    .cloned()
                    .ok_or_else(|| Error::Calculation("Honduras".into()))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Feriado Morazánico (12 Octubre)".to_owned(),
                date: (morazanic_week.clone())
                    .get(&("jueves".to_owned()))
                    .cloned()
                    .ok_or_else(|| Error::Calculation("Honduras".into()))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Feriado Morazánico (21 Octubre)".to_owned(),
                date: (morazanic_week.clone())
                    .get(&("viernes".to_owned()))
                    .cloned()
                    .ok_or_else(|| Error::Calculation("Honduras".into()))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ]]
        .concat())
    }
    pub(super) fn get_day_of_the_americas(&self, year: i32, easter: Date) -> Result<Date, Error> {
        let date = super::date(year, 4, 14)?;
        let jueves_santo = (easter) + Duration::days(i64::from(-(3)));
        let sabado_santo = (easter) + Duration::days(i64::from(-(1)));
        let lunes_posterior_ss = (sabado_santo) + Duration::days(i64::from(2));
        if (date) >= ((date_start_of_week(date)?) + Duration::days(i64::from(1)))
            && (date) <= ((date_end_of_week(date)?) + Duration::days(i64::from(-(2))))
        {
            if (date) >= ((jueves_santo) + Duration::days(i64::from(-(2))))
                && (date) <= (sabado_santo)
            {
                return Ok(lunes_posterior_ss);
            }
            return date_next(date, 1);
        }
        if date_is(date, "weekend") {
            return Ok(date);
        }
        Ok(date)
    }
    pub(super) fn get_morazanic_week(&self, year: i32) -> Result<IndexMap<String, Date>, Error> {
        let start_of_month = super::date(year, 10, 1)?;
        let miercoles = date_first_of_month(start_of_month, 3)?;
        Ok(IndexMap::from([
            ("miercoles".to_owned(), miercoles),
            (
                "jueves".to_owned(),
                (miercoles) + Duration::days(i64::from(1)),
            ),
            (
                "viernes".to_owned(),
                (miercoles) + Duration::days(i64::from(2)),
            ),
        ]))
    }
}
