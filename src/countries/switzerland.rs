use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn regional_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        if (self._region.to_owned()).is_empty() {
            return Ok(vec![]);
        }
        let easter = super::easter(year, false)?;
        let shared_holidays = vec![
            Holiday {
                name: "Neujahr".to_owned(),
                date: parse_date(&(format!("{}-01-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Auffahrt".to_owned(),
                date: (easter) + Duration::days(i64::from(39)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Bundesfeier".to_owned(),
                date: parse_date(&(format!("{}-08-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Weihnachtstag".to_owned(),
                date: parse_date(&(format!("{}-12-25", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ];
        let regionally_different_holidays = IndexMap::from([
            ("Neujahrs nächster tag".to_owned(), super::date(year, 1, 2)?),
            ("Berchtoldstag".to_owned(), super::date(year, 1, 2)?),
            ("Heilige Drei Könige".to_owned(), super::date(year, 1, 6)?),
            (
                "Neuenburger Republik tag".to_owned(),
                super::date(year, 3, 1)?,
            ),
            ("Josefstag".to_owned(), super::date(year, 3, 19)?),
            (
                "Karfreitag".to_owned(),
                (easter) + Duration::days(i64::from(-(2))),
            ),
            (
                "Ostermontag".to_owned(),
                (easter) + Duration::days(i64::from(1)),
            ),
            ("Tag der Arbeit".to_owned(), super::date(year, 5, 1)?),
            (
                "Pfingstmontag".to_owned(),
                (easter) + Duration::days(i64::from(50)),
            ),
            (
                "Fronleichnam".to_owned(),
                (easter) + Duration::days(i64::from(60)),
            ),
            (
                "Bundesfeier nächster tag".to_owned(),
                super::date(year, 8, 2)?,
            ),
            ("Maria Himmelfahrt".to_owned(), super::date(year, 8, 15)?),
            (
                "Genfer Fasten".to_owned(),
                (parse_date(&(format!("first sunday of September {}", year)))?)
                    + Duration::days(i64::from(4)),
            ),
            (
                "Buss- und Bettag".to_owned(),
                parse_date(&(format!("third sunday of September {}", year)))?,
            ),
            (
                "Buss- und Bettag Montag".to_owned(),
                (parse_date(&(format!("third sunday of September {}", year)))?)
                    + Duration::days(i64::from(1)),
            ),
            ("Allerheiligen".to_owned(), super::date(year, 11, 1)?),
            ("Maria Empfängnis".to_owned(), super::date(year, 12, 8)?),
            ("Stephanstag".to_owned(), super::date(year, 12, 26)?),
            (
                "Weihnachtsnächstertag".to_owned(),
                super::date(year, 12, 26)?,
            ),
            ("Genf Republik tag".to_owned(), super::date(year, 12, 31)?),
        ]);
        let current_region = match () {
            _ if self._region == "ch-ag" => {
                vec!["Karfreitag".to_owned()]
            }
            _ if self._region == "ch-ar" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-ai" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Fronleichnam".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-bl" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Tag der Arbeit".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-bs" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Tag der Arbeit".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-be" => {
                vec![
                    "Berchtoldstag".to_owned(),
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-fr" || self._region == "ch-fr-c" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Fronleichnam".to_owned(),
                    "Maria Himmelfahrt".to_owned(),
                    "Allerheiligen".to_owned(),
                    "Maria Empfängnis".to_owned(),
                ]
            }
            _ if self._region == "ch-fr-r" => {
                vec![
                    "Neujahrs nächster tag".to_owned(),
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-ge" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Genfer Fasten".to_owned(),
                    "Genf Republik tag".to_owned(),
                ]
            }
            _ if self._region == "ch-gl" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Allerheiligen".to_owned(),
                    "Maria Empfängnis".to_owned(),
                ]
            }
            _ if self._region == "ch-gr" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-ju" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Tag der Arbeit".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Fronleichnam".to_owned(),
                ]
            }
            _ if self._region == "ch-lu" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Fronleichnam".to_owned(),
                    "Maria Himmelfahrt".to_owned(),
                    "Allerheiligen".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-ne" => {
                vec![
                    "Neuenburger Republik tag".to_owned(),
                    "Karfreitag".to_owned(),
                    "Tag der Arbeit".to_owned(),
                ]
            }
            _ if self._region == "ch-nw" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Fronleichnam".to_owned(),
                    "Maria Himmelfahrt".to_owned(),
                    "Allerheiligen".to_owned(),
                    "Maria Empfängnis".to_owned(),
                ]
            }
            _ if self._region == "ch-ow" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Fronleichnam".to_owned(),
                    "Maria Himmelfahrt".to_owned(),
                    "Allerheiligen".to_owned(),
                    "Maria Empfängnis".to_owned(),
                ]
            }
            _ if self._region == "ch-sh" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-sz" => {
                vec![
                    "Josefstag".to_owned(),
                    "Karfreitag".to_owned(),
                    "Fronleichnam".to_owned(),
                    "Maria Himmelfahrt".to_owned(),
                    "Allerheiligen".to_owned(),
                ]
            }
            _ if self._region == "ch-so" => {
                vec!["Karfreitag".to_owned()]
            }
            _ if self._region == "ch-sg" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Allerheiligen".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-ti" => {
                vec![
                    "Heilige Drei Könige".to_owned(),
                    "Ostermontag".to_owned(),
                    "Maria Himmelfahrt".to_owned(),
                    "Allerheiligen".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-tg" => {
                vec![
                    "Berchtoldstag".to_owned(),
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Tag der Arbeit".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-ur" => {
                vec![
                    "Heilige Drei Könige".to_owned(),
                    "Josefstag".to_owned(),
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Fronleichnam".to_owned(),
                    "Maria Himmelfahrt".to_owned(),
                    "Allerheiligen".to_owned(),
                    "Maria Empfängnis".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ if self._region == "ch-vd" => {
                vec![
                    "Berchtoldstag".to_owned(),
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Buss- und Bettag Montag".to_owned(),
                ]
            }
            _ if self._region == "ch-vs" => {
                vec![
                    "Josefstag".to_owned(),
                    "Fronleichnam".to_owned(),
                    "Maria Himmelfahrt".to_owned(),
                    "Allerheiligen".to_owned(),
                    "Maria Empfängnis".to_owned(),
                ]
            }
            _ if self._region == "ch-zg" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Fronleichnam".to_owned(),
                    "Maria Himmelfahrt".to_owned(),
                    "Allerheiligen".to_owned(),
                    "Maria Empfängnis".to_owned(),
                ]
            }
            _ if self._region == "ch-zh" => {
                vec![
                    "Karfreitag".to_owned(),
                    "Ostermontag".to_owned(),
                    "Tag der Arbeit".to_owned(),
                    "Pfingstmontag".to_owned(),
                    "Buss- und Bettag".to_owned(),
                    "Stephanstag".to_owned(),
                ]
            }
            _ => {
                vec![]
            }
        };
        let regional_holiday_data = (regionally_different_holidays.clone())
            .into_iter()
            .filter(|(key, _)| (current_region.clone()).contains(&(key.clone())))
            .collect::<IndexMap<String, Date>>();
        let mut regional_holidays = vec![];
        for (name, date) in regional_holiday_data.clone() {
            regional_holidays.push(Holiday {
                name: name.clone(),
                date,
                holiday_type: HolidayType::Regional,
                region: Some(self._region.to_owned()),
            });
        }
        if ((["ch-ge".to_owned(), "ch-ne".to_owned()]).contains(&(self._region.to_owned())))
            && (date_is(parse_date(&(format!("{}-01-01", year)))?, "sunday"))
        {
            regional_holidays.push(Holiday {
                name: "Neujahrs nächster tag".to_owned(),
                date: parse_date(&(format!("{}-01-02", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some(self._region.to_owned()),
            });
        }
        if ((["ch-ge".to_owned()]).contains(&(self._region.to_owned())))
            && (date_is(parse_date(&(format!("{}-08-01", year)))?, "sunday"))
        {
            regional_holidays.push(Holiday {
                name: "Bundesfeier nächster tag".to_owned(),
                date: parse_date(&(format!("{}-08-02", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some(self._region.to_owned()),
            });
        }
        if ((["ch-ge".to_owned(), "ch-ne".to_owned()]).contains(&(self._region.to_owned())))
            && (date_is(parse_date(&(format!("{}-12-25", year)))?, "sunday"))
        {
            regional_holidays.push(Holiday {
                name: "Weihnachtsnächstertag".to_owned(),
                date: parse_date(&(format!("{}-12-26", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some(self._region.to_owned()),
            });
        }
        Ok([regional_holidays.clone(), shared_holidays.clone()].concat())
    }
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        if !(self._region.to_owned()).is_empty() {
            return self.regional_holidays(year);
        }
        Ok([
            vec![
                Holiday {
                    name: "Neujahr".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Berchtoldstag".to_owned(),
                    date: parse_date(&(format!("{}-01-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Bundesfeier".to_owned(),
                    date: parse_date(&(format!("{}-08-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Weihnachtstag".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Stephanstag".to_owned(),
                    date: parse_date(&(format!("{}-12-26", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok(vec![
            Holiday {
                name: "Karfreitag".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Ostermontag".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Auffahrt".to_owned(),
                date: (easter) + Duration::days(i64::from(39)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Pfingstmontag".to_owned(),
                date: (easter) + Duration::days(i64::from(50)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
