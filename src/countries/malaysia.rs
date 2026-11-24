use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Hari Pekerja".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Hari Kebangsaan".to_owned(),
                    date: parse_date(&(format!("{}-08-31", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Hari Malaysia".to_owned(),
                    date: parse_date(&(format!("{}-09-16", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Hari Krismas".to_owned(),
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
        let mut holidays = vec![];
        let chinese_new_year1 = chinese_to_gregorian_date("01-01", year)?;
        holidays.push(Holiday {
            name: "Tahun Baru Cina".to_owned(),
            date: chinese_new_year1,
            holiday_type: HolidayType::National,
            region: None,
        });
        let chinese_new_year2 = chinese_to_gregorian_date("01-02", year)?;
        holidays.push(Holiday {
            name: "Tahun Baru Cina Hari Kedua".to_owned(),
            date: chinese_new_year2,
            holiday_type: HolidayType::National,
            region: None,
        });
        let raya_aidilfitri = islamic_to_gregorian_date("10-01", year, false)?;
        holidays.push(Holiday {
            name: "Hari Raya Aidilfitri".to_owned(),
            date: raya_aidilfitri,
            holiday_type: HolidayType::Religious,
            region: None,
        });
        let raya_aidilfitri2 = islamic_to_gregorian_date("10-02", year, false)?;
        holidays.push(Holiday {
            name: "Hari Raya Aidilfitri Hari Kedua".to_owned(),
            date: raya_aidilfitri2,
            holiday_type: HolidayType::Religious,
            region: None,
        });
        let hari_wesak = self.hari_wesak(year)?;
        if (hari_wesak).is_some() {
            holidays.push(Holiday {
                name: "Hari Wesak".to_owned(),
                date: (hari_wesak).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        let raya_aidiladha = islamic_to_gregorian_date("12-10", year, false)?;
        holidays.push(Holiday {
            name: "Hari Raya Aidiladha".to_owned(),
            date: raya_aidiladha,
            holiday_type: HolidayType::Religious,
            region: None,
        });
        let awal_muharram = islamic_to_gregorian_date("01-01", year, true)?;
        holidays.push(Holiday {
            name: "Awal Muharram".to_owned(),
            date: awal_muharram,
            holiday_type: HolidayType::Religious,
            region: None,
        });
        let maulidur_rasul = islamic_to_gregorian_date("03-12", year, true)?;
        holidays.push(Holiday {
            name: "Maulidur Rasul".to_owned(),
            date: maulidur_rasul,
            holiday_type: HolidayType::Religious,
            region: None,
        });
        let hari_keputeraan_ydp = self.hari_keputeraan_ydp(year)?;
        if (hari_keputeraan_ydp).is_some() {
            holidays.push(Holiday {
                name: "Hari Keputeraan Yang Di-Pertuan Agong".to_owned(),
                date: (hari_keputeraan_ydp).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        self.holidays_by_region(year)
    }
    pub(super) fn holidays_by_region(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let holidays = match () {
            _ if self._region == "jhr" => self.region_johor(year)?,
            _ if self._region == "kdh" => self.region_kedah(year)?,
            _ if self._region == "ktn" => self.region_kelantan(year)?,
            _ if self._region == "kul" => self.region_kuala_lumpur(year)?,
            _ if self._region == "lbn" => self.region_labuan(year)?,
            _ if self._region == "mlk" => self.region_melaka(year)?,
            _ if self._region == "nsn" => self.region_negeri9(year)?,
            _ if self._region == "phg" => self.region_pahang(year)?,
            _ if self._region == "png" => self.region_penang(year)?,
            _ if self._region == "prk" => self.region_perak(year)?,
            _ if self._region == "pls" => self.region_perlis(year)?,
            _ if self._region == "pjy" => self.region_putrajaya(year)?,
            _ if self._region == "sbh" => self.region_sabah(year)?,
            _ if self._region == "swk" => self.region_sarawak(year)?,
            _ if self._region == "sgr" => self.region_selangor(year)?,
            _ if self._region == "trg" => self.region_terengganu(year)?,
            _ => [
                self.region_johor(year)?,
                self.region_kedah(year)?,
                self.region_kelantan(year)?,
                self.region_kuala_lumpur(year)?,
                self.region_labuan(year)?,
                self.region_melaka(year)?,
                self.region_negeri9(year)?,
                self.region_pahang(year)?,
                self.region_penang(year)?,
                self.region_perak(year)?,
                self.region_perlis(year)?,
                self.region_putrajaya(year)?,
                self.region_sabah(year)?,
                self.region_sarawak(year)?,
                self.region_selangor(year)?,
                self.region_terengganu(year)?,
            ]
            .concat(),
        };
        self.unique_holidays(holidays.clone())
    }
    pub(super) fn unique_holidays(&self, holidays: Vec<Holiday>) -> Result<Vec<Holiday>, Error> {
        let mut seen = std::collections::HashSet::new();
        Ok(holidays
            .into_iter()
            .filter(|h| seen.insert((h.name.clone(), h.date)))
            .collect())
    }
    pub(super) fn region_johor(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Hari Keputeraan Sultan Johor".to_owned(),
            date: parse_date(&(format!("{}-03-23", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("jhr".to_owned()),
        });
        let hari_thaipusam = self.hari_thaipusam(year)?;
        if (hari_thaipusam).is_some() {
            holidays.push(Holiday {
                name: "Hari Thaipusam".to_owned(),
                date: (hari_thaipusam).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("jhr".to_owned()),
            });
        }
        let awal_ramadan = islamic_to_gregorian_date("09-01", year, false)?;
        holidays.push(Holiday {
            name: "Awal Ramadan".to_owned(),
            date: awal_ramadan,
            holiday_type: HolidayType::Regional,
            region: Some("jhr".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("jhr".to_owned()),
            });
        }
        let hari_hol_johor = match () {
            _ if (year) == (2020) => Some(super::date(year, 9, 24)?),
            _ if (year) == (2021) => Some(super::date(year, 9, 13)?),
            _ if (year) == (2022) => Some(super::date(year, 9, 3)?),
            _ if (year) == (2023) => Some(super::date(year, 8, 23)?),
            _ if (year) == (2024) => Some(super::date(year, 8, 11)?),
            _ if (year) == (2025) => Some(super::date(year, 7, 31)?),
            _ if (year) == (2026) => Some(super::date(year, 7, 21)?),
            _ => None,
        };
        if (hari_hol_johor).is_some() {
            holidays.push(Holiday {
                name: "Hari Hol Johor".to_owned(),
                date: (hari_hol_johor).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("jhr".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_kedah(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        let hari_thaipusam = self.hari_thaipusam(year)?;
        if (hari_thaipusam).is_some() {
            holidays.push(Holiday {
                name: "Hari Thaipusam".to_owned(),
                date: (hari_thaipusam).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("kdh".to_owned()),
            });
        }
        let israk_mikraj = islamic_to_gregorian_date("07-27", year, false)?;
        holidays.push(Holiday {
            name: "Israk dan Mikraj".to_owned(),
            date: israk_mikraj,
            holiday_type: HolidayType::Regional,
            region: Some("kdh".to_owned()),
        });
        let awal_ramadan = islamic_to_gregorian_date("09-01", year, false)?;
        holidays.push(Holiday {
            name: "Awal Ramadan".to_owned(),
            date: awal_ramadan,
            holiday_type: HolidayType::Regional,
            region: Some("kdh".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Keputeraan Sultan Kedah".to_owned(),
            date: parse_date(&(format!("third sunday of june {}", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("kdh".to_owned()),
        });
        let raya_aidiladha2 = islamic_to_gregorian_date("12-11", year, false)?;
        holidays.push(Holiday {
            name: "Hari Raya Aidiladha Hari Kedua".to_owned(),
            date: raya_aidiladha2,
            holiday_type: HolidayType::Regional,
            region: Some("kdh".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("kdh".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_kelantan(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        let nurul_al_quran = islamic_to_gregorian_date("09-17", year, false)?;
        holidays.push(Holiday {
            name: "Hari Nurul Al-Quran".to_owned(),
            date: nurul_al_quran,
            holiday_type: HolidayType::Regional,
            region: Some("ktn".to_owned()),
        });
        let hari_arafah = islamic_to_gregorian_date("12-09", year, false)?;
        holidays.push(Holiday {
            name: "Hari Arafah".to_owned(),
            date: hari_arafah,
            holiday_type: HolidayType::Regional,
            region: Some("ktn".to_owned()),
        });
        let raya_aidiladha2 = islamic_to_gregorian_date("12-11", year, false)?;
        holidays.push(Holiday {
            name: "Hari Raya Aidiladha Hari Kedua".to_owned(),
            date: raya_aidiladha2,
            holiday_type: HolidayType::Regional,
            region: Some("ktn".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("ktn".to_owned()),
            });
        }
        let king_birthday = match () {
            _ if (((2010..=2022).collect::<Vec<_>>()).contains(&(year))) => {
                Some(super::date(year, 11, 11)?)
            }
            _ if (((2023..=2026).collect::<Vec<_>>()).contains(&(year))) => {
                Some(super::date(year, 9, 29)?)
            }
            _ => None,
        };
        if (king_birthday).is_some() {
            holidays.push(Holiday {
                name: "Hari Keputeraan Sultan Kelantan".to_owned(),
                date: (king_birthday).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("ktn".to_owned()),
            });
        }
        let king_birthday_second_day = match () {
            _ if (((2010..=2022).collect::<Vec<_>>()).contains(&(year))) => {
                Some(super::date(year, 11, 12)?)
            }
            _ if (((2023..=2026).collect::<Vec<_>>()).contains(&(year))) => {
                Some(super::date(year, 9, 30)?)
            }
            _ => None,
        };
        if (king_birthday_second_day).is_some() {
            holidays.push(Holiday {
                name: "Hari Keputeraan Sultan Kelantan Hari Kedua".to_owned(),
                date: (king_birthday_second_day)
                    .ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("ktn".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_kuala_lumpur(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tahun Baru".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("kul".to_owned()),
        });
        let hari_thaipusam = self.hari_thaipusam(year)?;
        if (hari_thaipusam).is_some() {
            holidays.push(Holiday {
                name: "Hari Thaipusam".to_owned(),
                date: (hari_thaipusam).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("kul".to_owned()),
            });
        }
        holidays.push(Holiday {
            name: "Hari Wilayah Persekutuan".to_owned(),
            date: self.hari_wilayah(year)?,
            holiday_type: HolidayType::Regional,
            region: Some("kul".to_owned()),
        });
        let nurul_al_quran = islamic_to_gregorian_date("09-17", year, false)?;
        holidays.push(Holiday {
            name: "Hari Nurul Al-Quran".to_owned(),
            date: nurul_al_quran,
            holiday_type: HolidayType::Regional,
            region: Some("kul".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("kul".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_labuan(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tahun Baru".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("lbn".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Wilayah Persekutuan".to_owned(),
            date: self.hari_wilayah(year)?,
            holiday_type: HolidayType::Regional,
            region: Some("lbn".to_owned()),
        });
        let pesta_kaamatan = self.pesta_kaamatan(year)?;
        holidays.push(Holiday {
            name: "Pesta Kaamatan".to_owned(),
            date: pesta_kaamatan,
            holiday_type: HolidayType::Regional,
            region: Some("lbn".to_owned()),
        });
        holidays.push(Holiday {
            name: "Pesta Kaamatan Hari Kedua".to_owned(),
            date: (pesta_kaamatan) + Duration::days(i64::from(1)),
            holiday_type: HolidayType::Regional,
            region: Some("lbn".to_owned()),
        });
        let nurul_al_quran = islamic_to_gregorian_date("09-17", year, false)?;
        holidays.push(Holiday {
            name: "Hari Nurul Al-Quran".to_owned(),
            date: nurul_al_quran,
            holiday_type: HolidayType::Regional,
            region: Some("lbn".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("lbn".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_melaka(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tahun Baru".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("mlk".to_owned()),
        });
        let awal_ramadan = islamic_to_gregorian_date("09-01", year, false)?;
        holidays.push(Holiday {
            name: "Awal Ramadan".to_owned(),
            date: awal_ramadan,
            holiday_type: HolidayType::Regional,
            region: Some("mlk".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Perisytiharan Melaka Sebagai Bandaraya Bersejarah".to_owned(),
            date: parse_date(&(format!("{}-04-15", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("mlk".to_owned()),
        });
        holidays.push(Holiday {
            name: "Harijadi Yang di-Pertua Negeri Melaka".to_owned(),
            date: parse_date(&(format!("{}-08-24", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("mlk".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("mlk".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_negeri9(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tahun Baru".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("nsn".to_owned()),
        });
        let hari_thaipusam = self.hari_thaipusam(year)?;
        if (hari_thaipusam).is_some() {
            holidays.push(Holiday {
                name: "Hari Thaipusam".to_owned(),
                date: (hari_thaipusam).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("nsn".to_owned()),
            });
        }
        let israk_mikraj = islamic_to_gregorian_date("07-27", year, false)?;
        holidays.push(Holiday {
            name: "Israk dan Mikraj".to_owned(),
            date: israk_mikraj,
            holiday_type: HolidayType::Regional,
            region: Some("nsn".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Keputeraan Yang Di-Pertuan Besar Negeri Sembilan".to_owned(),
            date: parse_date(&(format!("{}-01-14", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("nsn".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("nsn".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_pahang(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tahun Baru".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("phg".to_owned()),
        });
        let nurul_al_quran = islamic_to_gregorian_date("09-17", year, false)?;
        holidays.push(Holiday {
            name: "Hari Nurul Al-Quran".to_owned(),
            date: nurul_al_quran,
            holiday_type: HolidayType::Regional,
            region: Some("phg".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Hol Pahang".to_owned(),
            date: parse_date(&(format!("{}-05-22", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("phg".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Keputeraan Sultan Pahang".to_owned(),
            date: parse_date(&(format!("{}-07-30", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("phg".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("phg".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_penang(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tahun Baru".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("png".to_owned()),
        });
        let hari_thaipusam = self.hari_thaipusam(year)?;
        if (hari_thaipusam).is_some() {
            holidays.push(Holiday {
                name: "Hari Thaipusam".to_owned(),
                date: (hari_thaipusam).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("png".to_owned()),
            });
        }
        let nurul_al_quran = islamic_to_gregorian_date("09-17", year, false)?;
        holidays.push(Holiday {
            name: "Hari Nurul Al-Quran".to_owned(),
            date: nurul_al_quran,
            holiday_type: HolidayType::Regional,
            region: Some("png".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Bandar Warisan Dunia Georgetown".to_owned(),
            date: parse_date(&(format!("{}-07-07", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("png".to_owned()),
        });
        holidays.push(Holiday {
            name: "Harijadi Yang di-Pertua Negeri Pulau Pinang".to_owned(),
            date: parse_date(&(format!("second saturday of july {}", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("png".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("png".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_perak(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tahun Baru".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("prk".to_owned()),
        });
        let hari_thaipusam = self.hari_thaipusam(year)?;
        if (hari_thaipusam).is_some() {
            holidays.push(Holiday {
                name: "Hari Thaipusam".to_owned(),
                date: (hari_thaipusam).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("prk".to_owned()),
            });
        }
        let nurul_al_quran = islamic_to_gregorian_date("09-17", year, false)?;
        holidays.push(Holiday {
            name: "Hari Nurul Al-Quran".to_owned(),
            date: nurul_al_quran,
            holiday_type: HolidayType::Regional,
            region: Some("prk".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Keputeraan Sultan Perak".to_owned(),
            date: parse_date(&(format!("first friday of november {}", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("prk".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("prk".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_perlis(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        let israk_mikraj = islamic_to_gregorian_date("07-27", year, false)?;
        holidays.push(Holiday {
            name: "Israk dan Mikraj".to_owned(),
            date: israk_mikraj,
            holiday_type: HolidayType::Regional,
            region: Some("pls".to_owned()),
        });
        let nurul_al_quran = islamic_to_gregorian_date("09-17", year, false)?;
        holidays.push(Holiday {
            name: "Hari Nurul Al-Quran".to_owned(),
            date: nurul_al_quran,
            holiday_type: HolidayType::Regional,
            region: Some("pls".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Keputeraan Raja Perlis".to_owned(),
            date: parse_date(&(format!("{}-05-17", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("pls".to_owned()),
        });
        let raya_aidiladha2 = islamic_to_gregorian_date("12-11", year, false)?;
        holidays.push(Holiday {
            name: "Hari Raya Aidiladha Hari Kedua".to_owned(),
            date: raya_aidiladha2,
            holiday_type: HolidayType::Regional,
            region: Some("pls".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("pls".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_putrajaya(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tahun Baru".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("pjy".to_owned()),
        });
        let hari_thaipusam = self.hari_thaipusam(year)?;
        if (hari_thaipusam).is_some() {
            holidays.push(Holiday {
                name: "Hari Thaipusam".to_owned(),
                date: (hari_thaipusam).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("pjy".to_owned()),
            });
        }
        holidays.push(Holiday {
            name: "Hari Wilayah Persekutuan".to_owned(),
            date: self.hari_wilayah(year)?,
            holiday_type: HolidayType::Regional,
            region: Some("pjy".to_owned()),
        });
        let nurul_al_quran = islamic_to_gregorian_date("09-17", year, false)?;
        holidays.push(Holiday {
            name: "Hari Nurul Al-Quran".to_owned(),
            date: nurul_al_quran,
            holiday_type: HolidayType::Regional,
            region: Some("pjy".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("pjy".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn region_sabah(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tahun Baru".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("sbh".to_owned()),
        });
        let pesta_kaamatan = self.pesta_kaamatan(year)?;
        holidays.push(Holiday {
            name: "Pesta Kaamatan".to_owned(),
            date: pesta_kaamatan,
            holiday_type: HolidayType::Regional,
            region: Some("sbh".to_owned()),
        });
        holidays.push(Holiday {
            name: "Pesta Kaamatan Hari Kedua".to_owned(),
            date: (pesta_kaamatan) + Duration::days(i64::from(1)),
            holiday_type: HolidayType::Regional,
            region: Some("sbh".to_owned()),
        });
        let good_friday = self.good_friday(year)?;
        holidays.push(Holiday {
            name: "Good Friday".to_owned(),
            date: good_friday,
            holiday_type: HolidayType::Regional,
            region: Some("sbh".to_owned()),
        });
        holidays.push(Holiday {
            name: "Harijadi Yang di-Pertua Negeri Sabah".to_owned(),
            date: parse_date(&(format!("first saturday of october {}", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("sbh".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("sbh".to_owned()),
            });
        }
        holidays.push(Holiday {
            name: "Hari Natal".to_owned(),
            date: parse_date(&(format!("{}-12-24", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("sbh".to_owned()),
        });
        Ok(holidays.clone())
    }
    pub(super) fn region_sarawak(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tahun Baru".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("swk".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Sarawak".to_owned(),
            date: parse_date(&(format!("{}-07-22", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("swk".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Gawai".to_owned(),
            date: parse_date(&(format!("{}-06-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("swk".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Gawai Kedua".to_owned(),
            date: parse_date(&(format!("{}-06-02", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("swk".to_owned()),
        });
        holidays.push(Holiday {
            name: "Harijadi Yang di-Pertua Negeri Sarawak".to_owned(),
            date: parse_date(&(format!("second saturday of october {}", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("swk".to_owned()),
        });
        let good_friday = self.good_friday(year)?;
        holidays.push(Holiday {
            name: "Good Friday".to_owned(),
            date: good_friday,
            holiday_type: HolidayType::Regional,
            region: Some("swk".to_owned()),
        });
        Ok(holidays.clone())
    }
    pub(super) fn region_selangor(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        holidays.push(Holiday {
            name: "Tahun Baru".to_owned(),
            date: parse_date(&(format!("{}-01-01", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("sgr".to_owned()),
        });
        let hari_thaipusam = self.hari_thaipusam(year)?;
        if (hari_thaipusam).is_some() {
            holidays.push(Holiday {
                name: "Hari Thaipusam".to_owned(),
                date: (hari_thaipusam).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("sgr".to_owned()),
            });
        }
        let nurul_al_quran = islamic_to_gregorian_date("09-17", year, false)?;
        holidays.push(Holiday {
            name: "Hari Nurul Al-Quran".to_owned(),
            date: nurul_al_quran,
            holiday_type: HolidayType::Regional,
            region: Some("sgr".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("sgr".to_owned()),
            });
        }
        holidays.push(Holiday {
            name: "Hari Keputeraan Sultan Selangor".to_owned(),
            date: parse_date(&(format!("{}-12-11", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("sgr".to_owned()),
        });
        Ok(holidays.clone())
    }
    pub(super) fn region_terengganu(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = vec![];
        let israk_mikraj = islamic_to_gregorian_date("07-27", year, false)?;
        holidays.push(Holiday {
            name: "Israk dan Mikraj".to_owned(),
            date: israk_mikraj,
            holiday_type: HolidayType::Regional,
            region: Some("trg".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Ulang Tahun Pertabalan Sultan Terengganu".to_owned(),
            date: parse_date(&(format!("{}-03-04", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("trg".to_owned()),
        });
        let nurul_al_quran = islamic_to_gregorian_date("09-17", year, false)?;
        holidays.push(Holiday {
            name: "Hari Nurul Al-Quran".to_owned(),
            date: nurul_al_quran,
            holiday_type: HolidayType::Regional,
            region: Some("trg".to_owned()),
        });
        holidays.push(Holiday {
            name: "Hari Keputeraan Sultan Terengganu".to_owned(),
            date: parse_date(&(format!("{}-04-26", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some("trg".to_owned()),
        });
        let hari_arafah = islamic_to_gregorian_date("12-09", year, false)?;
        holidays.push(Holiday {
            name: "Hari Arafah".to_owned(),
            date: hari_arafah,
            holiday_type: HolidayType::Regional,
            region: Some("trg".to_owned()),
        });
        let raya_aidiladha2 = islamic_to_gregorian_date("12-11", year, false)?;
        holidays.push(Holiday {
            name: "Hari Raya Aidiladha Hari Kedua".to_owned(),
            date: raya_aidiladha2,
            holiday_type: HolidayType::Regional,
            region: Some("trg".to_owned()),
        });
        let hari_deepavali = self.hari_deepavali(year)?;
        if (hari_deepavali).is_some() {
            holidays.push(Holiday {
                name: "Hari Deepavali".to_owned(),
                date: (hari_deepavali).ok_or_else(|| Error::Calculation("Malaysia".into()))?,
                holiday_type: HolidayType::Regional,
                region: Some("trg".to_owned()),
            });
        }
        Ok(holidays.clone())
    }
    pub(super) fn hari_wilayah(&self, year: i32) -> Result<Date, Error> {
        super::date(year, 2, 1)
    }
    pub(super) fn pesta_kaamatan(&self, year: i32) -> Result<Date, Error> {
        super::date(year, 5, 30)
    }
    pub(super) fn good_friday(&self, year: i32) -> Result<Date, Error> {
        let easter = super::easter(year, false)?;
        Ok((easter) + Duration::days(i64::from(-(2))))
    }
    pub(super) fn hari_keputeraan_ydp(&self, year: i32) -> Result<Option<Date>, Error> {
        Ok(match () {
            _ if (((2001..=2026).collect::<Vec<_>>()).contains(&(year))) => Some(parse_date(
                &(format!("first saturday of october {}", year)),
            )?),
            _ => None,
        })
    }
    pub(super) fn hari_wesak(&self, year: i32) -> Result<Option<Date>, Error> {
        let date = (IndexMap::from([
            (2020, vec!["05-07".to_owned()]),
            (2021, vec!["05-26".to_owned()]),
            (2022, vec!["05-15".to_owned()]),
            (2023, vec!["05-04".to_owned()]),
            (2024, vec!["05-22".to_owned()]),
            (2025, vec!["05-12".to_owned()]),
            (2026, vec!["05-31".to_owned()]),
        ]))
        .get(&(year))
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_default();
        Ok(if !(date.clone()).is_empty() {
            Some(parse_date(&(format!("{}-{}", year, date.clone())))?)
        } else {
            None
        })
    }
    pub(super) fn hari_thaipusam(&self, year: i32) -> Result<Option<Date>, Error> {
        let date = (IndexMap::from([
            (2020, vec!["02-08".to_owned()]),
            (2021, vec!["01-28".to_owned()]),
            (2022, vec!["01-18".to_owned()]),
            (2023, vec!["02-04".to_owned()]),
            (2024, vec!["01-25".to_owned()]),
            (2025, vec!["02-11".to_owned()]),
            (2026, vec!["02-01".to_owned()]),
        ]))
        .get(&(year))
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_default();
        Ok(if !(date.clone()).is_empty() {
            Some(parse_date(&(format!("{}-{}", year, date.clone())))?)
        } else {
            None
        })
    }
    pub(super) fn hari_deepavali(&self, year: i32) -> Result<Option<Date>, Error> {
        let date = (IndexMap::from([
            (2020, vec!["11-14".to_owned()]),
            (2021, vec!["11-04".to_owned()]),
            (2022, vec!["10-24".to_owned()]),
            (2023, vec!["11-13".to_owned()]),
            (2024, vec!["11-01".to_owned()]),
            (2025, vec!["10-20".to_owned()]),
            (2026, vec!["11-08".to_owned()]),
        ]))
        .get(&(year))
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_default();
        Ok(if !(date.clone()).is_empty() {
            Some(parse_date(&(format!("{}-{}", year, date.clone())))?)
        } else {
            None
        })
    }
}
