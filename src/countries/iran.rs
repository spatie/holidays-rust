use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "پیروزی انقلاب اسلامی پنجاه و هفت".to_owned(),
                date: parse_date(&(format!("{}-02-11", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "روز ملی شدن صنعت نفت".to_owned(),
                date: parse_date(&(format!("{}-03-19", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "نخستین روز نوروز".to_owned(),
                date: parse_date(&(format!("{}-03-20", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "دومین روز نوروز".to_owned(),
                date: parse_date(&(format!("{}-03-21", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "سومین روز نوروز".to_owned(),
                date: parse_date(&(format!("{}-03-22", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "چهارمین روز نوروز".to_owned(),
                date: parse_date(&(format!("{}-03-23", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "روز جمهوری اسلامی".to_owned(),
                date: parse_date(&(format!("{}-03-31", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "سیزده بدر".to_owned(),
                date: parse_date(&(format!("{}-04-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "رحلت روح‌الله خمینی".to_owned(),
                date: parse_date(&(format!("{}-06-03", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "قیام ۱۵ خرداد".to_owned(),
                date: parse_date(&(format!("{}-06-04", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
