use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn eid_al_fitr_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2020, vec!["05-23".to_owned()]),
            (2021, vec!["05-12".to_owned()]),
            (2022, vec!["05-01".to_owned()]),
            (2023, vec!["04-21".to_owned()]),
            (2024, vec!["04-09".to_owned()]),
            (2025, vec!["03-30".to_owned()]),
            (2026, vec!["03-19".to_owned()]),
            (2027, vec!["03-09".to_owned()]),
            (2028, vec!["02-26".to_owned()]),
            (2029, vec!["02-14".to_owned()]),
            (2030, vec!["02-03".to_owned()]),
            (2031, vec!["01-24".to_owned()]),
            (2032, vec!["01-14".to_owned()]),
            (2033, vec!["01-02".to_owned(), "12-22".to_owned()]),
            (2034, vec!["12-11".to_owned()]),
            (2035, vec!["11-30".to_owned()]),
            (2036, vec!["11-19".to_owned()]),
            (2037, vec!["11-08".to_owned()]),
        ]))
    }
    pub(super) fn arafat_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2020, vec!["07-30".to_owned()]),
            (2021, vec!["07-19".to_owned()]),
            (2022, vec!["07-08".to_owned()]),
            (2023, vec!["06-27".to_owned()]),
            (2024, vec!["06-15".to_owned()]),
            (2025, vec!["06-05".to_owned()]),
            (2026, vec!["05-26".to_owned()]),
            (2027, vec!["05-15".to_owned()]),
            (2028, vec!["05-04".to_owned()]),
            (2029, vec!["04-23".to_owned()]),
            (2030, vec!["04-12".to_owned()]),
            (2031, vec!["04-01".to_owned()]),
            (2032, vec!["03-21".to_owned()]),
            (2033, vec!["03-11".to_owned()]),
            (2034, vec!["02-28".to_owned()]),
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
            (2024, vec!["06-16".to_owned()]),
            (2025, vec!["06-06".to_owned()]),
            (2026, vec!["05-27".to_owned()]),
            (2027, vec!["05-16".to_owned()]),
            (2028, vec!["05-05".to_owned()]),
            (2029, vec!["04-24".to_owned()]),
            (2030, vec!["04-13".to_owned()]),
            (2031, vec!["04-02".to_owned()]),
            (2032, vec!["03-22".to_owned()]),
            (2033, vec!["03-12".to_owned()]),
            (2034, vec!["03-01".to_owned()]),
            (2035, vec!["02-19".to_owned()]),
            (2036, vec!["02-08".to_owned()]),
            (2037, vec!["01-27".to_owned()]),
        ]))
    }
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut new_holidays = vec![];
        if (year) >= (2022) {
            new_holidays.push(Holiday {
                name: "Founding Day".to_owned(),
                date: parse_date(&(format!("{}-02-22", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok([
            vec![Holiday {
                name: "National Day".to_owned(),
                date: parse_date(&(format!("{}-09-23", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            }],
            new_holidays.clone(),
            self.islamic_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn islamic_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![Holiday {
            name: "Arafat Day".to_owned(),
            date: get_single_day_holiday(&(self.arafat_dates()?), year)?,
            holiday_type: HolidayType::Religious,
            region: None,
        }];
        for period in get_multi_day_holiday(&(self.eid_al_fitr_dates()?), year, 4)? {
            holidays = [
                holidays.clone(),
                convert_periods(
                    "Eid al-Fitr Holiday".to_owned(),
                    year,
                    period.clone(),
                    HolidayType::Religious,
                    "Day".to_owned(),
                    String::new(),
                    false,
                ),
            ]
            .concat();
        }
        for period in get_multi_day_holiday(&(self.eid_al_adha_dates()?), year, 3)? {
            holidays = [
                holidays.clone(),
                convert_periods(
                    "Eid al-Adha Holiday".to_owned(),
                    year,
                    period.clone(),
                    HolidayType::Religious,
                    "Day".to_owned(),
                    String::new(),
                    false,
                ),
            ]
            .concat();
        }
        Ok(holidays.clone())
    }
}
