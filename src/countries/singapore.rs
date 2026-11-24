use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn chinese_new_year_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2020, vec!["01-25".to_owned()]),
            (2021, vec!["02-12".to_owned()]),
            (2022, vec!["02-01".to_owned()]),
            (2023, vec!["01-22".to_owned()]),
            (2024, vec!["02-10".to_owned()]),
            (2025, vec!["01-29".to_owned()]),
            (2026, vec!["02-17".to_owned()]),
            (2027, vec!["02-06".to_owned()]),
        ]))
    }
    pub(super) fn hari_raya_puasa_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2020, vec!["05-24".to_owned()]),
            (2021, vec!["05-13".to_owned()]),
            (2022, vec!["05-03".to_owned()]),
            (2023, vec!["04-22".to_owned()]),
            (2024, vec!["04-10".to_owned()]),
            (2025, vec!["03-31".to_owned()]),
            (2026, vec!["03-21".to_owned()]),
            (2027, vec!["03-10".to_owned()]),
        ]))
    }
    pub(super) fn hari_raya_haji_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2020, vec!["07-31".to_owned()]),
            (2021, vec!["07-20".to_owned()]),
            (2022, vec!["07-10".to_owned()]),
            (2023, vec!["06-29".to_owned()]),
            (2024, vec!["06-17".to_owned()]),
            (2025, vec!["06-07".to_owned()]),
            (2026, vec!["05-27".to_owned()]),
            (2027, vec!["05-17".to_owned()]),
        ]))
    }
    pub(super) fn vesak_day_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2020, vec!["05-07".to_owned()]),
            (2021, vec!["05-26".to_owned()]),
            (2022, vec!["05-15".to_owned()]),
            (2023, vec!["06-02".to_owned()]),
            (2024, vec!["05-22".to_owned()]),
            (2025, vec!["05-12".to_owned()]),
            (2026, vec!["05-31".to_owned()]),
            (2027, vec!["05-20".to_owned()]),
        ]))
    }
    pub(super) fn deepavali_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2020, vec!["11-14".to_owned()]),
            (2021, vec!["11-04".to_owned()]),
            (2022, vec!["10-24".to_owned()]),
            (2023, vec!["11-12".to_owned()]),
            (2024, vec!["10-31".to_owned()]),
            (2025, vec!["10-20".to_owned()]),
            (2026, vec!["11-08".to_owned()]),
            (2027, vec!["10-28".to_owned()]),
        ]))
    }
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let chinese_new_year = get_single_day_holiday(&(self.chinese_new_year_dates()?), year)?;
        let holidays = vec![
            Holiday {
                name: "New Year's Day".to_owned(),
                date: parse_date(&(format!("{}-01-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Chinese New Year".to_owned(),
                date: chinese_new_year,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Chinese New Year Day 2".to_owned(),
                date: (chinese_new_year) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Good Friday".to_owned(),
                date: (super::easter(year, false)?) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::Religious,
                region: None,
            },
            Holiday {
                name: "Hari Raya Puasa".to_owned(),
                date: get_single_day_holiday(&(self.hari_raya_puasa_dates()?), year)?,
                holiday_type: HolidayType::Religious,
                region: None,
            },
            Holiday {
                name: "Labour Day".to_owned(),
                date: parse_date(&(format!("{}-05-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Vesak Day".to_owned(),
                date: get_single_day_holiday(&(self.vesak_day_dates()?), year)?,
                holiday_type: HolidayType::Religious,
                region: None,
            },
            Holiday {
                name: "Hari Raya Haji".to_owned(),
                date: get_single_day_holiday(&(self.hari_raya_haji_dates()?), year)?,
                holiday_type: HolidayType::Religious,
                region: None,
            },
            Holiday {
                name: "National Day".to_owned(),
                date: parse_date(&(format!("{}-08-09", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Deepavali".to_owned(),
                date: get_single_day_holiday(&(self.deepavali_dates()?), year)?,
                holiday_type: HolidayType::Religious,
                region: None,
            },
            Holiday {
                name: "Christmas Day".to_owned(),
                date: parse_date(&(format!("{}-12-25", year)))?,
                holiday_type: HolidayType::Religious,
                region: None,
            },
        ];
        Ok([holidays.clone(), self.observed_holidays(holidays.clone())?].concat())
    }
    pub(super) fn observed_holidays(&self, holidays: Vec<Holiday>) -> Result<Vec<Holiday>, Error> {
        let mut occupied_dates = (holidays.clone())
            .into_iter()
            .map(|holiday| -> Result<_, Error> { Ok(((holiday.clone()).date).to_string()) })
            .collect::<Result<Vec<_>, Error>>()?;
        let mut observed_holidays = vec![];
        for holiday in holidays.clone() {
            if !(date_is((holiday.clone()).date, "sunday")) {
                continue;
            }
            let mut observed_date = ((holiday.clone()).date) + Duration::days(i64::from(1));
            while (occupied_dates.clone()).contains(&((observed_date).to_string())) {
                observed_date += Duration::days(i64::from(1));
            }
            occupied_dates.push((observed_date).to_string());
            observed_holidays.push(Holiday {
                name: format!("{} (observed)", (holiday.clone()).name.clone()),
                date: observed_date,
                holiday_type: HolidayType::Observed,
                region: None,
            });
        }
        Ok(observed_holidays.clone())
    }
}
