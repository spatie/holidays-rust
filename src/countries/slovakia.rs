use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = [
            vec![
                Holiday {
                    name: "Deň vzniku Slovenskej republiky".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Zjavenie Pána (Traja králi)".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Sviatok práce".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Deň víťazstva nad fašizmom".to_owned(),
                    date: parse_date(&(format!("{}-05-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Sviatok svätého Cyrila a Metoda".to_owned(),
                    date: parse_date(&(format!("{}-07-05", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Výročie Slovenského národného povstania".to_owned(),
                    date: parse_date(&(format!("{}-08-29", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Sedembolestná Panna Mária".to_owned(),
                    date: parse_date(&(format!("{}-09-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Sviatok všetkých svätých".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Deň boja za slobodu a demokraciu".to_owned(),
                    date: parse_date(&(format!("{}-11-17", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Štedrý deň".to_owned(),
                    date: parse_date(&(format!("{}-12-24", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Prvý sviatok vianočný".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Druhý sviatok vianočný".to_owned(),
                    date: parse_date(&(format!("{}-12-26", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
        ]
        .concat();
        if (year) == (2018) {
            holidays.push(Holiday {
                name: "Výročie Deklarácie slovenského národa".to_owned(),
                date: parse_date(&(format!("{}-10-30", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        if (year) < (2024) {
            holidays.push(Holiday {
                name: "Deň Ústavy Slovenskej republiky".to_owned(),
                date: parse_date(&(format!("{}-09-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        if (year) > (2024) {
            holidays = (holidays.clone())
                .into_iter()
                .filter(|holiday| {
                    ((holiday.clone()).name.clone()) != "Deň boja za slobodu a demokraciu"
                })
                .collect::<Vec<Holiday>>();
        }
        Ok(holidays.clone())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok(vec![
            Holiday {
                name: "Veľkonočný pondelok".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Veľký piatok".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
