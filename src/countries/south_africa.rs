use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let holidays = IndexMap::from([
            ("New Year's Day".to_owned(), super::date(year, 1, 1)?),
            ("Human Rights Day".to_owned(), super::date(year, 3, 21)?),
            ("Freedom Day".to_owned(), super::date(year, 4, 27)?),
            ("Workers' Day".to_owned(), super::date(year, 5, 1)?),
            ("Youth Day".to_owned(), super::date(year, 6, 16)?),
            ("National Women's Day".to_owned(), super::date(year, 8, 9)?),
            ("Heritage Day".to_owned(), super::date(year, 9, 24)?),
            (
                "Day of Reconciliation".to_owned(),
                super::date(year, 12, 16)?,
            ),
            ("Christmas Day".to_owned(), super::date(year, 12, 25)?),
            ("Day of Goodwill".to_owned(), super::date(year, 12, 26)?),
        ]);
        let mut result = vec![];
        for (name, date) in holidays.clone() {
            result.push(Holiday {
                name: name.clone(),
                date,
                holiday_type: HolidayType::National,
                region: None,
            });
            let observed_day = sunday_to_next_monday(date);
            if (observed_day).is_some() {
                result.push(Holiday {
                    name: format!("{} Observed", name.clone()),
                    date: (observed_day).ok_or_else(|| Error::Calculation("SouthAfrica".into()))?,
                    holiday_type: HolidayType::National,
                    region: None,
                });
            }
        }
        Ok([result.clone(), self.variable_holidays(year)?].concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok(vec![
            Holiday {
                name: "Good Friday".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Family Day".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
