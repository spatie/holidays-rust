use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Kashmir Solidarity Day".to_owned(),
                    date: parse_date(&(format!("{}-02-05", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Pakistan Day".to_owned(),
                    date: parse_date(&(format!("{}-03-23", year)))?,
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
                    name: "Independence Day".to_owned(),
                    date: parse_date(&(format!("{}-08-14", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Iqbal Day".to_owned(),
                    date: parse_date(&(format!("{}-11-09", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Quaid-e-Azam Day".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, _year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![])
    }
}
