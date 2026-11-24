use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Fête du Nouvel An".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Fete du travail".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Fête de l'indépendance".to_owned(),
                    date: parse_date(&(format!("{}-08-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Jour de l'assomption".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Jour de la Toussaint".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Jour de Noel".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.traditional_religions_holidays(year)?,
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn traditional_religions_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let name = "Fête annuelle des réligions traditionnelles".to_owned();
        if (year) >= (2025) {
            let second_friday = parse_date(&(format!("second friday of january {}", year)))?;
            return Ok(vec![
                Holiday {
                    name: name.clone(),
                    date: (second_friday) + Duration::days(i64::from(-(1))),
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: name.clone(),
                    date: second_friday,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ]);
        }
        if (year) >= (1998) {
            return Ok(vec![Holiday {
                name: name.clone(),
                date: parse_date(&(format!("{}-01-10", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }]);
        }
        Ok(vec![])
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok([
            vec![
                Holiday {
                    name: "Lundi de Pâques".to_owned(),
                    date: (easter) + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Jour de l’Ascension".to_owned(),
                    date: (easter) + Duration::days(i64::from(39)),
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Lundi de Pentecôte".to_owned(),
                    date: (easter) + Duration::days(i64::from(50)),
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.get_islamic_holidays(
                year,
                IndexMap::from([
                    (1970, vec!["05-17".to_owned()]),
                    (1971, vec!["05-07".to_owned()]),
                    (1972, vec!["04-25".to_owned()]),
                    (1973, vec!["04-14".to_owned()]),
                    (1974, vec!["04-04".to_owned()]),
                    (1975, vec!["03-24".to_owned()]),
                    (1976, vec!["03-02".to_owned()]),
                    (1977, vec!["02-19".to_owned()]),
                    (1978, vec!["02-09".to_owned()]),
                    (1979, vec!["01-29".to_owned()]),
                    (1980, vec!["01-17".to_owned()]),
                    (1981, vec!["01-07".to_owned()]),
                    (1982, vec!["12-27".to_owned()]),
                    (1983, vec!["12-16".to_owned()]),
                    (1984, vec!["12-05".to_owned()]),
                    (1985, vec!["11-24".to_owned()]),
                    (1986, vec!["11-14".to_owned()]),
                    (1987, vec!["11-03".to_owned()]),
                    (1988, vec!["10-22".to_owned()]),
                    (1989, vec!["10-12".to_owned()]),
                    (1990, vec!["10-01".to_owned()]),
                    (1991, vec!["09-20".to_owned()]),
                    (1992, vec!["09-09".to_owned()]),
                    (1993, vec!["08-29".to_owned()]),
                    (1994, vec!["08-18".to_owned()]),
                    (1995, vec!["08-08".to_owned()]),
                    (1996, vec!["07-27".to_owned()]),
                    (1997, vec!["07-17".to_owned()]),
                    (1998, vec!["07-06".to_owned()]),
                    (1999, vec!["06-25".to_owned()]),
                    (2000, vec!["06-14".to_owned()]),
                    (2001, vec!["06-03".to_owned()]),
                    (2002, vec!["05-23".to_owned()]),
                    (2003, vec!["05-13".to_owned()]),
                    (2004, vec!["05-01".to_owned()]),
                    (2005, vec!["04-20".to_owned()]),
                    (2006, vec!["04-10".to_owned()]),
                    (2007, vec!["03-30".to_owned()]),
                    (2008, vec!["03-08".to_owned()]),
                    (2009, vec!["02-25".to_owned()]),
                    (2010, vec!["02-15".to_owned()]),
                    (2011, vec!["02-04".to_owned()]),
                    (2012, vec!["01-23".to_owned()]),
                    (2013, vec!["01-13".to_owned()]),
                    (2014, vec!["01-02".to_owned()]),
                    (2015, vec!["12-23".to_owned()]),
                    (2016, vec!["12-11".to_owned()]),
                    (2017, vec!["11-30".to_owned()]),
                    (2018, vec!["11-20".to_owned()]),
                    (2019, vec!["11-09".to_owned()]),
                    (2020, vec!["10-28".to_owned()]),
                    (2021, vec!["10-18".to_owned()]),
                    (2022, vec!["10-07".to_owned()]),
                    (2023, vec!["09-26".to_owned()]),
                    (2024, vec!["09-15".to_owned()]),
                    (2025, vec!["09-04".to_owned()]),
                    (2026, vec!["08-25".to_owned()]),
                    (2027, vec!["08-14".to_owned()]),
                    (2028, vec!["08-02".to_owned()]),
                    (2029, vec!["07-23".to_owned()]),
                    (2030, vec!["07-12".to_owned()]),
                    (2031, vec!["07-01".to_owned()]),
                    (2032, vec!["06-20".to_owned()]),
                    (2033, vec!["06-09".to_owned()]),
                    (2034, vec!["05-29".to_owned()]),
                    (2035, vec!["05-19".to_owned()]),
                    (2036, vec!["05-07".to_owned()]),
                    (2037, vec!["04-27".to_owned()]),
                ]),
                "Jour du Maouloud".to_owned(),
                1,
            )?,
            self.get_islamic_holidays(
                year,
                IndexMap::from([
                    (1970, vec!["12-11".to_owned()]),
                    (1971, vec!["11-30".to_owned()]),
                    (1972, vec!["11-20".to_owned()]),
                    (1973, vec!["11-08".to_owned()]),
                    (1974, vec!["10-28".to_owned()]),
                    (1975, vec!["10-18".to_owned()]),
                    (1976, vec!["10-07".to_owned()]),
                    (1977, vec!["09-15".to_owned()]),
                    (1978, vec!["09-04".to_owned()]),
                    (1979, vec!["08-25".to_owned()]),
                    (1980, vec!["08-13".to_owned()]),
                    (1981, vec!["08-02".to_owned()]),
                    (1982, vec!["07-23".to_owned()]),
                    (1983, vec!["07-12".to_owned()]),
                    (1984, vec!["06-30".to_owned()]),
                    (1985, vec!["06-20".to_owned()]),
                    (1986, vec!["06-09".to_owned()]),
                    (1987, vec!["05-30".to_owned()]),
                    (1988, vec!["05-18".to_owned()]),
                    (1989, vec!["05-07".to_owned()]),
                    (1990, vec!["04-27".to_owned()]),
                    (1991, vec!["04-16".to_owned()]),
                    (1992, vec!["04-04".to_owned()]),
                    (1993, vec!["03-25".to_owned()]),
                    (1994, vec!["03-14".to_owned()]),
                    (1995, vec!["03-03".to_owned()]),
                    (1996, vec!["02-21".to_owned()]),
                    (1997, vec!["02-09".to_owned()]),
                    (1998, vec!["01-30".to_owned()]),
                    (1999, vec!["01-19".to_owned()]),
                    (2000, vec!["01-08".to_owned()]),
                    (2001, vec!["12-28".to_owned()]),
                    (2002, vec!["12-17".to_owned()]),
                    (2003, vec!["12-06".to_owned()]),
                    (2004, vec!["11-26".to_owned()]),
                    (2005, vec!["11-14".to_owned()]),
                    (2006, vec!["11-03".to_owned()]),
                    (2007, vec!["10-24".to_owned()]),
                    (2008, vec!["10-13".to_owned()]),
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
                    (2019, vec!["06-05".to_owned()]),
                    (2020, vec!["05-24".to_owned()]),
                    (2021, vec!["05-13".to_owned()]),
                    (2022, vec!["05-03".to_owned()]),
                    (2023, vec!["04-22".to_owned()]),
                    (2024, vec!["04-10".to_owned()]),
                    (2025, vec!["03-31".to_owned()]),
                    (2026, vec!["03-20".to_owned()]),
                    (2027, vec!["03-10".to_owned()]),
                    (2028, vec!["02-27".to_owned()]),
                    (2029, vec!["02-15".to_owned()]),
                    (2030, vec!["02-05".to_owned()]),
                    (2031, vec!["01-25".to_owned()]),
                    (2032, vec!["01-14".to_owned()]),
                    (2033, vec!["01-03".to_owned()]),
                    (2034, vec!["12-23".to_owned()]),
                    (2035, vec!["12-12".to_owned()]),
                    (2036, vec!["12-02".to_owned()]),
                    (2037, vec!["11-20".to_owned()]),
                ]),
                "Fête de Ramadan".to_owned(),
                1,
            )?,
            self.get_islamic_holidays(
                year,
                IndexMap::from([
                    (1970, vec!["02-17".to_owned()]),
                    (1971, vec!["02-06".to_owned()]),
                    (1972, vec!["01-27".to_owned()]),
                    (1973, vec!["01-15".to_owned()]),
                    (1974, vec!["01-04".to_owned()]),
                    (1975, vec!["12-25".to_owned()]),
                    (1976, vec!["12-14".to_owned()]),
                    (1977, vec!["11-22".to_owned()]),
                    (1978, vec!["11-11".to_owned()]),
                    (1979, vec!["11-01".to_owned()]),
                    (1980, vec!["10-20".to_owned()]),
                    (1981, vec!["10-09".to_owned()]),
                    (1982, vec!["09-29".to_owned()]),
                    (1983, vec!["09-18".to_owned()]),
                    (1984, vec!["09-06".to_owned()]),
                    (1985, vec!["08-27".to_owned()]),
                    (1986, vec!["08-16".to_owned()]),
                    (1987, vec!["08-06".to_owned()]),
                    (1988, vec!["07-25".to_owned()]),
                    (1989, vec!["07-14".to_owned()]),
                    (1990, vec!["07-04".to_owned()]),
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
                    (2006, vec!["01-10".to_owned()]),
                    (2007, vec!["12-31".to_owned()]),
                    (2008, vec!["12-20".to_owned()]),
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
                    (2019, vec!["08-12".to_owned()]),
                    (2020, vec!["07-31".to_owned()]),
                    (2021, vec!["07-20".to_owned()]),
                    (2022, vec!["07-10".to_owned()]),
                    (2023, vec!["06-29".to_owned()]),
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
                ]),
                "Fête de la Tabaski".to_owned(),
                1,
            )?,
        ]
        .concat())
    }
    pub(super) fn get_islamic_holidays(
        &self,
        year: i32,
        holidays: Table,
        label: String,
        day: i32,
    ) -> Result<Vec<Holiday>, Error> {
        let mut result = Vec::new();
        let mut counter = 0;
        if year != 1970 {
            let previous = holidays
                .get(&(year - 1))
                .and_then(|v| v.last())
                .ok_or(Error::UnsupportedYear)?;
            let previous = parse_date(&format!("{}-{previous}", year - 1))?;
            if (previous + Duration::days(i64::from(day - 1))).year() == year {
                result =
                    self.prepare_holidays(previous, day, label.clone(), year, String::new())?;
                counter += 1;
            }
        }
        for md in holidays.get(&year).ok_or(Error::UnsupportedYear)? {
            let start = parse_date(&format!("{year}-{md}"))?;
            let prefix = if counter > 0 {
                format!("{}. ", counter + 1)
            } else {
                String::new()
            };
            result.extend(self.prepare_holidays(start, day, label.clone(), year, prefix)?);
            counter += 1;
        }
        if year != 2037 {
            let next = holidays
                .get(&(year + 1))
                .and_then(|v| v.last())
                .ok_or(Error::UnsupportedYear)?;
            let next = parse_date(&format!("{}-{next}", year + 1))?;
            if (next - Duration::days(1)).year() == year {
                let prefix = if counter > 0 {
                    format!("{}. ", counter + 1)
                } else {
                    String::new()
                };
                result.extend(self.prepare_holidays(next, day, label, year, prefix)?);
            }
        }
        Ok(result)
    }
    pub(super) fn prepare_holidays(
        &self,
        holiday: Date,
        day: i32,
        label: String,
        filter_year: i32,
        prefix: String,
    ) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        for range in (1..=day).collect::<Vec<_>>() {
            holidays.push(Holiday {
                name: format!("{}{} - Jour {}", prefix.clone(), label.clone(), range),
                date: (holiday) + Duration::days(i64::from((range) - (1))),
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok((holidays.clone())
            .into_iter()
            .filter(|holiday| (((holiday.clone()).date).year()) == (filter_year))
            .collect::<Vec<Holiday>>())
    }
}
