use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([vec![Holiday { name: "Nova godina".to_owned(), date: parse_date(&(format!("{}-01-01", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Bogojavljenje ili Sveta tri kralja".to_owned(), date: parse_date(&(format!("{}-01-06", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Praznik rada".to_owned(), date: parse_date(&(format!("{}-05-01", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Dan državnosti".to_owned(), date: parse_date(&(format!("{}-05-30", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Dan antifašističke borbe".to_owned(), date: parse_date(&(format!("{}-06-22", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja".to_owned(), date: parse_date(&(format!("{}-08-05", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Velika Gospa".to_owned(), date: parse_date(&(format!("{}-08-15", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Svi sveti".to_owned(), date: parse_date(&(format!("{}-11-01", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje".to_owned(), date: parse_date(&(format!("{}-11-18", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Božić".to_owned(), date: parse_date(&(format!("{}-12-25", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Sveti Stjepan".to_owned(), date: parse_date(&(format!("{}-12-26", year)))?, holiday_type: HolidayType::National, region: None }], self.variable_holidays(year)?].concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok(vec![
            Holiday {
                name: "Uskrs".to_owned(),
                date: easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Uskrsni ponedjeljak".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Tijelovo".to_owned(),
                date: (easter) + Duration::days(i64::from(60)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
