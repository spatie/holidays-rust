use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([self.fixed_holidays(year)?, self.variable_holidays(year)?].concat())
    }
    pub(super) fn fixed_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let holidays = IndexMap::from([
            ("New Year's Day".to_owned(), super::date(year, 1, 1)?),
            ("Labour Day".to_owned(), super::date(year, 5, 23)?),
            ("Emancipation Day".to_owned(), super::date(year, 8, 1)?),
            ("Independence Day".to_owned(), super::date(year, 8, 6)?),
            ("Christmas Day".to_owned(), super::date(year, 12, 25)?),
            ("Boxing Day".to_owned(), super::date(year, 12, 26)?),
        ]);
        let mut result = vec![];
        for (name, date) in holidays.clone() {
            let observed_day = match () {
                _ if (name.clone()) == "Labour Day" || (name.clone()) == "Boxing Day" => {
                    self.observed(name.clone(), date, year)?
                }
                _ => sunday_to_next_monday(date),
            };
            if (observed_day).is_some() {
                result.push(Holiday {
                    name: format!("{} Observed", name.clone()),
                    date: (observed_day).ok_or_else(|| Error::Calculation("Jamaica".into()))?,
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
                name: "Ash Wednesday".to_owned(),
                date: (easter) + Duration::days(i64::from(-(46))),
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
            Holiday {
                name: "National Heroes Day".to_owned(),
                date: parse_date(&(format!("third monday of October {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn observed(
        &self,
        name: String,
        date: Date,
        _year: i32,
    ) -> Result<Option<Date>, Error> {
        let holiday = date;
        if ((name.clone()) == "Labour Day") && (date_is(holiday, "saturday")) {
            return Ok(Some(date_next(holiday, "monday".to_owned())?));
        }
        if ((name.clone()) == "Boxing Day") && (date_is(holiday, "monday")) {
            return Ok(Some(date_next(holiday, "tuesday".to_owned())?));
        }
        Ok(None)
    }
}
