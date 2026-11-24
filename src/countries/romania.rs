use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Anul Nou".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "A doua zi de Anul Nou".to_owned(),
                    date: parse_date(&(format!("{}-01-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Bobotează".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Sfântul Ion".to_owned(),
                    date: parse_date(&(format!("{}-01-07", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ziua Unirii Principatelor Române".to_owned(),
                    date: parse_date(&(format!("{}-01-24", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ziua Muncii".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ziua Copilului".to_owned(),
                    date: parse_date(&(format!("{}-06-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Adormirea Maicii Domnului".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Sfântul Andrei".to_owned(),
                    date: parse_date(&(format!("{}-11-30", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ziua Națională".to_owned(),
                    date: parse_date(&(format!("{}-12-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Crăciunul".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "A doua zi de Crăciun".to_owned(),
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
        let easter = super::easter(year, true)?;
        Ok(vec![
            Holiday {
                name: "Vinerea Mare".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Paștele".to_owned(),
                date: easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "A doua zi de Paște".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Rusaliile".to_owned(),
                date: (easter) + Duration::days(i64::from(49)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "A doua zi de Rusalii".to_owned(),
                date: (easter) + Duration::days(i64::from(50)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
