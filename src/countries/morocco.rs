use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![
            Holiday {
                name: "New Year's Day".to_owned(),
                date: parse_date(&(format!("{}-01-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Proclamation of Independence Day".to_owned(),
                date: parse_date(&(format!("{}-01-11", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Amazigh New Year (ⵉⴹ ⵏ ⵢⵉⵏⵏⴰⵢⵔ)".to_owned(),
                date: parse_date(&(format!("{}-01-14", year)))?,
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
                name: "Throne Day".to_owned(),
                date: parse_date(&(format!("{}-07-30", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Oued Ed-Dahab Day".to_owned(),
                date: parse_date(&(format!("{}-08-14", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Revolution Day".to_owned(),
                date: parse_date(&(format!("{}-08-20", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Youth Day".to_owned(),
                date: parse_date(&(format!("{}-08-21", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Green March".to_owned(),
                date: parse_date(&(format!("{}-11-06", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Independence Day".to_owned(),
                date: parse_date(&(format!("{}-11-18", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ];
        if (year) >= (2026) {
            holidays.push(Holiday {
                name: "Unity Day".to_owned(),
                date: parse_date(&(format!("{}-10-31", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok([holidays.clone(), self.variable_holidays(year)?].concat())
    }
    fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = Vec::new();
        for (name, month, day) in [
            ("Islamic New Year", 1, 1),
            ("Birthday of the Prophet Muhammad", 3, 12),
            ("Birthday of the Prophet Muhammad 2", 3, 13),
            ("Eid al-Fitr", 10, 1),
            ("Eid al-Fitr 2", 10, 2),
            ("Eid al-Adha", 12, 10),
            ("Eid al-Adha 2", 12, 11),
        ] {
            let announced = match (year, month, day) {
                (2023, 3, 12) => Some((9, 28)),
                (2023, 3, 13) => Some((9, 29)),
                (2024, 1, 1) => Some((7, 7)),
                (2026, 3, 12) => Some((8, 25)),
                (2026, 3, 13) => Some((8, 26)),
                _ => None,
            };
            let holiday = if let Some((month, day)) = announced {
                super::date(year, month, day)?
            } else {
                let mut hijri = 1444 + (year - 2022);
                let mut converted = self.islamic_to_gregorian(hijri, month, day)?;
                while converted.year() != year {
                    converted = self.islamic_to_gregorian(hijri, month, day)?;
                    hijri -= 1;
                    if converted.year() < 1976 {
                        return Err(Error::UnsupportedYear);
                    }
                }
                converted
            };
            holidays.push(Holiday {
                name: name.into(),
                date: holiday,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays)
    }
    fn islamic_to_gregorian(&self, y: i32, m: i32, d: i32) -> Result<Date, Error> {
        let jd = (11 * y + 3) / 30 + 354 * y + 30 * m - (m - 1) / 2 + d + 1948440 - 385;
        Date::from_num_days_from_ce_opt(jd - 1721425).ok_or(Error::UnsupportedYear)
    }
}
