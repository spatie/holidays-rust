use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn eid_al_fitr_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
            (2024, vec!["04-10".to_owned()]),
            (2025, vec!["03-30".to_owned()]),
            (2026, vec!["03-20".to_owned()]),
            (2027, vec!["03-09".to_owned()]),
            (2028, vec!["02-26".to_owned()]),
            (2029, vec!["02-14".to_owned()]),
            (2030, vec!["02-04".to_owned()]),
            (2031, vec!["01-24".to_owned()]),
            (2032, vec!["01-14".to_owned()]),
            (2033, vec!["01-02".to_owned()]),
            (2034, vec!["12-12".to_owned()]),
        ]))
    }
    pub(super) fn eid_al_adha_dates(&self) -> Result<Table, Error> {
        Ok(IndexMap::from([
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
        ]))
    }
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Viti i Ri".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dita e Verës".to_owned(),
                    date: parse_date(&(format!("{}-03-14", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dita e Sulltan Nevruzit".to_owned(),
                    date: parse_date(&(format!("{}-03-22", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dita Ndërkombëtare e Punëtorëve".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dita e Shenjtërimit të Shenjt Terezës".to_owned(),
                    date: parse_date(&(format!("{}-09-05", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dita e Pavarësisë".to_owned(),
                    date: parse_date(&(format!("{}-11-28", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dita e Çlirimit".to_owned(),
                    date: parse_date(&(format!("{}-11-29", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dita Kombëtare e Rinisë".to_owned(),
                    date: parse_date(&(format!("{}-12-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Krishtlindja".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
            self.islamic_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "E diela e Pashkëve Katolike".to_owned(),
                date: super::easter(year, false)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "E diela e Pashkëve Ortodokse".to_owned(),
                date: super::easter(year, true)?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn islamic_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        if !(2024..=2034).contains(&(year)) {
            return Err(Error::UnsupportedYear);
        }
        let mut dates = self.eid_al_fitr_dates()?;
        let eid_al_fitr_date = (dates.clone())
            .get(&(year))
            .and_then(|values| values.first())
            .cloned()
            .unwrap_or_default();
        dates = self.eid_al_adha_dates()?;
        let eid_al_adha_date = (dates.clone())
            .get(&(year))
            .and_then(|values| values.first())
            .cloned()
            .unwrap_or_default();
        Ok(vec![
            Holiday {
                name: "Dita e Bajramit të Madh".to_owned(),
                date: parse_date(&(format!("{}-{}", year, eid_al_fitr_date.clone())))?,
                holiday_type: HolidayType::Religious,
                region: None,
            },
            Holiday {
                name: "Dita e Kurban Bajramit".to_owned(),
                date: parse_date(&(format!("{}-{}", year, eid_al_adha_date.clone())))?,
                holiday_type: HolidayType::Religious,
                region: None,
            },
        ])
    }
}
