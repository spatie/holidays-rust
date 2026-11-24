use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Nouvel an / Jour de l'Indépendance".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Jour des Aieux".to_owned(),
                    date: parse_date(&(format!("{}-01-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Fête du Travail / Fête des Travailleurs".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Jour du Drapeau et de l'Université".to_owned(),
                    date: parse_date(&(format!("{}-05-18", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "L'Assomption de Marie".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Anniversaire de la mort de Dessalines".to_owned(),
                    date: parse_date(&(format!("{}-10-17", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Toussaint".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Jour des Morts".to_owned(),
                    date: parse_date(&(format!("{}-11-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Vertières".to_owned(),
                    date: parse_date(&(format!("{}-11-18", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Noël".to_owned(),
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
                name: "Carnaval/Mardi Gras".to_owned(),
                date: (easter) + Duration::days(i64::from(-(47))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Vendredi saint".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
