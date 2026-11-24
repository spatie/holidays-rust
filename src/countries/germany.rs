use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Neujahr".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Tag der Arbeit".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "1. Weihnachtstag".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "2. Weihnachtstag".to_owned(),
                    date: parse_date(&(format!("{}-12-26", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.variable_holidays(year)?,
            self.historical_holidays(year)?,
            self.region_holidays(year)?,
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
                name: "Himmelfahrt".to_owned(),
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
    pub(super) fn historical_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tag der deutschen Einheit".to_owned(),
            date: if (1954..=1990).contains(&(year)) {
                parse_date(&(format!("{}-06-17", year)))?
            } else {
                parse_date(&(format!("{}-10-03", year)))?
            },
            holiday_type: HolidayType::National,
            region: None,
        });
        if (1990..=1994).contains(&(year)) {
            holidays.push(Holiday {
                name: "Buß- und Bettag".to_owned(),
                date: self.get_repentance_and_prayer_day(year)?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        if (year) == (2017) {
            holidays.push(Holiday {
                name: "Reformationstag".to_owned(),
                date: parse_date(&(format!("{}-10-31", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        if self._region == "DE-BW" {
            return Ok(vec![
                Holiday {
                    name: "Heilige Drei Könige".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BW".to_owned()),
                },
                Holiday {
                    name: "Fronleichnam".to_owned(),
                    date: (easter) + Duration::days(i64::from(60)),
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BW".to_owned()),
                },
                Holiday {
                    name: "Allerheiligen".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BW".to_owned()),
                },
            ]);
        }
        if self._region == "DE-BY" {
            let mut by_holidays = vec![
                Holiday {
                    name: "Heilige Drei Könige".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BY".to_owned()),
                },
                Holiday {
                    name: "Fronleichnam".to_owned(),
                    date: (easter) + Duration::days(i64::from(60)),
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BY".to_owned()),
                },
                Holiday {
                    name: "Allerheiligen".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BY".to_owned()),
                },
                Holiday {
                    name: "Mariä Himmelfahrt".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BY".to_owned()),
                },
            ];
            if (1948..=1969).contains(&(year)) {
                by_holidays.push(Holiday {
                    name: "Josefstag".to_owned(),
                    date: parse_date(&(format!("{}-03-19", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BY".to_owned()),
                });
            }
            return Ok(by_holidays.clone());
        }
        if self._region == "DE-BE" {
            let mut be_holidays = vec![];
            if (year) >= (2019) {
                be_holidays.push(Holiday {
                    name: "Internationaler Frauentag".to_owned(),
                    date: parse_date(&(format!("{}-03-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BE".to_owned()),
                });
            }
            if ((year) == (2020)) || ((year) == (2025)) {
                be_holidays.push(Holiday {
                    name: "Tag der Befreiung".to_owned(),
                    date: parse_date(&(format!("{}-05-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BE".to_owned()),
                });
            }
            if (year) == (2028) {
                be_holidays.push(Holiday {
                    name: "75-jähriges Jubiläum des Volksaufstands in der DDR".to_owned(),
                    date: parse_date(&(format!("{}-06-17", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BE".to_owned()),
                });
            }
            return Ok(be_holidays.clone());
        }
        if self._region == "DE-BB" {
            if (year) >= (1991) {
                return Ok(vec![
                    Holiday {
                        name: "Ostersonntag".to_owned(),
                        date: easter,
                        holiday_type: HolidayType::Regional,
                        region: Some("DE-BB".to_owned()),
                    },
                    Holiday {
                        name: "Reformationstag".to_owned(),
                        date: parse_date(&(format!("{}-10-31", year)))?,
                        holiday_type: HolidayType::Regional,
                        region: Some("DE-BB".to_owned()),
                    },
                    Holiday {
                        name: "Pfingstsonntag".to_owned(),
                        date: (easter) + Duration::days(i64::from(49)),
                        holiday_type: HolidayType::Regional,
                        region: Some("DE-BB".to_owned()),
                    },
                ]);
            }
            return Ok(vec![
                Holiday {
                    name: "Ostersonntag".to_owned(),
                    date: easter,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BB".to_owned()),
                },
                Holiday {
                    name: "Pfingstsonntag".to_owned(),
                    date: (easter) + Duration::days(i64::from(49)),
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-BB".to_owned()),
                },
            ]);
        }
        if self._region == "DE-HB"
            || self._region == "DE-HH"
            || self._region == "DE-NI"
            || self._region == "DE-SH"
        {
            if (year) >= (2017) {
                return Ok(vec![Holiday {
                    name: "Reformationstag".to_owned(),
                    date: parse_date(&(format!("{}-10-31", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]);
            }
            return Ok(vec![]);
        }
        if self._region == "DE-HE" {
            return Ok(vec![
                Holiday {
                    name: "Ostersonntag".to_owned(),
                    date: easter,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-HE".to_owned()),
                },
                Holiday {
                    name: "Pfingstsonntag".to_owned(),
                    date: (easter) + Duration::days(i64::from(49)),
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-HE".to_owned()),
                },
                Holiday {
                    name: "Fronleichnam".to_owned(),
                    date: (easter) + Duration::days(i64::from(60)),
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-HE".to_owned()),
                },
            ]);
        }
        if self._region == "DE-MV" {
            let mut mv_holidays = vec![];
            if (year) >= (1990) {
                mv_holidays.push(Holiday {
                    name: "Reformationstag".to_owned(),
                    date: parse_date(&(format!("{}-10-31", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-MV".to_owned()),
                });
            }
            if (year) >= (2023) {
                mv_holidays.push(Holiday {
                    name: "Internationaler Frauentag".to_owned(),
                    date: parse_date(&(format!("{}-03-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-MV".to_owned()),
                });
            }
            return Ok(mv_holidays.clone());
        }
        if self._region == "DE-NW" || self._region == "DE-RP" {
            return Ok(vec![
                Holiday {
                    name: "Fronleichnam".to_owned(),
                    date: (easter) + Duration::days(i64::from(60)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                },
                Holiday {
                    name: "Allerheiligen".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                },
            ]);
        }
        if self._region == "DE-SL" {
            return Ok(vec![
                Holiday {
                    name: "Fronleichnam".to_owned(),
                    date: (easter) + Duration::days(i64::from(60)),
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-SL".to_owned()),
                },
                Holiday {
                    name: "Allerheiligen".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-SL".to_owned()),
                },
                Holiday {
                    name: "Mariä Himmelfahrt".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-SL".to_owned()),
                },
            ]);
        }
        if self._region == "DE-SN" {
            let mut sn_holidays = vec![];
            if (year) >= (1990) {
                sn_holidays.push(Holiday {
                    name: "Reformationstag".to_owned(),
                    date: parse_date(&(format!("{}-10-31", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-SN".to_owned()),
                });
            }
            if (year) > (1994) {
                sn_holidays.push(Holiday {
                    name: "Buß- und Bettag".to_owned(),
                    date: self.get_repentance_and_prayer_day(year)?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-SN".to_owned()),
                });
            }
            return Ok(sn_holidays.clone());
        }
        if self._region == "DE-ST" {
            let mut st_holidays = vec![];
            if (year) >= (1990) {
                st_holidays.push(Holiday {
                    name: "Reformationstag".to_owned(),
                    date: parse_date(&(format!("{}-10-31", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-ST".to_owned()),
                });
            }
            if (year) >= (1991) {
                st_holidays.push(Holiday {
                    name: "Heilige Drei Könige".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-ST".to_owned()),
                });
            }
            return Ok(st_holidays.clone());
        }
        if self._region == "DE-TH" {
            let mut th_holidays = vec![];
            if (year) >= (1990) {
                th_holidays.push(Holiday {
                    name: "Reformationstag".to_owned(),
                    date: parse_date(&(format!("{}-10-31", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-TH".to_owned()),
                });
            }
            if (year) >= (2019) {
                th_holidays.push(Holiday {
                    name: "Weltkindertag".to_owned(),
                    date: parse_date(&(format!("{}-09-20", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("DE-TH".to_owned()),
                });
            }
            return Ok(th_holidays.clone());
        }
        Ok(vec![])
    }
    pub(super) fn get_repentance_and_prayer_day(&self, year: i32) -> Result<Date, Error> {
        parse_date(&(format!("next wednesday {}-11-15", year)))
    }
}
