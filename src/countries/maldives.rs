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
            date: parse_date(&(format!("{}-07-26", year)))?,
            holiday_type: HolidayType::National,
            region: None,
        });
        holidays.push(Holiday {
            name: "Victory Day".to_owned(),
            date: parse_date(&(format!("{}-11-03", year)))?,
            holiday_type: HolidayType::National,
            region: None,
        });
        holidays.push(Holiday {
            name: "Republic Day".to_owned(),
            date: parse_date(&(format!("{}-11-11", year)))?,
            holiday_type: HolidayType::National,
            region: None,
        });
        if (year) >= (2015) {
            holidays.push(Holiday {
                name: "National Day".to_owned(),
                date: parse_date(&(format!("{}-01-24", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok([holidays.clone(), self.islamic_holidays(year)?].concat())
    }
    pub(super) fn islamic_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let eid_al_fitr = get_multi_day_holiday(&(self.eid_al_fitr_dates()?), year, 3)?;
        let eid_al_adha = get_multi_day_holiday(&(self.eid_al_adha_dates()?), year, 4)?;
        let mut holidays = [
            convert_periods(
                "Eid al-Adha".to_owned(),
                year,
                (eid_al_adha.clone())
                    .first()
                    .cloned()
                    .ok_or_else(|| Error::Calculation("Maldives".into()))?,
                HolidayType::Religious,
                "Day".to_owned(),
                String::new(),
                true,
            ),
            convert_periods(
                "Eid al-Fitr".to_owned(),
                year,
                (eid_al_fitr.clone())
                    .first()
                    .cloned()
                    .ok_or_else(|| Error::Calculation("Maldives".into()))?,
                HolidayType::Religious,
                "Day".to_owned(),
                String::new(),
                true,
            ),
        ]
        .concat();
        if ((eid_al_adha.clone()).len() as i32) > (1) {
            holidays = [
                holidays.clone(),
                convert_periods(
                    "2. Eid al-Adha".to_owned(),
                    year,
                    (eid_al_adha.clone())
                        .get(1_usize)
                        .cloned()
                        .ok_or_else(|| Error::Calculation("Maldives".into()))?,
                    HolidayType::Religious,
                    "Day".to_owned(),
                    String::new(),
                    true,
                ),
            ]
            .concat();
        }
        if ((eid_al_fitr.clone()).len() as i32) > (1) {
            holidays = [
                holidays.clone(),
                convert_periods(
                    "2. Eid al-Fitr".to_owned(),
                    year,
                    (eid_al_fitr.clone())
                        .get(1_usize)
                        .cloned()
                        .ok_or_else(|| Error::Calculation("Maldives".into()))?,
                    HolidayType::Religious,
                    "Day".to_owned(),
                    String::new(),
                    true,
                ),
            ]
            .concat();
        }
        Ok(holidays.clone())
    }
}
