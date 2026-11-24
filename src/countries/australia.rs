use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "New Year's Day".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Australia Day".to_owned(),
                    date: parse_date(&(format!("{}-01-26", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Anzac Day".to_owned(),
                    date: parse_date(&(format!("{}-04-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Christmas Day".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Boxing Day".to_owned(),
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
        Ok([
            vec![Holiday {
                name: "Good Friday".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            }],
            vec![Holiday {
                name: "Easter Monday".to_owned(),
                date: (easter) + Duration::days(i64::from(1)),
                holiday_type: HolidayType::National,
                region: None,
            }],
            match () {
                _ if self._region == "act" => [
                    vec![Holiday {
                        name: "Canberra Day".to_owned(),
                        date: parse_date(&(format!("second monday of march {}", year)))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    }],
                    vec![Holiday {
                        name: "Easter Saturday".to_owned(),
                        date: (easter) + Duration::days(i64::from(-(1))),
                        holiday_type: HolidayType::National,
                        region: None,
                    }],
                    vec![Holiday {
                        name: "Easter Sunday".to_owned(),
                        date: easter,
                        holiday_type: HolidayType::National,
                        region: None,
                    }],
                    vec![Holiday {
                        name: "Reconciliation Day".to_owned(),
                        date: date_modify(super::date(year, 5, 27)?, "monday".to_owned())?,
                        holiday_type: HolidayType::National,
                        region: None,
                    }],
                    vec![Holiday {
                        name: self.sovereign_birthday_key(year)?,
                        date: parse_date(&(format!("second monday of june {}", year)))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    }],
                    vec![Holiday {
                        name: "Labour Day".to_owned(),
                        date: parse_date(&(format!("first monday of october {}", year)))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    }],
                ]
                .concat(),
                _ if self._region == "nsw" => {
                    vec![
                        Holiday {
                            name: "Easter Saturday".to_owned(),
                            date: (easter) + Duration::days(i64::from(-(1))),
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: "Easter Sunday".to_owned(),
                            date: easter,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: self.sovereign_birthday_key(year)?,
                            date: parse_date(&(format!("second monday of june {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: "Labour Day".to_owned(),
                            date: parse_date(&(format!("first monday of october {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                    ]
                }
                _ if self._region == "nt" => {
                    vec![
                        Holiday {
                            name: "Easter Saturday".to_owned(),
                            date: (easter) + Duration::days(i64::from(-(1))),
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: "May Day".to_owned(),
                            date: parse_date(&(format!("first monday of may {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: "Picnic Day".to_owned(),
                            date: parse_date(&(format!("first monday of august {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                    ]
                }
                _ if self._region == "qld" => {
                    vec![
                        Holiday {
                            name: "The day after Good Friday".to_owned(),
                            date: (easter) + Duration::days(i64::from(-(1))),
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: "Easter Sunday".to_owned(),
                            date: easter,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: "Labour Day".to_owned(),
                            date: parse_date(&(format!("first monday of may {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: self.sovereign_birthday_key(year)?,
                            date: parse_date(&(format!("first monday of october {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                    ]
                }
                _ if self._region == "sa" => {
                    vec![
                        Holiday {
                            name: "Adelaide Cup Day".to_owned(),
                            date: if (year) < (2006) {
                                parse_date(&(format!("third monday of may {}", year)))?
                            } else {
                                parse_date(&(format!("second monday of march {}", year)))?
                            },
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: "The day after Good Friday".to_owned(),
                            date: (easter) + Duration::days(i64::from(-(1))),
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: self.sovereign_birthday_key(year)?,
                            date: parse_date(&(format!("second monday of june {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: "Labour Day".to_owned(),
                            date: parse_date(&(format!("first monday of october {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                    ]
                }
                _ if self._region == "tas" => {
                    vec![
                        Holiday {
                            name: "Eight Hours Day".to_owned(),
                            date: parse_date(&(format!("second monday of march {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: self.sovereign_birthday_key(year)?,
                            date: parse_date(&(format!("second monday of june {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                    ]
                }
                _ if self._region == "vic" => [
                    vec![Holiday {
                        name: "Labour Day".to_owned(),
                        date: parse_date(&(format!("second monday of march {}", year)))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    }],
                    vec![Holiday {
                        name: "Saturday before Easter Sunday".to_owned(),
                        date: (easter) + Duration::days(i64::from(-(1))),
                        holiday_type: HolidayType::National,
                        region: None,
                    }],
                    vec![Holiday {
                        name: "Easter Sunday".to_owned(),
                        date: easter,
                        holiday_type: HolidayType::National,
                        region: None,
                    }],
                    vec![Holiday {
                        name: self.sovereign_birthday_key(year)?,
                        date: parse_date(&(format!("second monday of june {}", year)))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    }],
                    if (self.friday_before_afl_grand_final(year)?).is_some() {
                        vec![Holiday {
                            name: "Friday before AFL Grand Final".to_owned(),
                            date: (self.friday_before_afl_grand_final(year)?)
                                .ok_or_else(|| Error::Calculation("Australia".into()))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        }]
                    } else {
                        vec![]
                    },
                    vec![Holiday {
                        name: "Melbourne Cup".to_owned(),
                        date: parse_date(&(format!("first tuesday of november {}", year)))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    }],
                ]
                .concat(),
                _ if self._region == "wa" => {
                    vec![
                        Holiday {
                            name: "Labour Day".to_owned(),
                            date: parse_date(&(format!("first monday of march {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: "Easter Sunday".to_owned(),
                            date: easter,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: "Western Australia Day".to_owned(),
                            date: parse_date(&(format!("first monday of june {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                        Holiday {
                            name: self.sovereign_birthday_key(year)?,
                            date: parse_date(&(format!("last monday of september {}", year)))?,
                            holiday_type: HolidayType::National,
                            region: None,
                        },
                    ]
                }
                _ => {
                    vec![]
                }
            },
        ]
        .concat())
    }
    pub(super) fn friday_before_afl_grand_final(&self, year: i32) -> Result<Option<Date>, Error> {
        if (year) < (2015) {
            return Ok(None);
        }
        Ok(match () {
            _ if (year) == (2015) => Some(parse_date("2015-10-02")?),
            _ if (year) == (2020) => Some(parse_date("2020-10-23")?),
            _ if (year) == (2022) => Some(parse_date("2022-09-23")?),
            _ => Some(parse_date(&(format!("last friday of september {}", year)))?),
        })
    }
    pub(super) fn sovereign_birthday_key(&self, year: i32) -> Result<String, Error> {
        let change_year = if self._region == "wa" { 2022 } else { 2023 };
        Ok(if (year) >= (change_year) {
            "King's Birthday".to_owned()
        } else {
            "Queen's Birthday".to_owned()
        })
    }
}
