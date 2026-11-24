use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Nova godina - prvi dan".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Nova godina - drugi dan".to_owned(),
                    date: parse_date(&(format!("{}-01-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Badnji dan (za pravoslavce)".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Božić (za pravoslavce)".to_owned(),
                    date: parse_date(&(format!("{}-01-07", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Praznik rada - prvi dan".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Praznik rada - drugi dan".to_owned(),
                    date: parse_date(&(format!("{}-05-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Badnji dan (za rimokatolike)".to_owned(),
                    date: parse_date(&(format!("{}-12-24", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Božić (za rimokatolike)".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
            ],
            self.region_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn region_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "ba-rs" => {
                vec![
                    Holiday {
                        name: "Dan Republike".to_owned(),
                        date: parse_date(&(format!("{}-01-09", year)))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    },
                    Holiday {
                        name: "Dan pobjede nad fašizmom".to_owned(),
                        date: parse_date(&(format!("{}-05-09", year)))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    },
                    Holiday {
                        name: "Dan uspostavljanja Opšteg okvirnog sporazuma za mir u BiH"
                            .to_owned(),
                        date: parse_date(&(format!("{}-11-21", year)))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    },
                ]
            }
            _ if self._region == "ba-fbih" => {
                vec![
                    Holiday {
                        name: "Dan nezavisnosti Bosne i Hercegovine".to_owned(),
                        date: parse_date(&(format!("{}-03-01", year)))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    },
                    Holiday {
                        name: "Dan državnosti Bosne i Hercegovine".to_owned(),
                        date: parse_date(&(format!("{}-11-25", year)))?,
                        holiday_type: HolidayType::National,
                        region: None,
                    },
                ]
            }
            _ if self._region == "ba-bd" => {
                vec![Holiday {
                    name: "Dan uspostavljanja Brčko distrikta".to_owned(),
                    date: parse_date(&(format!("{}-03-08", year)))?,
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
