use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Jour de l'An".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Fête du Travail".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Victoire 1945".to_owned(),
                    date: parse_date(&(format!("{}-05-08", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Fête Nationale".to_owned(),
                    date: parse_date(&(format!("{}-07-14", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Assomption".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Toussaint".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Armistice 1918".to_owned(),
                    date: parse_date(&(format!("{}-11-11", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Noël".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
            self.region_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        let mut holidays = vec![
            Holiday {
                name: "Lundi de Pâques".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Ascension".to_owned(),
                date: (easter) + Duration::days(i64::from(39)),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Lundi de Pentecôte".to_owned(),
                date: (easter) + Duration::days(i64::from(50)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ];
        if (["FR-57".to_owned(), "FR-67".to_owned(), "FR-68".to_owned()])
            .contains(&(self._region.to_owned()))
        {
            holidays.push(Holiday {
                name: "Vendredi Saint".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "FR-57" || self._region == "FR-67" || self._region == "FR-68" => {
                vec![Holiday {
                    name: "Saint-Étienne".to_owned(),
                    date: parse_date(&(format!("{}-12-26", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }]
            }
            _ if self._region == "FR-971" || self._region == "FR-MF" => {
                vec![Holiday {
                    name: "Abolition de l'esclavage".to_owned(),
                    date: parse_date(&(format!("{}-05-27", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }]
            }
            _ if self._region == "FR-972" => {
                vec![Holiday {
                    name: "Abolition de l'esclavage".to_owned(),
                    date: parse_date(&(format!("{}-05-22", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }]
            }
            _ if self._region == "FR-973" => {
                vec![Holiday {
                    name: "Abolition de l'esclavage".to_owned(),
                    date: parse_date(&(format!("{}-06-10", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }]
            }
            _ if self._region == "FR-974" => {
                vec![Holiday {
                    name: "Abolition de l'esclavage".to_owned(),
                    date: parse_date(&(format!("{}-12-20", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }]
            }
            _ if self._region == "FR-976" => {
                vec![Holiday {
                    name: "Abolition de l'esclavage".to_owned(),
                    date: parse_date(&(format!("{}-04-27", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }]
            }
            _ if self._region == "FR-BL" => {
                vec![Holiday {
                    name: "Abolition de l'esclavage".to_owned(),
                    date: parse_date(&(format!("{}-10-09", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                }]
            }
            _ => {
                vec![]
            }
        })
    }
}
