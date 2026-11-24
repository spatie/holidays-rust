use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([self.observed_holidays(year)?, self.variable_holidays(year)?].concat())
    }
    pub(super) fn observed_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let holidays = IndexMap::from([
            ("New Year's Day".to_owned(), super::date(year, 1, 1)?),
            (
                "Day after New Year's Day".to_owned(),
                super::date(year, 1, 2)?,
            ),
            ("Waitangi Day".to_owned(), super::date(year, 2, 6)?),
            ("ANZAC Day".to_owned(), super::date(year, 4, 25)?),
            ("Christmas Day".to_owned(), super::date(year, 12, 25)?),
            ("Boxing Day".to_owned(), super::date(year, 12, 26)?),
        ]);
        let mut result = vec![];
        for (name, date) in holidays.clone() {
            let observed_day = match () {
                _ if (name.clone()) == "Day after New Year's Day" => {
                    self.second_of_january(year)?
                }
                _ if (name.clone()) == "Christmas Day" => observed_christmas_day(date),
                _ if (name.clone()) == "Boxing Day" => observed_boxing_day(date),
                _ => weekend_to_next_monday(date),
            };
            if (observed_day).is_some() {
                result.push(Holiday {
                    name: format!("{} (Mondayisation)", name.clone()),
                    date: (observed_day).ok_or_else(|| Error::Calculation("NewZealand".into()))?,
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
        let sovereign_title = self.sovereign_birthday_key(year)?;
        let sovereign_monday = parse_date(&(format!("first monday of june {}", year)))?;
        let labour_monday = parse_date(&(format!("fourth monday of october {}", year)))?;
        let mut holidays = vec![
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
                name: sovereign_title.clone(),
                date: sovereign_monday,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Labour Day".to_owned(),
                date: labour_monday,
                holiday_type: HolidayType::National,
                region: None,
            },
        ];
        let matariki = self.calculate_matariki(year)?;
        if (matariki).is_some() {
            holidays.push(Holiday {
                name: "Matariki".to_owned(),
                date: (matariki).ok_or_else(|| Error::Calculation("NewZealand".into()))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn second_of_january(&self, year: i32) -> Result<Option<Date>, Error> {
        let new_years_day = parse_date(&(format!("{}-01-01", year)))?;
        let second_of_january = (new_years_day) + Duration::days(i64::from(1));
        Ok(match () {
            _ if ((new_years_day).format("%A").to_string()) == "Friday" => {
                Some(date_next(second_of_january, "monday".to_owned())?)
            }
            _ if ((new_years_day).format("%A").to_string()) == "Saturday"
                || ((new_years_day).format("%A").to_string()) == "Sunday" =>
            {
                Some(date_next(second_of_january, "tuesday".to_owned())?)
            }
            _ => None,
        })
    }
    pub(super) fn sovereign_birthday_key(&self, year: i32) -> Result<String, Error> {
        Ok(if (year) >= (2023) {
            "King's Birthday".to_owned()
        } else {
            "Queen's Birthday".to_owned()
        })
    }
    pub(super) fn calculate_matariki(&self, year: i32) -> Result<Option<Date>, Error> {
        let matariki_dates = IndexMap::from([
            (2022, "2022-06-24".to_owned()),
            (2023, "2023-07-14".to_owned()),
            (2024, "2024-06-28".to_owned()),
            (2025, "2025-06-20".to_owned()),
            (2026, "2026-07-10".to_owned()),
            (2027, "2027-06-25".to_owned()),
            (2028, "2028-07-14".to_owned()),
            (2029, "2029-07-06".to_owned()),
            (2030, "2030-06-21".to_owned()),
            (2031, "2031-07-11".to_owned()),
            (2032, "2032-07-02".to_owned()),
            (2033, "2033-06-24".to_owned()),
            (2034, "2034-07-07".to_owned()),
            (2035, "2035-06-29".to_owned()),
            (2036, "2036-07-18".to_owned()),
            (2037, "2037-07-10".to_owned()),
            (2038, "2038-06-25".to_owned()),
            (2039, "2039-07-15".to_owned()),
            (2040, "2040-07-06".to_owned()),
            (2041, "2041-07-19".to_owned()),
            (2042, "2042-07-11".to_owned()),
            (2043, "2043-07-03".to_owned()),
            (2044, "2044-06-24".to_owned()),
            (2045, "2045-07-07".to_owned()),
            (2046, "2046-06-29".to_owned()),
            (2047, "2047-07-19".to_owned()),
            (2048, "2048-07-03".to_owned()),
            (2049, "2049-06-25".to_owned()),
            (2050, "2050-07-15".to_owned()),
            (2051, "2051-06-30".to_owned()),
            (2052, "2052-06-21".to_owned()),
        ]);
        Ok(if (matariki_dates.clone()).contains_key(&(year)) {
            Some(parse_date(
                &((matariki_dates.clone())
                    .get(&(year))
                    .cloned()
                    .ok_or_else(|| Error::Calculation("NewZealand".into()))?),
            )?)
        } else {
            None
        })
    }
}
