use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn eid_al_fitr_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2005, vec!["11-04".to_owned()]),
            (2006, vec!["10-24".to_owned()]),
            (2007, vec!["10-13".to_owned()]),
            (2008, vec!["10-02".to_owned()]),
            (2009, vec!["09-21".to_owned()]),
            (2010, vec!["09-10".to_owned()]),
            (2011, vec!["08-30".to_owned()]),
            (2012, vec!["08-19".to_owned()]),
            (2013, vec!["08-08".to_owned()]),
            (2014, vec!["07-28".to_owned()]),
            (2015, vec!["07-17".to_owned()]),
            (2016, vec!["07-07".to_owned()]),
            (2017, vec!["06-26".to_owned()]),
            (2018, vec!["06-15".to_owned()]),
            (2019, vec!["06-05".to_owned()]),
            (2020, vec!["05-24".to_owned()]),
            (2021, vec!["05-13".to_owned()]),
            (2022, vec!["05-02".to_owned()]),
            (2023, vec!["04-20".to_owned()]),
            (2024, vec!["04-10".to_owned()]),
            (2025, vec!["03-31".to_owned()]),
            (2026, vec!["03-21".to_owned()]),
            (2027, vec!["03-10".to_owned()]),
            (2028, vec!["02-27".to_owned()]),
            (2029, vec!["02-15".to_owned()]),
            (2030, vec!["02-04".to_owned()]),
            (2031, vec!["01-25".to_owned()]),
            (2032, vec!["01-15".to_owned()]),
            (2033, vec!["01-03".to_owned()]),
            (2034, vec!["12-13".to_owned()]),
            (2035, vec!["12-02".to_owned()]),
            (2036, vec!["11-20".to_owned()]),
            (2037, vec!["11-09".to_owned()]),
        ]))
    }
    pub(super) fn arafat_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2005, vec!["01-21".to_owned()]),
            (2006, vec!["01-10".to_owned()]),
            (2007, vec!["01-01".to_owned()]),
            (2008, vec!["12-09".to_owned()]),
            (2009, vec!["11-26".to_owned()]),
            (2010, vec!["11-15".to_owned()]),
            (2011, vec!["11-05".to_owned()]),
            (2012, vec!["10-25".to_owned()]),
            (2013, vec!["10-15".to_owned()]),
            (2014, vec!["10-04".to_owned()]),
            (2015, vec!["09-23".to_owned()]),
            (2016, vec!["09-11".to_owned()]),
            (2017, vec!["08-31".to_owned()]),
            (2018, vec!["08-20".to_owned()]),
            (2019, vec!["08-10".to_owned()]),
            (2020, vec!["07-30".to_owned()]),
            (2021, vec!["07-19".to_owned()]),
            (2022, vec!["07-09".to_owned()]),
            (2023, vec!["06-27".to_owned()]),
            (2024, vec!["06-16".to_owned()]),
            (2025, vec!["06-06".to_owned()]),
            (2026, vec!["05-26".to_owned()]),
            (2027, vec!["05-16".to_owned()]),
            (2028, vec!["05-05".to_owned()]),
            (2029, vec!["04-24".to_owned()]),
            (2030, vec!["04-13".to_owned()]),
            (2031, vec!["04-02".to_owned()]),
            (2032, vec!["03-21".to_owned()]),
            (2033, vec!["03-11".to_owned()]),
            (2034, vec!["03-01".to_owned()]),
            (2035, vec!["02-18".to_owned()]),
            (2036, vec!["02-07".to_owned()]),
            (2037, vec!["01-26".to_owned()]),
        ]))
    }
    pub(super) fn eid_al_adha_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2005, vec!["01-22".to_owned()]),
            (2006, vec!["01-11".to_owned()]),
            (2007, vec!["01-02".to_owned()]),
            (2008, vec!["12-10".to_owned()]),
            (2009, vec!["11-27".to_owned()]),
            (2010, vec!["11-16".to_owned()]),
            (2011, vec!["11-06".to_owned()]),
            (2012, vec!["10-26".to_owned()]),
            (2013, vec!["10-16".to_owned()]),
            (2014, vec!["10-05".to_owned()]),
            (2015, vec!["09-24".to_owned()]),
            (2016, vec!["09-12".to_owned()]),
            (2017, vec!["08-31".to_owned()]),
            (2018, vec!["08-21".to_owned()]),
            (2019, vec!["08-11".to_owned()]),
            (2020, vec!["07-31".to_owned()]),
            (2021, vec!["07-20".to_owned()]),
            (2022, vec!["07-09".to_owned()]),
            (2023, vec!["06-28".to_owned()]),
            (2024, vec!["06-17".to_owned()]),
            (2025, vec!["06-07".to_owned()]),
            (2026, vec!["05-27".to_owned()]),
            (2027, vec!["05-17".to_owned()]),
            (2028, vec!["05-06".to_owned()]),
            (2029, vec!["04-25".to_owned()]),
            (2030, vec!["04-14".to_owned()]),
            (2031, vec!["04-03".to_owned()]),
            (2032, vec!["03-22".to_owned()]),
            (2033, vec!["03-12".to_owned()]),
            (2034, vec!["03-02".to_owned()]),
            (2035, vec!["02-19".to_owned()]),
            (2036, vec!["02-08".to_owned()]),
            (2037, vec!["01-27".to_owned()]),
        ]))
    }
    pub(super) fn islamic_new_year_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2005, vec!["02-10".to_owned()]),
            (2006, vec!["01-31".to_owned()]),
            (2007, vec!["01-20".to_owned()]),
            (2008, vec!["01-10".to_owned()]),
            (2009, vec!["12-18".to_owned()]),
            (2010, vec!["12-07".to_owned()]),
            (2011, vec!["11-27".to_owned()]),
            (2012, vec!["11-15".to_owned()]),
            (2013, vec!["11-05".to_owned()]),
            (2014, vec!["10-25".to_owned()]),
            (2015, vec!["10-14".to_owned()]),
            (2016, vec!["10-03".to_owned()]),
            (2017, vec!["09-22".to_owned()]),
            (2018, vec!["09-11".to_owned()]),
            (2019, vec!["08-31".to_owned()]),
            (2020, vec!["08-20".to_owned()]),
            (2021, vec!["08-09".to_owned()]),
            (2022, vec!["07-30".to_owned()]),
            (2023, vec!["07-19".to_owned()]),
            (2024, vec!["07-08".to_owned()]),
            (2025, vec!["06-06".to_owned()]),
            (2026, vec!["06-17".to_owned()]),
            (2027, vec!["06-07".to_owned()]),
            (2028, vec!["05-26".to_owned()]),
            (2029, vec!["05-15".to_owned()]),
            (2030, vec!["05-05".to_owned()]),
            (2031, vec!["04-24".to_owned()]),
            (2032, vec!["04-12".to_owned()]),
            (2033, vec!["04-01".to_owned()]),
            (2034, vec!["03-22".to_owned()]),
            (2035, vec!["03-12".to_owned()]),
            (2036, vec!["02-29".to_owned()]),
            (2037, vec!["02-17".to_owned()]),
        ]))
    }
    pub(super) fn prophet_muhammad_birthday_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
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
            (2023, vec!["09-28".to_owned()]),
            (2024, vec!["09-16".to_owned()]),
            (2025, vec!["09-06".to_owned()]),
            (2026, vec!["08-26".to_owned()]),
            (2027, vec!["08-15".to_owned()]),
            (2028, vec!["08-04".to_owned()]),
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
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "New Year's Day".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::National,
            region: None,
        });
        holidays.push(Holiday {
            name: "Independence Day".to_owned(),
            date: parse_date(&(format!("{}-03-20", year)))?,
            holiday_type: HolidayType::National,
            region: None,
        });
        holidays.push(Holiday {
            name: "Martyrs' Day".to_owned(),
            date: parse_date(&(format!("{}-04-09", year)))?,
            holiday_type: HolidayType::National,
            region: None,
        });
        holidays.push(Holiday {
            name: "Labour Day".to_owned(),
            date: parse_date(&(format!("{}-05-01", year)))?,
            holiday_type: HolidayType::National,
            region: None,
        });
        holidays.push(Holiday {
            name: "Republic Day".to_owned(),
            date: parse_date(&(format!("{}-07-25", year)))?,
            holiday_type: HolidayType::National,
            region: None,
        });
        holidays.push(Holiday {
            name: "Women's Day".to_owned(),
            date: parse_date(&(format!("{}-08-13", year)))?,
            holiday_type: HolidayType::National,
            region: None,
        });
        holidays.push(Holiday {
            name: "Evacuation Day".to_owned(),
            date: parse_date(&(format!("{}-10-15", year)))?,
            holiday_type: HolidayType::National,
            region: None,
        });
        if (2011..2022).contains(&(year)) {
            holidays.push(Holiday {
                name: "Revolution and Youth Day".to_owned(),
                date: parse_date(&(format!("{}-01-14", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        if (year) >= (2022) {
            holidays.push(Holiday {
                name: "Revolution and Youth Day".to_owned(),
                date: parse_date(&(format!("{}-12-17", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok([holidays.clone(), self.islamic_holidays(year)?].concat())
    }
    pub(super) fn islamic_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let eid_al_fitr = get_multi_day_holiday(&(self.eid_al_fitr_dates()?), year, 2)?;
        let eid_al_adha = get_multi_day_holiday(&(self.eid_al_adha_dates()?), year, 2)?;
        Ok([
            vec![
                Holiday {
                    name: "Arafat Day".to_owned(),
                    date: get_single_day_holiday(&(self.arafat_dates()?), year)?,
                    holiday_type: HolidayType::Religious,
                    region: None,
                },
                Holiday {
                    name: "Islamic New Year".to_owned(),
                    date: get_single_day_holiday(&(self.islamic_new_year_dates()?), year)?,
                    holiday_type: HolidayType::Religious,
                    region: None,
                },
                Holiday {
                    name: "Birthday of the Prophet Muhammad".to_owned(),
                    date: get_single_day_holiday(&(self.prophet_muhammad_birthday_dates()?), year)?,
                    holiday_type: HolidayType::Religious,
                    region: None,
                },
            ],
            convert_periods(
                "Eid al-Fitr".to_owned(),
                year,
                (eid_al_fitr.clone())
                    .first()
                    .cloned()
                    .ok_or_else(|| Error::Calculation("Tunisia".into()))?,
                HolidayType::Religious,
                "Day".to_owned(),
                String::new(),
                false,
            ),
            convert_periods(
                "Eid al-Adha".to_owned(),
                year,
                (eid_al_adha.clone())
                    .first()
                    .cloned()
                    .ok_or_else(|| Error::Calculation("Tunisia".into()))?,
                HolidayType::Religious,
                "Day".to_owned(),
                String::new(),
                false,
            ),
        ]
        .concat())
    }
}
