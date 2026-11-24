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
            ("Constitution Day".to_owned(), super::date(year, 1, 7)?),
            ("Independence Day".to_owned(), super::date(year, 3, 6)?),
            ("May Day".to_owned(), super::date(year, 5, 1)?),
            ("Founder's Day".to_owned(), super::date(year, 9, 21)?),
            ("Christmas Day".to_owned(), super::date(year, 12, 25)?),
            ("Boxing Day".to_owned(), super::date(year, 12, 26)?),
        ]);
        let mut result = vec![];
        for (name, date) in holidays.clone() {
            let observed_day = match () {
                _ if (name.clone()) == "Christmas Day" => observed_christmas_day(date),
                _ if (name.clone()) == "Boxing Day" => observed_boxing_day(date),
                _ => self.day_to_next_friday_or_monday(date, year)?,
            };
            if (observed_day).is_some() {
                result.push(Holiday {
                    name: name.clone(),
                    date: (observed_day).ok_or_else(|| Error::Calculation("Ghana".into()))?,
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
        let easter = super::easter(year, false)?;
        Ok(vec![
            Holiday {
                name: "Farmers Day".to_owned(),
                date: parse_date(&(format!("first friday of December {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Good Friday".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Easter Monday".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn day_to_next_friday_or_monday(
        &self,
        date: Date,
        year: i32,
    ) -> Result<Option<Date>, Error> {
        if date == super::date(year, 12, 25)?
            || date == super::date(year, 12, 26)?
            || date == super::date(year, 1, 1)?
        {
            return Ok(Some(date));
        }
        if date_is(date, "weekend") {
            return Ok(Some(date_next(date, 1)?));
        }
        if !date_is(date, "friday") {
            return Ok(Some(date_next(date, 5)?));
        }
        Ok(None)
    }
}
