use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Нова година".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Божик, првиот ден на Божик според православниот календар".to_owned(),
                    date: parse_date(&(format!("{}-01-07", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ден на трудот".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Св. Кирил и Методиј - Ден на сесловенските просветители".to_owned(),
                    date: parse_date(&(format!("{}-05-24", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ден на Републиката".to_owned(),
                    date: parse_date(&(format!("{}-08-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ден на независноста".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ден на народното востание".to_owned(),
                    date: parse_date(&(format!("{}-10-11", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Ден на македонската револуционерна борба".to_owned(),
                    date: parse_date(&(format!("{}-10-23", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Св. Климент Охридски".to_owned(),
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
        let easter = super::easter(year, true)?;
        Ok(vec![Holiday {
            name: "Велигден, вториот ден на Велигден според православниот календар".to_owned(),
            date: (easter) + Duration::days(i64::from(1)),
            holiday_type: HolidayType::National,
            region: None,
        }])
    }
}
