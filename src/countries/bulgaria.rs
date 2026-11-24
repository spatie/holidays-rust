use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let fixed_holidays = self.fixed_holidays(year)?;
        Ok([
            fixed_holidays.clone(),
            if year >= 2017 {
                self.substitute_holidays(fixed_holidays.clone())?
            } else {
                Vec::new()
            },
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn fixed_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![Holiday { name: "Нова година".to_owned(), date: parse_date(&(format!("{}-01-01", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Ден на Освобождението на България от османско иго".to_owned(), date: parse_date(&(format!("{}-03-03", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Ден на труда и на международната работническа солидарност".to_owned(), date: parse_date(&(format!("{}-05-01", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Гергьовден, Ден на храбростта и Българската армия".to_owned(), date: parse_date(&(format!("{}-05-06", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност".to_owned(), date: parse_date(&(format!("{}-05-24", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Ден на Съединението".to_owned(), date: parse_date(&(format!("{}-09-06", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Ден на Независимостта на България".to_owned(), date: parse_date(&(format!("{}-09-22", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Бъдни вечер".to_owned(), date: parse_date(&(format!("{}-12-24", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Рождество Христово".to_owned(), date: parse_date(&(format!("{}-12-25", year)))?, holiday_type: HolidayType::National, region: None }, Holiday { name: "Вторият ден на Рождество Христово".to_owned(), date: parse_date(&(format!("{}-12-26", year)))?, holiday_type: HolidayType::National, region: None }])
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, true)?;
        Ok(vec![
            Holiday {
                name: "Разпети петък".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Велика събота".to_owned(),
                date: (easter) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Великден".to_owned(),
                date: easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Вторият ден на Великден".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn substitute_holidays(
        &self,
        mut fixed_holidays: Vec<Holiday>,
    ) -> Result<Vec<Holiday>, Error> {
        fixed_holidays.sort_by_key(|h| h.date);
        let holiday_dates = (fixed_holidays.clone())
            .into_iter()
            .map(|holiday| -> Result<_, Error> { Ok(((holiday.clone()).date).to_string()) })
            .collect::<Result<Vec<_>, Error>>()?;
        let mut used_dates = vec![];
        let mut substitutes = vec![];
        for holiday in fixed_holidays.clone() {
            if !(date_is((holiday.clone()).date, "weekend")) {
                continue;
            }
            let mut substitute_date = ((holiday.clone()).date) + Duration::days(i64::from(1));
            while ((date_is(substitute_date, "weekend"))
                || ((holiday_dates.clone()).contains(&((substitute_date).to_string()))))
                || ((used_dates.clone()).contains(&((substitute_date).to_string())))
            {
                substitute_date += Duration::days(i64::from(1));
            }
            used_dates.push((substitute_date).to_string());
            substitutes.push(Holiday {
                name: format!("{} (неприсъствен ден)", (holiday.clone()).name.clone()),
                date: substitute_date,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(substitutes.clone())
    }
}
