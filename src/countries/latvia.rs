use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Jaunais gads".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Darba svētki".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Latvijas Republikas Neatkarības deklarācijas pasludināšanas diena"
                        .to_owned(),
                    date: parse_date(&(format!("{}-05-04", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Līgo diena".to_owned(),
                    date: parse_date(&(format!("{}-06-23", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Jāņu diena".to_owned(),
                    date: parse_date(&(format!("{}-06-24", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Latvijas Republikas proklamēšanas diena".to_owned(),
                    date: parse_date(&(format!("{}-11-18", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ziemassvētku vakars".to_owned(),
                    date: parse_date(&(format!("{}-12-24", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Pirmie Ziemassvētki".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Otrie Ziemassvētki".to_owned(),
                    date: parse_date(&(format!("{}-12-26", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Vecgada vakars".to_owned(),
                    date: parse_date(&(format!("{}-12-31", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
            self.observed_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok(vec![
            Holiday {
                name: "Lielā Piektdiena".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Pirmās Lieldienas".to_owned(),
                date: easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Otrās Lieldienas".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn observed_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let holidays = IndexMap::from([
            (
                "Latvijas Republikas Neatkarības deklarācijas pasludināšanas diena".to_owned(),
                super::date(year, 5, 4)?,
            ),
            (
                "Latvijas Republikas proklamēšanas diena".to_owned(),
                super::date(year, 11, 18)?,
            ),
        ]);
        let mut result = vec![];
        for (name, date) in holidays.clone() {
            let observed_day = weekend_to_next_monday(date);
            if (observed_day).is_some() {
                if (name.clone())
                    == "Latvijas Republikas Neatkarības deklarācijas pasludināšanas diena"
                {
                    result.push(Holiday {
                        name: "Pārceltā 4. maija brīvdiena".to_owned(),
                        date: (observed_day).ok_or_else(|| Error::Calculation("Latvia".into()))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    });
                }
                if (name.clone()) == "Latvijas Republikas proklamēšanas diena" {
                    result.push(Holiday {
                        name: "Pārceltā 18. novembra brīvdiena".to_owned(),
                        date: (observed_day).ok_or_else(|| Error::Calculation("Latvia".into()))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    });
                }
            }
        }
        Ok(result.clone())
    }
}
