use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([self.fixed_holidays(year)?, self.variable_holidays(year)?].concat())
    }
    pub(super) fn fixed_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "Nýársdagur".to_owned(),
                date: parse_date(&(format!("{}-01-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Verkalýðsdagurinn".to_owned(),
                date: parse_date(&(format!("{}-05-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Þjóðhátíðardagurinn".to_owned(),
                date: parse_date(&(format!("{}-06-17", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Jóladagur".to_owned(),
                date: parse_date(&(format!("{}-12-25", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Annar í jólum".to_owned(),
                date: parse_date(&(format!("{}-12-26", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        let mut first_day_of_summer = date_next(super::date(year, 4, 19)?, 4)?;
        if ((first_day_of_summer).day() as i32) > (25) {
            first_day_of_summer += Duration::days(i64::from(-7));
        }
        let mut commerce_day = super::date(year, 8, 1)?;
        if ((commerce_day).weekday().num_days_from_sunday() as i32) != (1) {
            commerce_day = date_next(commerce_day, 1)?;
        }
        Ok(vec![
            Holiday {
                name: "Skírdagur".to_owned(),
                date: (easter) + Duration::days(i64::from(-(3))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Föstudagurinn langi".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Páskadagur".to_owned(),
                date: easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Annar í páskum".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Sumardagurinn fyrsti".to_owned(),
                date: first_day_of_summer,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Uppstigningardagur".to_owned(),
                date: (easter) + Duration::days(i64::from(39)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Hvítasunnudagur".to_owned(),
                date: (easter) + Duration::days(i64::from(49)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Annar í hvítasunnu".to_owned(),
                date: (easter) + Duration::days(i64::from(50)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Frídagur verslunarmanna".to_owned(),
                date: commerce_day,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
