use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Første nyttårsdag".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Arbeidernes dag".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Grunnlovsdag".to_owned(),
                    date: parse_date(&(format!("{}-05-17", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Første juledag".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Andre juledag".to_owned(),
                    date: parse_date(&(format!("{}-12-26", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok(vec![
            Holiday {
                name: "Skjærtorsdag".to_owned(),
                date: (easter) + Duration::days(i64::from(-(3))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Langfredag".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Første påskedag".to_owned(),
                date: easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Andre påskedag,".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Kristi Himmelfartsdag".to_owned(),
                date: (easter) + Duration::days(i64::from(39)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Første pinsedag".to_owned(),
                date: (easter) + Duration::days(i64::from(49)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Andre pinsedag".to_owned(),
                date: (easter) + Duration::days(i64::from(50)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
