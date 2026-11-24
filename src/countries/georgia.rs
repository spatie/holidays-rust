use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "ახალი წელი".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "ახალი წლის სადღესასწაულო დღე".to_owned(),
                    date: parse_date(&(format!("{}-01-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "შობა".to_owned(),
                    date: parse_date(&(format!("{}-01-07", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "ნათლისღება".to_owned(),
                    date: parse_date(&(format!("{}-01-19", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "დედის დღე".to_owned(),
                    date: parse_date(&(format!("{}-03-03", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "ქალთა საერთაშორისო დღე".to_owned(),
                    date: parse_date(&(format!("{}-03-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "ერთიანობის დღე".to_owned(),
                    date: parse_date(&(format!("{}-04-09", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "გამარჯვების დღე".to_owned(),
                    date: parse_date(&(format!("{}-05-09", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "წმ. ანდრია პირველწოდებულის ხსენების დღე".to_owned(),
                    date: parse_date(&(format!("{}-05-12", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "ოჯახის სიწმინდის დღე".to_owned(),
                    date: parse_date(&(format!("{}-05-17", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "დამოუკიდებლობის დღე".to_owned(),
                    date: parse_date(&(format!("{}-05-26", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "მარიამობა".to_owned(),
                    date: parse_date(&(format!("{}-08-28", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "სვეტიცხოვლობა".to_owned(),
                    date: parse_date(&(format!("{}-10-14", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "გიორგობა".to_owned(),
                    date: parse_date(&(format!("{}-11-23", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let orthodox_easter = super::easter(year, true)?;
        Ok(vec![
            Holiday {
                name: "აღდგომა".to_owned(),
                date: orthodox_easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "წითელი პარასკევი".to_owned(),
                date: (orthodox_easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "დიდი შაბათი".to_owned(),
                date: (orthodox_easter) + Duration::days(i64::from(-(1))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "მიცვალებულთა მოხსენიების დღე".to_owned(),
                date: (orthodox_easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
