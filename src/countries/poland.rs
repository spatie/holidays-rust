use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Nowy Rok".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Święto Trzech Króli".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Święto Pracy".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Święto Konstytucji 3 Maja".to_owned(),
                    date: parse_date(&(format!("{}-05-03", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Święto Wojska Polskiego, Wniebowzięcie Najświętszej Maryi Panny"
                        .to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Wszystkich Świętych".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Święto Niepodległości".to_owned(),
                    date: parse_date(&(format!("{}-11-11", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Boże Narodzenie".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Drugi Dzień Bożego Narodzenia".to_owned(),
                    date: parse_date(&(format!("{}-12-26", year)))?,
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
        let mut variable_holidays = vec![
            Holiday {
                name: "Wielkanoc".to_owned(),
                date: easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Poniedziałek Wielkanocny".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Zielone Świątki".to_owned(),
                date: (easter) + Duration::days(i64::from((7) * 7)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Boże Ciało".to_owned(),
                date: (easter) + Duration::days(i64::from(60)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ];
        if (year) >= (2025) {
            variable_holidays.push(Holiday {
                name: "wigilii Bożego Narodzenia".to_owned(),
                date: parse_date(&(format!("{}-12-24", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(variable_holidays.clone())
    }
}
