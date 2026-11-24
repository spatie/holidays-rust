use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Yeni il".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Beynəlxalq Qadınlar günü".to_owned(),
                    date: parse_date(&(format!("{}-03-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Novruz bayramı".to_owned(),
                    date: parse_date(&(format!("{}-03-20", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Faşizm üzərində qələbə günü".to_owned(),
                    date: parse_date(&(format!("{}-05-09", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Müstəqillik Günü".to_owned(),
                    date: parse_date(&(format!("{}-05-28", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Azərbaycan xalqının milli qurtuluş günü".to_owned(),
                    date: parse_date(&(format!("{}-06-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Azərbaycan Respublikasının Silahlı Qüvvələri günü".to_owned(),
                    date: parse_date(&(format!("{}-06-26", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Müstəqilliyin bərpası günü".to_owned(),
                    date: parse_date(&(format!("{}-10-18", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Zəfər Günü".to_owned(),
                    date: parse_date(&(format!("{}-11-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Azərbaycan Respublikasının Dövlət bayrağı günü".to_owned(),
                    date: parse_date(&(format!("{}-11-09", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dünya azərbaycanlılarının həmrəyliyi günü".to_owned(),
                    date: parse_date(&(format!("{}-12-31", year)))?,
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
