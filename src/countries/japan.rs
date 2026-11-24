use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = [
            vec![Holiday {
                name: "元日".to_owned(),
                date: parse_date(&(format!("{}-01-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            vec![Holiday {
                name: "建国記念の日".to_owned(),
                date: parse_date(&(format!("{}-02-11", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            vec![Holiday {
                name: "春分の日".to_owned(),
                date: self.equinox(year, 3, 20.8431)?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            vec![Holiday {
                name: "昭和の日".to_owned(),
                date: parse_date(&(format!("{}-04-29", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            vec![Holiday {
                name: "憲法記念日".to_owned(),
                date: parse_date(&(format!("{}-05-03", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            vec![Holiday {
                name: "みどりの日".to_owned(),
                date: parse_date(&(format!("{}-05-04", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            vec![Holiday {
                name: "こどもの日".to_owned(),
                date: parse_date(&(format!("{}-05-05", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            vec![Holiday {
                name: "秋分の日".to_owned(),
                date: self.equinox(year, 9, 23.2488)?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            vec![Holiday {
                name: "文化の日".to_owned(),
                date: parse_date(&(format!("{}-11-03", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            vec![Holiday {
                name: "勤労感謝の日".to_owned(),
                date: parse_date(&(format!("{}-11-23", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            self.emperors_birthday(year)?,
            self.variable_holidays(year)?,
        ]
        .concat();
        if (year) >= (2016) {
            holidays.push(Holiday {
                name: "山の日".to_owned(),
                date: match () {
                    _ if (year) == (2020) => parse_date("2020-08-10")?,
                    _ if (year) == (2021) => parse_date("2021-08-08")?,
                    _ => parse_date(&(format!("{}-08-11", year)))?,
                },
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        if (year) == (2019) {
            holidays.push(Holiday {
                name: "天皇の即位の日".to_owned(),
                date: parse_date("2019-05-01")?,
                holiday_type: HolidayType::National,
                region: None,
            });
            holidays.push(Holiday {
                name: "即位礼正殿の儀の行われる日".to_owned(),
                date: parse_date("2019-10-22")?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok([
            holidays.clone(),
            self.additional_holidays(holidays.clone())?,
        ]
        .concat())
    }
    pub(super) fn equinox(&self, year: i32, month: i32, base: f64) -> Result<Date, Error> {
        let day = ((((base) + ((0.242194) * (f64::from((year) - (1980)))))
            - (((f64::from((year) - (1980))) / (f64::from(4))).floor()))
        .floor()) as i32;
        parse_date(&(format!("{}-{:02}-{:02}", year, month, day)))
    }
    pub(super) fn emperors_birthday(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if (year) <= (2018) => {
                vec![Holiday {
                    name: "天皇誕生日".to_owned(),
                    date: parse_date(&(format!("{}-12-23", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }]
            }
            _ if (year) == (2019) => {
                vec![]
            }
            _ => {
                vec![Holiday {
                    name: "天皇誕生日".to_owned(),
                    date: parse_date(&(format!("{}-02-23", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }]
            }
        })
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "成人の日".to_owned(),
                date: parse_date(&(format!("second monday of january {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "敬老の日".to_owned(),
                date: parse_date(&(format!("third monday of september {}", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "海の日".to_owned(),
                date: match () {
                    _ if (year) == (2020) => parse_date("2020-07-23")?,
                    _ if (year) == (2021) => parse_date("2021-07-22")?,
                    _ => parse_date(&(format!("third monday of july {}", year)))?,
                },
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: if (year) >= (2020) {
                    "スポーツの日".to_owned()
                } else {
                    "体育の日".to_owned()
                },
                date: match () {
                    _ if (year) == (2020) => parse_date("2020-07-24")?,
                    _ if (year) == (2021) => parse_date("2021-07-23")?,
                    _ => parse_date(&(format!("second monday of october {}", year)))?,
                },
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn additional_holidays(
        &self,
        mut holidays: Vec<Holiday>,
    ) -> Result<Vec<Holiday>, Error> {
        let mut occupied_dates = (holidays.clone())
            .into_iter()
            .map(|holiday| -> Result<_, Error> { Ok(((holiday.clone()).date).to_string()) })
            .collect::<Result<Vec<_>, Error>>()?;
        let mut additional_holidays = vec![];
        for holiday in holidays.clone() {
            let day_between = ((holiday.clone()).date) + Duration::days(i64::from(1));
            if date_is(day_between, "sunday") {
                continue;
            }
            if (occupied_dates.clone()).contains(&((day_between).to_string())) {
                continue;
            }
            if !((occupied_dates.clone())
                .contains(&(((day_between) + Duration::days(i64::from(1))).to_string())))
            {
                continue;
            }
            occupied_dates.push((day_between).to_string());
            additional_holidays.push(Holiday {
                name: "国民の休日".to_owned(),
                date: day_between,
                holiday_type: HolidayType::Observed,
                region: None,
            });
        }
        holidays.sort_by_key(|h| h.date);
        for holiday in holidays.clone() {
            if !(date_is((holiday.clone()).date, "sunday")) {
                continue;
            }
            let mut substitute_date = ((holiday.clone()).date) + Duration::days(i64::from(1));
            while (occupied_dates.clone()).contains(&((substitute_date).to_string())) {
                substitute_date += Duration::days(i64::from(1));
            }
            occupied_dates.push((substitute_date).to_string());
            additional_holidays.push(Holiday {
                name: "振替休日".to_owned(),
                date: substitute_date,
                holiday_type: HolidayType::Observed,
                region: None,
            });
        }
        Ok(additional_holidays.clone())
    }
}
