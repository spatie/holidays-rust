use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        let mut holidays = vec![
            Holiday {
                name: "Año Nuevo".to_owned(),
                date: parse_date(&(format!("{}-01-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Viernes Santo".to_owned(),
                date: easter - Duration::days(2),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Sábado Santo".to_owned(),
                date: easter - Duration::days(1),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día del Trabajador".to_owned(),
                date: parse_date(&(format!("{}-05-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de las Glorias Navales".to_owned(),
                date: parse_date(&(format!("{}-05-21", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "San Pedro y San Pablo".to_owned(),
                date: self.monday_holiday(super::date(year, 6, 29)?)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de la Virgen del Carmen".to_owned(),
                date: parse_date(&(format!("{}-07-16", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Asunción de la Virgen".to_owned(),
                date: parse_date(&(format!("{}-08-15", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Independencia Nacional".to_owned(),
                date: parse_date(&(format!("{}-09-18", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de las Glorias del Ejército".to_owned(),
                date: parse_date(&(format!("{}-09-19", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Encuentro de Dos Mundos".to_owned(),
                date: self.monday_holiday(super::date(year, 10, 12)?)?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de las Iglesias Evangélicas".to_owned(),
                date: {
                    let day = super::date(year, 10, 31)?;
                    if year >= 2008 && date_is(day, "tuesday") {
                        day - Duration::days(4)
                    } else if year >= 2008 && date_is(day, "wednesday") {
                        day + Duration::days(2)
                    } else {
                        day
                    }
                },
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de Todos los Santos".to_owned(),
                date: parse_date(&(format!("{}-11-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Inmaculada Concepción".to_owned(),
                date: parse_date(&(format!("{}-12-08", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Navidad".to_owned(),
                date: parse_date(&(format!("{}-12-25", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ];
        if year >= 2021 {
            holidays.push(Holiday {
                name: "Día Nacional de los Pueblos Indígenas".to_owned(),
                date: if year == 2021 {
                    super::date(year, 6, 21)?
                } else {
                    self.winter_solstice(year)?
                },
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        let september_17 = super::date(year, 9, 17)?;
        let september_20 = super::date(year, 9, 20)?;
        let extra = if (year >= 2007 && date_is(september_17, "monday"))
            || (year >= 2017 && date_is(september_17, "friday"))
        {
            Some(september_17)
        } else if year >= 2007 && date_is(september_20, "friday") {
            Some(september_20)
        } else {
            None
        };
        if let Some(day) = extra {
            holidays.push(Holiday {
                name: "Feriado añadido de Fiestas Patrias".to_owned(),
                date: day,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        if year >= 2017 && date_is(super::date(year, 1, 1)?, "sunday") {
            holidays.push(Holiday {
                name: "Feriado añadido de Año Nuevo".to_owned(),
                date: super::date(year, 1, 2)?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays)
    }

    fn monday_holiday(&self, day: Date) -> Result<Date, Error> {
        if day.year() < 2000 {
            return Ok(day);
        }
        match day.weekday().num_days_from_monday() {
            1..=3 => date_previous(day, "monday"),
            4 => date_next(day, "monday"),
            _ => Ok(day),
        }
    }

    fn winter_solstice(&self, year: i32) -> Result<Date, Error> {
        if year > 2100 {
            return Err(Error::UnsupportedYear);
        }
        // Meeus, Astronomical Algorithms, chapter 27. See tools/calendar-sources.md.
        let y = f64::from(year - 2000) / 1000.0;
        let mean =
            2_451_716.567_67 + y * (365_241.626_03 + y * (0.00325 + y * (0.00888 - y * 0.00030)));
        let t = (mean - 2_451_545.0) / 36_525.0;
        let w = (35_999.373 * t - 2.47).to_radians();
        let correction = [
            (485.0, 324.96, 1934.136),
            (203.0, 337.23, 32964.467),
            (199.0, 342.08, 20.186),
            (182.0, 27.85, 445267.112),
            (156.0, 73.14, 45036.886),
            (136.0, 171.52, 22518.443),
            (77.0, 222.54, 65928.934),
            (74.0, 296.72, 3034.906),
            (70.0, 243.58, 9037.513),
            (58.0, 119.81, 33718.147),
            (52.0, 297.17, 150.678),
            (50.0, 21.02, 2281.226),
            (45.0, 247.54, 29929.562),
            (44.0, 325.15, 31555.956),
            (29.0, 60.93, 4443.417),
            (18.0, 155.12, 67555.328),
            (17.0, 288.79, 4562.452),
            (16.0, 198.04, 62894.029),
            (14.0, 199.76, 31436.921),
            (12.0, 95.39, 14577.848),
            (12.0, 287.11, 31931.756),
            (12.0, 320.81, 34777.259),
            (9.0, 227.73, 1222.114),
            (8.0, 15.45, 16859.074),
        ]
        .into_iter()
        .map(|(a, b, c)| a * (b + c * t).to_radians().cos())
        .sum::<f64>();
        let dynamical =
            mean + 0.00001 * correction / (1.0 + 0.0334 * w.cos() + 0.0007 * (2.0 * w).cos());
        // NASA's predicted TT-UT difference, in seconds. June uses decimal year.
        let decimal_year = f64::from(year) + 5.5 / 12.0;
        let delta_t = if decimal_year < 2050.0 {
            let t = decimal_year - 2000.0;
            62.92 + 0.32217 * t + 0.005589 * t * t
        } else {
            let u = (decimal_year - 1820.0) / 100.0;
            -20.0 + 32.0 * u * u - 0.5628 * (2150.0 - decimal_year)
        };
        // Julian days start at noon; Rata Die days and Chile's UTC-4 dates start at midnight.
        let local_day = dynamical - delta_t / 86_400.0 - 4.0 / 24.0 - 1_721_424.5;
        Date::from_num_days_from_ce_opt(local_day.floor() as i32).ok_or(Error::UnsupportedYear)
    }
}
