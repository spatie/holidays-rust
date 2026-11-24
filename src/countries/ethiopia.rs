use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([self.variable_holidays(year)?, self.fixed_holidays(year)?].concat())
    }
    pub(super) fn fixed_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "Ethiopian New Year".to_owned(),
                date: self.meskerem_date(year, 1)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Meskel".to_owned(),
                date: self.meskerem_date(year, 17)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Ethiopian Christmas".to_owned(),
                date: self.january_date(year, 7)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Epiphany".to_owned(),
                date: self.timkat_date(year)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Adwa Victory Day".to_owned(),
                date: parse_date(&(format!("{}-03-02", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "May Day".to_owned(),
                date: parse_date(&(format!("{}-05-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Patriots' Victory Day".to_owned(),
                date: parse_date(&(format!("{}-05-05", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Derg Downfall Day".to_owned(),
                date: parse_date(&(format!("{}-05-28", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, true)?;
        Ok([
            vec![
                Holiday {
                    name: "Siklet".to_owned(),
                    date: (easter) + Duration::days(i64::from(-(2))),
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Fasika".to_owned(),
                    date: easter,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.islamic_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn meskerem_date(&self, year: i32, day: i32) -> Result<Date, Error> {
        let leap_shift =
            if (parse_date(&(format!("{}{}", (year) + (1), "-01-01".to_owned())))?).leap_year() {
                1
            } else {
                0
            };
        Ok((parse_date(&(format!("{}-09-10", year)))?)
            + Duration::days(i64::from((day) + (leap_shift))))
    }
    pub(super) fn timkat_date(&self, year: i32) -> Result<Date, Error> {
        self.january_date(year, 19)
    }
    pub(super) fn january_date(&self, year: i32, mut day: i32) -> Result<Date, Error> {
        {
            day += if (parse_date(&(format!("{}-01-01", year)))?).leap_year() {
                1
            } else {
                0
            };
            day
        };
        parse_date(&(format!("{}-01-{}", year, day)))
    }
    pub(super) fn eid_al_fitr_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (1970, vec!["12-01".to_owned()]),
            (1971, vec!["11-20".to_owned()]),
            (1972, vec!["11-08".to_owned()]),
            (1973, vec!["10-28".to_owned()]),
            (1974, vec!["10-17".to_owned()]),
            (1975, vec!["10-06".to_owned()]),
            (1976, vec!["09-25".to_owned()]),
            (1977, vec!["09-15".to_owned()]),
            (1978, vec!["09-04".to_owned()]),
            (1979, vec!["08-24".to_owned()]),
            (1980, vec!["08-12".to_owned()]),
            (1981, vec!["08-01".to_owned()]),
            (1982, vec!["07-22".to_owned()]),
            (1983, vec!["07-12".to_owned()]),
            (1984, vec!["06-30".to_owned()]),
            (1985, vec!["06-20".to_owned()]),
            (1986, vec!["06-09".to_owned()]),
            (1987, vec!["05-29".to_owned()]),
            (1988, vec!["05-17".to_owned()]),
            (1989, vec!["05-06".to_owned()]),
            (1990, vec!["04-26".to_owned()]),
            (1991, vec!["04-16".to_owned()]),
            (1992, vec!["04-04".to_owned()]),
            (1993, vec!["03-24".to_owned()]),
            (1994, vec!["03-13".to_owned()]),
            (1995, vec!["03-03".to_owned()]),
            (1996, vec!["02-20".to_owned()]),
            (1997, vec!["02-09".to_owned()]),
            (1998, vec!["01-29".to_owned()]),
            (1999, vec!["01-19".to_owned()]),
            (2000, vec!["01-08".to_owned(), "12-27".to_owned()]),
            (2001, vec!["12-16".to_owned()]),
            (2002, vec!["12-05".to_owned()]),
            (2003, vec!["11-25".to_owned()]),
            (2004, vec!["11-14".to_owned()]),
            (2005, vec!["11-03".to_owned()]),
            (2006, vec!["10-23".to_owned()]),
            (2007, vec!["10-12".to_owned()]),
            (2008, vec!["09-30".to_owned()]),
            (2009, vec!["09-20".to_owned()]),
            (2010, vec!["09-09".to_owned()]),
            (2011, vec!["08-30".to_owned()]),
            (2012, vec!["08-19".to_owned()]),
            (2013, vec!["08-08".to_owned()]),
            (2014, vec!["07-28".to_owned()]),
            (2015, vec!["07-17".to_owned()]),
            (2016, vec!["07-05".to_owned()]),
            (2017, vec!["06-25".to_owned()]),
            (2018, vec!["06-15".to_owned()]),
            (2019, vec!["06-04".to_owned()]),
            (2020, vec!["05-24".to_owned()]),
            (2021, vec!["05-13".to_owned()]),
            (2022, vec!["05-02".to_owned()]),
            (2023, vec!["04-21".to_owned()]),
            (2024, vec!["04-10".to_owned()]),
            (2025, vec!["03-30".to_owned()]),
            (2026, vec!["03-20".to_owned()]),
            (2027, vec!["03-09".to_owned()]),
            (2028, vec!["02-26".to_owned()]),
            (2029, vec!["02-14".to_owned()]),
            (2030, vec!["02-04".to_owned()]),
            (2031, vec!["01-24".to_owned()]),
            (2032, vec!["01-14".to_owned()]),
            (2033, vec!["01-02".to_owned(), "12-23".to_owned()]),
            (2034, vec!["12-12".to_owned()]),
            (2035, vec!["12-01".to_owned()]),
            (2036, vec!["11-19".to_owned()]),
            (2037, vec!["11-09".to_owned()]),
        ]))
    }
    pub(super) fn eid_al_adha_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (1970, vec!["02-17".to_owned()]),
            (1971, vec!["02-16".to_owned()]),
            (1972, vec!["01-27".to_owned()]),
            (1973, vec!["01-15".to_owned()]),
            (1974, vec!["01-04".to_owned(), "12-24".to_owned()]),
            (1975, vec!["12-13".to_owned()]),
            (1976, vec!["12-02".to_owned()]),
            (1977, vec!["11-22".to_owned()]),
            (1978, vec!["11-11".to_owned()]),
            (1979, vec!["10-31".to_owned()]),
            (1980, vec!["10-19".to_owned()]),
            (1981, vec!["10-08".to_owned()]),
            (1982, vec!["09-27".to_owned()]),
            (1983, vec!["09-17".to_owned()]),
            (1984, vec!["09-06".to_owned()]),
            (1985, vec!["08-26".to_owned()]),
            (1986, vec!["08-16".to_owned()]),
            (1987, vec!["08-05".to_owned()]),
            (1988, vec!["07-24".to_owned()]),
            (1989, vec!["07-13".to_owned()]),
            (1990, vec!["07-03".to_owned()]),
            (1991, vec!["06-23".to_owned()]),
            (1992, vec!["06-11".to_owned()]),
            (1993, vec!["06-01".to_owned()]),
            (1994, vec!["05-21".to_owned()]),
            (1995, vec!["05-10".to_owned()]),
            (1996, vec!["04-28".to_owned()]),
            (1997, vec!["04-18".to_owned()]),
            (1998, vec!["04-07".to_owned()]),
            (1999, vec!["03-28".to_owned()]),
            (2000, vec!["03-16".to_owned()]),
            (2001, vec!["03-05".to_owned()]),
            (2002, vec!["02-22".to_owned()]),
            (2003, vec!["02-11".to_owned()]),
            (2004, vec!["02-01".to_owned()]),
            (2005, vec!["01-20".to_owned()]),
            (2006, vec!["01-10".to_owned(), "12-31".to_owned()]),
            (2007, vec!["12-20".to_owned()]),
            (2008, vec!["12-08".to_owned()]),
            (2009, vec!["11-27".to_owned()]),
            (2010, vec!["11-16".to_owned()]),
            (2011, vec!["11-06".to_owned()]),
            (2012, vec!["10-25".to_owned()]),
            (2013, vec!["10-15".to_owned()]),
            (2014, vec!["10-04".to_owned()]),
            (2015, vec!["09-23".to_owned()]),
            (2016, vec!["09-12".to_owned()]),
            (2017, vec!["09-01".to_owned()]),
            (2018, vec!["08-21".to_owned()]),
            (2019, vec!["08-11".to_owned()]),
            (2020, vec!["07-31".to_owned()]),
            (2021, vec!["07-20".to_owned()]),
            (2022, vec!["07-09".to_owned()]),
            (2023, vec!["06-28".to_owned()]),
            (2024, vec!["06-16".to_owned()]),
            (2025, vec!["06-06".to_owned()]),
            (2026, vec!["05-27".to_owned()]),
            (2027, vec!["05-16".to_owned()]),
            (2028, vec!["05-05".to_owned()]),
            (2029, vec!["04-24".to_owned()]),
            (2030, vec!["04-13".to_owned()]),
            (2031, vec!["04-02".to_owned()]),
            (2032, vec!["03-22".to_owned()]),
            (2033, vec!["03-11".to_owned()]),
            (2034, vec!["03-01".to_owned()]),
            (2035, vec!["02-18".to_owned()]),
            (2036, vec!["02-07".to_owned()]),
            (2037, vec!["01-26".to_owned()]),
        ]))
    }
    pub(super) fn prophet_muhammad_birthday_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (1970, vec!["05-18".to_owned()]),
            (1971, vec!["05-08".to_owned()]),
            (1972, vec!["04-26".to_owned()]),
            (1973, vec!["04-15".to_owned()]),
            (1974, vec!["04-05".to_owned()]),
            (1975, vec!["03-25".to_owned()]),
            (1976, vec!["03-13".to_owned()]),
            (1977, vec!["03-03".to_owned()]),
            (1978, vec!["02-20".to_owned()]),
            (1979, vec!["02-10".to_owned()]),
            (1980, vec!["01-30".to_owned()]),
            (1981, vec!["01-18".to_owned()]),
            (1982, vec!["01-08".to_owned()]),
            (1983, vec!["12-17".to_owned()]),
            (1984, vec!["12-06".to_owned()]),
            (1985, vec!["11-25".to_owned()]),
            (1986, vec!["11-15".to_owned()]),
            (1987, vec!["11-04".to_owned()]),
            (1988, vec!["10-23".to_owned()]),
            (1989, vec!["10-13".to_owned()]),
            (1990, vec!["10-02".to_owned()]),
            (1991, vec!["09-21".to_owned()]),
            (1992, vec!["09-10".to_owned()]),
            (1993, vec!["08-30".to_owned()]),
            (1994, vec!["08-19".to_owned()]),
            (1995, vec!["08-09".to_owned()]),
            (1996, vec!["07-28".to_owned()]),
            (1997, vec!["07-18".to_owned()]),
            (1998, vec!["07-07".to_owned()]),
            (1999, vec!["06-26".to_owned()]),
            (2000, vec!["06-15".to_owned()]),
            (2001, vec!["06-04".to_owned()]),
            (2002, vec!["05-24".to_owned()]),
            (2003, vec!["05-14".to_owned()]),
            (2004, vec!["05-02".to_owned()]),
            (2005, vec!["04-21".to_owned()]),
            (2006, vec!["04-11".to_owned()]),
            (2007, vec!["03-31".to_owned()]),
            (2008, vec!["03-20".to_owned()]),
            (2009, vec!["03-09".to_owned()]),
            (2010, vec!["02-26".to_owned()]),
            (2011, vec!["02-16".to_owned()]),
            (2012, vec!["02-05".to_owned()]),
            (2013, vec!["01-24".to_owned()]),
            (2014, vec!["01-13".to_owned()]),
            (2015, vec!["12-23".to_owned()]),
            (2016, vec!["12-12".to_owned()]),
            (2017, vec!["12-01".to_owned()]),
            (2018, vec!["11-20".to_owned()]),
            (2019, vec!["11-09".to_owned()]),
            (2020, vec!["10-29".to_owned()]),
            (2021, vec!["10-21".to_owned()]),
            (2022, vec!["10-08".to_owned()]),
            (2023, vec!["09-27".to_owned()]),
            (2024, vec!["09-15".to_owned()]),
            (2025, vec!["09-05".to_owned()]),
            (2026, vec!["08-25".to_owned()]),
            (2027, vec!["08-15".to_owned()]),
            (2028, vec!["08-03".to_owned()]),
            (2029, vec!["07-25".to_owned()]),
            (2030, vec!["07-14".to_owned()]),
            (2031, vec!["07-03".to_owned()]),
            (2032, vec!["06-21".to_owned()]),
            (2033, vec!["06-10".to_owned()]),
            (2034, vec!["05-31".to_owned()]),
            (2035, vec!["05-21".to_owned()]),
            (2036, vec!["05-09".to_owned()]),
            (2037, vec!["04-29".to_owned()]),
        ]))
    }
    pub(super) fn islamic_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![Holiday {
                name: "Mawlid".to_owned(),
                date: get_single_day_holiday(&(self.prophet_muhammad_birthday_dates()?), year)?,
                holiday_type: HolidayType::Religious,
                region: None,
            }],
            self.single_day_islamic_holidays(
                "Eid al-Adha".to_owned(),
                self.eid_al_adha_dates()?,
                year,
            )?,
            self.single_day_islamic_holidays(
                "Eid al-Fitr".to_owned(),
                self.eid_al_fitr_dates()?,
                year,
            )?,
        ]
        .concat())
    }
    pub(super) fn single_day_islamic_holidays(
        &self,
        name: String,
        dates: Table,
        year: i32,
    ) -> Result<Vec<Holiday>, Error> {
        dates
            .get(&year)
            .ok_or(Error::UnsupportedYear)?
            .iter()
            .map(|md| {
                Ok(Holiday {
                    name: name.clone(),
                    date: parse_date(&format!("{year}-{md}"))?,
                    holiday_type: HolidayType::Religious,
                    region: None,
                })
            })
            .collect()
    }
}
