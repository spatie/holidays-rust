use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            self.observed_holidays(year)?,
            self.early_may_bank_holiday(year)?,
            self.variable_holidays(year)?,
            self.one_off_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn observed_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let holidays = IndexMap::from([
            ("New Year's Day".to_owned(), super::date(year, 1, 1)?),
            ("Christmas Day".to_owned(), super::date(year, 12, 25)?),
            ("Boxing Day".to_owned(), super::date(year, 12, 26)?),
        ]);
        let mut result = vec![];
        for (name, date) in holidays.clone() {
            let observed_day = match () {
                _ if (name.clone()) == "Christmas Day" => observed_christmas_day(date),
                _ if (name.clone()) == "Boxing Day" => observed_boxing_day(date),
                _ => weekend_to_next_monday(date),
            };
            if (observed_day).is_some() {
                result.push(Holiday {
                    name: format!("{} (substitute day)", name.clone()),
                    date: (observed_day).ok_or_else(|| Error::Calculation("Wales".into()))?,
                    holiday_type: HolidayType::National,
                    region: None,
                });
            } else {
                result.push(Holiday {
                    name: name.clone(),
                    date,
                    holiday_type: HolidayType::National,
                    region: None,
                });
            }
        }
        Ok(result.clone())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter_sunday = super::easter(year, false)?;
        let good_friday = (easter_sunday) + Duration::days(i64::from(-(2)));
        let easter_monday = (easter_sunday) + Duration::days(i64::from(1));
        Ok(vec![
            Holiday {
                name: "Good Friday".to_owned(),
                date: good_friday,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Easter Monday".to_owned(),
                date: easter_monday,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Spring bank holiday".to_owned(),
                date: parse_date(&(format!("last monday of may {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Summer bank holiday".to_owned(),
                date: parse_date(&(format!("last monday of august {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn early_may_bank_holiday(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        if (year) == (2020) {
            return Ok(vec![Holiday {
                name: "Early May bank holiday (VE day)".to_owned(),
                date: parse_date("2020-05-08")?,
                holiday_type: HolidayType::National,
                region: None,
            }]);
        }
        Ok(vec![Holiday {
            name: "Early May bank holiday".to_owned(),
            date: parse_date(&(format!("first monday of may {}", year)))?,
            holiday_type: HolidayType::National,
            region: None,
        }])
    }
    pub(super) fn one_off_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if (year) == (2022) => {
                vec![
                    Holiday {
                        name: "Platinum Jubilee bank holiday".to_owned(),
                        date: parse_date("2022-06-03")?,
                        holiday_type: HolidayType::National,
                        region: None,
                    },
                    Holiday {
                        name: "Bank Holiday for the State Funeral of Queen Elizabeth II".to_owned(),
                        date: parse_date("2022-09-19")?,
                        holiday_type: HolidayType::National,
                        region: None,
                    },
                ]
            }
            _ if (year) == (2023) => {
                vec![Holiday {
                    name: "Bank holiday for the coronation of King Charles III".to_owned(),
                    date: parse_date("2023-05-08")?,
                    holiday_type: HolidayType::National,
                    region: None,
                }]
            }
            _ => {
                vec![]
            }
        })
    }
}
