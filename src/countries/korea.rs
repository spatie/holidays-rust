use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "신정".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "3ㆍ1절".to_owned(),
                    date: parse_date(&(format!("{}-03-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "어린이날".to_owned(),
                    date: parse_date(&(format!("{}-05-05", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "현충일".to_owned(),
                    date: parse_date(&(format!("{}-06-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "광복절".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "개천절".to_owned(),
                    date: parse_date(&(format!("{}-10-03", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "한글날".to_owned(),
                    date: parse_date(&(format!("{}-10-09", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "크리스마스".to_owned(),
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
        Ok([
            self.get_lunar_new_year_holiday(year)?,
            self.get_lunar_buddhas_birthday(year)?,
            self.get_lunar_chuseok(year)?,
        ]
        .concat())
    }
    pub(super) fn get_lunar_new_year_holiday(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let first_of_jan_in_chinese_calendar = chinese_to_gregorian_date("01-01", year)?;
        Ok(vec![
            Holiday {
                name: "설날 연휴 1".to_owned(),
                date: (first_of_jan_in_chinese_calendar) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "설날".to_owned(),
                date: first_of_jan_in_chinese_calendar,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "설날 연휴 2".to_owned(),
                date: (first_of_jan_in_chinese_calendar) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn get_lunar_buddhas_birthday(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let buddhas_birthday_in_chinese_calendar = chinese_to_gregorian_date("04-08", year)?;
        Ok(vec![Holiday {
            name: "부처님 오신날".to_owned(),
            date: buddhas_birthday_in_chinese_calendar,
            holiday_type: HolidayType::National,
            region: None,
        }])
    }
    pub(super) fn get_lunar_chuseok(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let chuseok_in_chinese_calendar = chinese_to_gregorian_date("08-15", year)?;
        Ok(vec![
            Holiday {
                name: "추석 연휴 1".to_owned(),
                date: (chuseok_in_chinese_calendar) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "추석".to_owned(),
                date: chuseok_in_chinese_calendar,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "추석 연휴 2".to_owned(),
                date: (chuseok_in_chinese_calendar) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
