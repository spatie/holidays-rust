use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "Año Nuevo".to_owned(),
                date: parse_date(&(format!("{}-01-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de la Altagracia".to_owned(),
                date: parse_date(&(format!("{}-01-21", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de Duarte".to_owned(),
                date: parse_date(&(format!("{}-01-26", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de la Independencia".to_owned(),
                date: parse_date(&(format!("{}-02-27", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día del Trabajo".to_owned(),
                date: parse_date(&(format!("{}-05-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de la Restauración".to_owned(),
                date: parse_date(&(format!("{}-08-16", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de las Mercedes".to_owned(),
                date: parse_date(&(format!("{}-09-24", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de la Constitución".to_owned(),
                date: parse_date(&(format!("{}-11-06", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Navidad".to_owned(),
                date: parse_date(&(format!("{}-12-25", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
