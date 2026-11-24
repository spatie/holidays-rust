use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([vec![Holiday { name: "Новий Рік".to_owned(), date: parse_date(&(format!("{}-01-01", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Міжнародний жіночий день".to_owned(), date: parse_date(&(format!("{}-03-08", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "День праці".to_owned(), date: parse_date(&(format!("{}-05-01", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "День пам'яті та перемоги над нацизмом у Другій світовій війні 1939 – 1945 років".to_owned(), date: parse_date(&(format!("{}-05-08", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "День Конституції України".to_owned(), date: parse_date(&(format!("{}-06-28", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "День Української Державності".to_owned(), date: parse_date(&(format!("{}-07-15", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "День Незалежності України".to_owned(), date: parse_date(&(format!("{}-08-24", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "День захисників і захисниць України".to_owned(), date: parse_date(&(format!("{}-10-01", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Різдво Христове".to_owned(), date: parse_date(&(format!("{}-12-25", year)))?, holiday_type: HolidayType::National, region: None }], self.variable_holidays(year)?].concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, true)?;
        Ok(vec![
            Holiday {
                name: "Великодній Понеділок".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Трійця".to_owned(),
                date: (easter) + Duration::days(i64::from(50)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
