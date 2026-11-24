use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        if (year) < (1991) {
            return Ok(vec![]);
        }
        Ok([
            vec![
                Holiday {
                    name: "Yangi yil".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Xalqaro xotin-qizlar kuni".to_owned(),
                    date: parse_date(&(format!("{}-03-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Navro'z".to_owned(),
                    date: parse_date(&(format!("{}-03-21", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Xotira va qadrlash kuni".to_owned(),
                    date: parse_date(&(format!("{}-05-09", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Mustaqillik kuni".to_owned(),
                    date: parse_date(&(format!("{}-09-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ustoz va murabbiylar kuni".to_owned(),
                    date: parse_date(&(format!("{}-10-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Konstitutsiya kuni".to_owned(),
                    date: parse_date(&(format!("{}-12-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        if (IndexMap::from([
            (1991, vec!["04-16".to_owned()]),
            (1992, vec!["04-04".to_owned()]),
            (1993, vec!["03-25".to_owned()]),
            (1994, vec!["03-14".to_owned()]),
            (1995, vec!["03-03".to_owned()]),
            (1996, vec!["02-21".to_owned()]),
            (1997, vec!["02-09".to_owned()]),
            (1998, vec!["01-30".to_owned()]),
            (1999, vec!["01-19".to_owned()]),
            (2000, vec!["01-08".to_owned(), "12-28".to_owned()]),
            (2001, vec!["12-17".to_owned()]),
            (2002, vec!["12-06".to_owned()]),
            (2003, vec!["11-26".to_owned()]),
            (2004, vec!["11-14".to_owned()]),
            (2005, vec!["11-04".to_owned()]),
            (2006, vec!["10-24".to_owned()]),
            (2007, vec!["10-13".to_owned()]),
            (2008, vec!["10-02".to_owned()]),
            (2009, vec!["09-21".to_owned()]),
            (2010, vec!["09-10".to_owned()]),
            (2011, vec!["08-31".to_owned()]),
            (2012, vec!["08-19".to_owned()]),
            (2013, vec!["08-08".to_owned()]),
            (2014, vec!["07-29".to_owned()]),
            (2015, vec!["07-18".to_owned()]),
            (2016, vec!["07-07".to_owned()]),
            (2017, vec!["06-26".to_owned()]),
            (2018, vec!["06-15".to_owned()]),
            (2019, vec!["06-04".to_owned()]),
            (2020, vec!["05-24".to_owned()]),
            (2021, vec!["05-13".to_owned()]),
            (2022, vec!["05-02".to_owned()]),
            (2023, vec!["04-21".to_owned()]),
            (2024, vec!["04-10".to_owned()]),
            (2025, vec!["03-31".to_owned()]),
            (2026, vec!["03-20".to_owned()]),
            (2027, vec!["03-10".to_owned()]),
            (2028, vec!["02-27".to_owned()]),
            (2029, vec!["02-15".to_owned()]),
            (2030, vec!["02-05".to_owned()]),
            (2031, vec!["01-25".to_owned()]),
            (2032, vec!["01-14".to_owned()]),
            (2033, vec!["01-03".to_owned(), "12-23".to_owned()]),
            (2034, vec!["12-12".to_owned()]),
            (2035, vec!["12-02".to_owned()]),
            (2036, vec!["11-20".to_owned()]),
            (2037, vec!["11-10".to_owned()]),
        ]))
        .contains_key(&(year))
        {
            for (key, holiday) in ((IndexMap::from([
                (1991, vec!["04-16".to_owned()]),
                (1992, vec!["04-04".to_owned()]),
                (1993, vec!["03-25".to_owned()]),
                (1994, vec!["03-14".to_owned()]),
                (1995, vec!["03-03".to_owned()]),
                (1996, vec!["02-21".to_owned()]),
                (1997, vec!["02-09".to_owned()]),
                (1998, vec!["01-30".to_owned()]),
                (1999, vec!["01-19".to_owned()]),
                (2000, vec!["01-08".to_owned(), "12-28".to_owned()]),
                (2001, vec!["12-17".to_owned()]),
                (2002, vec!["12-06".to_owned()]),
                (2003, vec!["11-26".to_owned()]),
                (2004, vec!["11-14".to_owned()]),
                (2005, vec!["11-04".to_owned()]),
                (2006, vec!["10-24".to_owned()]),
                (2007, vec!["10-13".to_owned()]),
                (2008, vec!["10-02".to_owned()]),
                (2009, vec!["09-21".to_owned()]),
                (2010, vec!["09-10".to_owned()]),
                (2011, vec!["08-31".to_owned()]),
                (2012, vec!["08-19".to_owned()]),
                (2013, vec!["08-08".to_owned()]),
                (2014, vec!["07-29".to_owned()]),
                (2015, vec!["07-18".to_owned()]),
                (2016, vec!["07-07".to_owned()]),
                (2017, vec!["06-26".to_owned()]),
                (2018, vec!["06-15".to_owned()]),
                (2019, vec!["06-04".to_owned()]),
                (2020, vec!["05-24".to_owned()]),
                (2021, vec!["05-13".to_owned()]),
                (2022, vec!["05-02".to_owned()]),
                (2023, vec!["04-21".to_owned()]),
                (2024, vec!["04-10".to_owned()]),
                (2025, vec!["03-31".to_owned()]),
                (2026, vec!["03-20".to_owned()]),
                (2027, vec!["03-10".to_owned()]),
                (2028, vec!["02-27".to_owned()]),
                (2029, vec!["02-15".to_owned()]),
                (2030, vec!["02-05".to_owned()]),
                (2031, vec!["01-25".to_owned()]),
                (2032, vec!["01-14".to_owned()]),
                (2033, vec!["01-03".to_owned(), "12-23".to_owned()]),
                (2034, vec!["12-12".to_owned()]),
                (2035, vec!["12-02".to_owned()]),
                (2036, vec!["11-20".to_owned()]),
                (2037, vec!["11-10".to_owned()]),
            ]))
            .get(&(year))
            .cloned()
            .ok_or(Error::UnsupportedYear)?)
            .into_iter()
            .enumerate()
            {
                let prefix = if key == (0) {
                    "".to_owned()
                } else {
                    format!("{}{}", " ".to_owned(), key + (1))
                };
                holidays.push(Holiday {
                    name: format!("Ramazon Hayiti{}", prefix.clone()),
                    date: parse_date(&(format!("{}-{}", year, holiday.clone())))?,
                    holiday_type: HolidayType::National,
                    region: None,
                });
            }
        }
        if (IndexMap::from([
            (1991, vec!["06-23".to_owned()]),
            (1992, vec!["06-11".to_owned()]),
            (1993, vec!["06-01".to_owned()]),
            (1994, vec!["05-21".to_owned()]),
            (1995, vec!["05-10".to_owned()]),
            (1996, vec!["04-29".to_owned()]),
            (1997, vec!["04-18".to_owned()]),
            (1998, vec!["04-08".to_owned()]),
            (1999, vec!["03-28".to_owned()]),
            (2000, vec!["03-16".to_owned()]),
            (2001, vec!["03-06".to_owned()]),
            (2002, vec!["02-23".to_owned()]),
            (2003, vec!["02-12".to_owned()]),
            (2004, vec!["02-02".to_owned()]),
            (2005, vec!["01-21".to_owned()]),
            (2006, vec!["01-10".to_owned(), "12-31".to_owned()]),
            (2007, vec!["12-20".to_owned()]),
            (2008, vec!["12-09".to_owned()]),
            (2009, vec!["11-28".to_owned()]),
            (2010, vec!["11-17".to_owned()]),
            (2011, vec!["11-07".to_owned()]),
            (2012, vec!["10-26".to_owned()]),
            (2013, vec!["10-15".to_owned()]),
            (2014, vec!["10-05".to_owned()]),
            (2015, vec!["09-24".to_owned()]),
            (2016, vec!["09-13".to_owned()]),
            (2017, vec!["09-02".to_owned()]),
            (2018, vec!["08-22".to_owned()]),
            (2019, vec!["08-11".to_owned()]),
            (2020, vec!["07-31".to_owned()]),
            (2021, vec!["07-20".to_owned()]),
            (2022, vec!["07-09".to_owned()]),
            (2023, vec!["06-28".to_owned()]),
            (2024, vec!["06-17".to_owned()]),
            (2025, vec!["06-07".to_owned()]),
            (2026, vec!["05-27".to_owned()]),
            (2027, vec!["05-17".to_owned()]),
            (2028, vec!["05-05".to_owned()]),
            (2029, vec!["04-24".to_owned()]),
            (2030, vec!["04-14".to_owned()]),
            (2031, vec!["04-03".to_owned()]),
            (2032, vec!["03-22".to_owned()]),
            (2033, vec!["03-12".to_owned()]),
            (2034, vec!["03-01".to_owned()]),
            (2035, vec!["02-18".to_owned()]),
            (2036, vec!["02-08".to_owned()]),
            (2037, vec!["01-27".to_owned()]),
        ]))
        .contains_key(&(year))
        {
            for (key, holiday) in ((IndexMap::from([
                (1991, vec!["06-23".to_owned()]),
                (1992, vec!["06-11".to_owned()]),
                (1993, vec!["06-01".to_owned()]),
                (1994, vec!["05-21".to_owned()]),
                (1995, vec!["05-10".to_owned()]),
                (1996, vec!["04-29".to_owned()]),
                (1997, vec!["04-18".to_owned()]),
                (1998, vec!["04-08".to_owned()]),
                (1999, vec!["03-28".to_owned()]),
                (2000, vec!["03-16".to_owned()]),
                (2001, vec!["03-06".to_owned()]),
                (2002, vec!["02-23".to_owned()]),
                (2003, vec!["02-12".to_owned()]),
                (2004, vec!["02-02".to_owned()]),
                (2005, vec!["01-21".to_owned()]),
                (2006, vec!["01-10".to_owned(), "12-31".to_owned()]),
                (2007, vec!["12-20".to_owned()]),
                (2008, vec!["12-09".to_owned()]),
                (2009, vec!["11-28".to_owned()]),
                (2010, vec!["11-17".to_owned()]),
                (2011, vec!["11-07".to_owned()]),
                (2012, vec!["10-26".to_owned()]),
                (2013, vec!["10-15".to_owned()]),
                (2014, vec!["10-05".to_owned()]),
                (2015, vec!["09-24".to_owned()]),
                (2016, vec!["09-13".to_owned()]),
                (2017, vec!["09-02".to_owned()]),
                (2018, vec!["08-22".to_owned()]),
                (2019, vec!["08-11".to_owned()]),
                (2020, vec!["07-31".to_owned()]),
                (2021, vec!["07-20".to_owned()]),
                (2022, vec!["07-09".to_owned()]),
                (2023, vec!["06-28".to_owned()]),
                (2024, vec!["06-17".to_owned()]),
                (2025, vec!["06-07".to_owned()]),
                (2026, vec!["05-27".to_owned()]),
                (2027, vec!["05-17".to_owned()]),
                (2028, vec!["05-05".to_owned()]),
                (2029, vec!["04-24".to_owned()]),
                (2030, vec!["04-14".to_owned()]),
                (2031, vec!["04-03".to_owned()]),
                (2032, vec!["03-22".to_owned()]),
                (2033, vec!["03-12".to_owned()]),
                (2034, vec!["03-01".to_owned()]),
                (2035, vec!["02-18".to_owned()]),
                (2036, vec!["02-08".to_owned()]),
                (2037, vec!["01-27".to_owned()]),
            ]))
            .get(&(year))
            .cloned()
            .ok_or(Error::UnsupportedYear)?)
            .into_iter()
            .enumerate()
            {
                let prefix = if key == (0) {
                    "".to_owned()
                } else {
                    format!("{}{}", " ".to_owned(), key + (1))
                };
                holidays.push(Holiday {
                    name: format!("Qurbon Hayiti{}", prefix.clone()),
                    date: parse_date(&(format!("{}-{}", year, holiday.clone())))?,
                    holiday_type: HolidayType::National,
                    region: None,
                });
            }
        }
        Ok(holidays.clone())
    }
}
