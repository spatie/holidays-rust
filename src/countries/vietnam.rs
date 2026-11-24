use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Tết Dương Lịch".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ngày Giải Phóng Miền Nam, Thống Nhất Đất Nước".to_owned(),
                    date: parse_date(&(format!("{}-04-30", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ngày Quốc Tế Lao Động".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ngày Quốc Khánh".to_owned(),
                    date: parse_date(&(format!("{}-09-02", year)))?,
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
            self.get_hung_kings_festival(year)?,
            self.get_lunar_new_year_holiday(year)?,
            self.get_the_extra_day_for_independence_day(year)?,
        ]
        .concat())
    }
    pub(super) fn get_hung_kings_festival(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![Holiday {
            name: "Ngày Giỗ Tổ Hùng Vương".to_owned(),
            date: chinese_to_gregorian_date("03-10", year)?,
            holiday_type: HolidayType::National,
            region: None,
        }])
    }
    pub(super) fn get_lunar_new_year_holiday(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let first_of_jan_in_chinese_calendar = chinese_to_gregorian_date("01-01", year)?;
        Ok(vec![
            Holiday {
                name: "Ngày Hai Mươi Chín Tết".to_owned(),
                date: (first_of_jan_in_chinese_calendar) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Ngày Ba Mươi Tết".to_owned(),
                date: (first_of_jan_in_chinese_calendar) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Mùng Một Tết Âm Lịch".to_owned(),
                date: first_of_jan_in_chinese_calendar,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Mùng Hai Tết Âm Lịch".to_owned(),
                date: (first_of_jan_in_chinese_calendar) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Mùng Ba Tết Âm Lịch".to_owned(),
                date: (first_of_jan_in_chinese_calendar) + Duration::days(i64::from(2)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Mùng Bốn Tết Âm Lịch".to_owned(),
                date: (first_of_jan_in_chinese_calendar) + Duration::days(i64::from(3)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Mùng Năm Tết Âm Lịch".to_owned(),
                date: (first_of_jan_in_chinese_calendar) + Duration::days(i64::from(4)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn get_the_extra_day_for_independence_day(
        &self,
        year: i32,
    ) -> Result<Vec<Holiday>, Error> {
        if (year) < (2021) {
            return Ok(vec![]);
        }
        let independence_day = parse_date(&(format!("{}-09-02", year)))?;
        if ((independence_day).weekday().num_days_from_sunday() as i32) == (1) {
            return Ok(vec![Holiday {
                name: "Ngày Sau Quốc Khánh".to_owned(),
                date: (independence_day) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            }]);
        }
        if ((independence_day).weekday().num_days_from_sunday() as i32) == (2) {
            return Ok(vec![Holiday {
                name: "Ngày Trước Quốc Khánh".to_owned(),
                date: (independence_day) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            }]);
        }
        if ((independence_day).weekday().num_days_from_sunday() as i32) == (3) {
            return Ok(vec![Holiday {
                name: "Ngày Trước Quốc Khánh".to_owned(),
                date: (independence_day) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            }]);
        }
        if ((independence_day).weekday().num_days_from_sunday() as i32) == (4) {
            return Ok(vec![Holiday {
                name: "Ngày Sau Quốc Khánh".to_owned(),
                date: (independence_day) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            }]);
        }
        if ((independence_day).weekday().num_days_from_sunday() as i32) == (5) {
            return Ok(vec![Holiday {
                name: "Ngày Trước Quốc Khánh".to_owned(),
                date: (independence_day) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            }]);
        }
        if ((independence_day).weekday().num_days_from_sunday() as i32) == (6) {
            return Ok(vec![Holiday {
                name: "Ngày Trước Quốc Khánh".to_owned(),
                date: (independence_day) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            }]);
        }
        if ((independence_day).weekday().num_days_from_sunday() as i32) == (0) {
            return Ok(vec![Holiday {
                name: "Ngày Sau Quốc Khánh".to_owned(),
                date: (independence_day) + Duration::days(i64::from(2)),
                holiday_type: HolidayType::National,
                region: None,
            }]);
        }
        Ok(vec![])
    }
}
