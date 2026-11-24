use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Nieuwjaarsdag".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Eerste kerstdag".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Tweede kerstdag".to_owned(),
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
        let mut konings_dag = super::date(year, 4, 27)?;
        if date_is(konings_dag, "sunday") {
            konings_dag += Duration::days(i64::from(-(1)));
        }
        let easter = super::easter(year, false)?;
        let mut holidays = vec![
            Holiday {
                name: "Koningsdag".to_owned(),
                date: konings_dag,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Eerste paasdag".to_owned(),
                date: easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Tweede paasdag".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Hemelvaartsdag".to_owned(),
                date: (easter) + Duration::days(i64::from(39)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Eerste pinksterdag".to_owned(),
                date: (easter) + Duration::days(i64::from(49)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Tweede pinksterdag".to_owned(),
                date: (easter) + Duration::days(i64::from(50)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ];
        if ((year) % (5)) == (0) {
            holidays.push(Holiday {
                name: "Bevrijdingsdag".to_owned(),
                date: parse_date(&(format!("{}-05-05", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays.clone())
    }
}
