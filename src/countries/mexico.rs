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
                    name: "Día Internacional de los Trabajadores".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de Independencia".to_owned(),
                    date: parse_date(&(format!("{}-09-16", year)))?,
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
        let constitution_day =
            (parse_date(&(format!("first monday of february {}", year)))?) - Duration::days(1);
        let benito_juarez_birth =
            (parse_date(&(format!("third monday of March {}", year)))?) - Duration::days(1);
        let revolution_day =
            (parse_date(&(format!("third monday of november {}", year)))?) - Duration::days(1);
        let mut holidays = vec![
            Holiday {
                name: "Día de la Constitución".to_owned(),
                date: constitution_day,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Natalicio de Benito Juárez".to_owned(),
                date: benito_juarez_birth,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de la Revolución".to_owned(),
                date: revolution_day,
                holiday_type: HolidayType::National,
                region: None,
            },
        ];
        let executive_change = self.government_change_date(year)?;
        if (executive_change).is_some() {
            holidays.push(Holiday {
                name: "Cambio de Gobierno".to_owned(),
                date: (executive_change).ok_or_else(|| Error::Calculation("Mexico".into()))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn government_change_date(&self, year: i32) -> Result<Option<Date>, Error> {
        let base_year = 1946;
        if (((year) - (base_year)) % (6)) == (0) {
            return Ok(Some((super::date(year, 10, 1)?) - Duration::days(1)));
        }
        Ok(None)
    }
}
