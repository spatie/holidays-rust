use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn eid_al_fitr_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
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
    pub(super) fn ashura_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2020, vec!["08-30".to_owned()]),
            (2021, vec!["08-19".to_owned()]),
            (2022, vec!["08-08".to_owned()]),
            (2023, vec!["07-28".to_owned()]),
            (2024, vec!["07-17".to_owned()]),
            (2025, vec!["07-07".to_owned()]),
            (2026, vec!["06-26".to_owned()]),
            (2027, vec!["06-15".to_owned()]),
            (2028, vec!["06-04".to_owned()]),
            (2029, vec!["05-24".to_owned()]),
            (2030, vec!["05-13".to_owned()]),
            (2031, vec!["05-02".to_owned()]),
            (2032, vec!["04-20".to_owned()]),
            (2033, vec!["04-10".to_owned()]),
            (2034, vec!["03-30".to_owned()]),
            (2035, vec!["03-19".to_owned()]),
            (2036, vec!["03-08".to_owned()]),
            (2037, vec!["02-25".to_owned()]),
        ]))
    }
    pub(super) fn prophet_muhammad_birthday_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
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
        Ok([
            vec![
                Holiday {
                    name: "New Year's Day".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Labour Day".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "National Day".to_owned(),
                    date: parse_date(&(format!("{}-12-16", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "National Day 2".to_owned(),
                    date: parse_date(&(format!("{}-12-17", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.islamic_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn islamic_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let eid_al_fitr = get_multi_day_holiday(&(self.eid_al_fitr_dates()?), year, 3)?;
        let eid_al_adha = get_multi_day_holiday(&(self.eid_al_adha_dates()?), year, 3)?;
        let ashura = get_multi_day_holiday(&(self.ashura_dates()?), year, 2)?;
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
                "Eid al-Adha".to_owned(),
                year,
                (eid_al_adha.clone())
                    .first()
                    .cloned()
                    .ok_or_else(|| Error::Calculation("Bahrain".into()))?,
                HolidayType::Religious,
                "Day".to_owned(),
                String::new(),
                false,
            ),
            convert_periods(
                "Eid al-Fitr".to_owned(),
                year,
                (eid_al_fitr.clone())
                    .first()
                    .cloned()
                    .ok_or_else(|| Error::Calculation("Bahrain".into()))?,
                HolidayType::Religious,
                "Day".to_owned(),
                String::new(),
                false,
            ),
            convert_periods(
                "Ashura".to_owned(),
                year,
                (ashura.clone())
                    .first()
                    .cloned()
                    .ok_or_else(|| Error::Calculation("Bahrain".into()))?,
                HolidayType::Religious,
                "Day".to_owned(),
                String::new(),
                false,
            ),
        ]
        .concat())
    }
}
