use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = Vec::new();
        for (name, md) in [
            ("一月一日", "01-01"),
            ("勞動節", "05-01"),
            ("香港特別行政區成立紀念日", "07-01"),
            ("國慶日", "10-01"),
            ("聖誕節", "12-25"),
            ("聖誕節後第一個周日", "12-26"),
        ] {
            holidays.push(Holiday {
                name: name.into(),
                date: parse_date(&format!("{year}-{md}"))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        let since = f64::from(year - 2000);
        let qingming = (since * 0.2422 + 4.81).floor() as i32 - (since / 4.0).floor() as i32;
        holidays.insert(
            1,
            Holiday {
                name: "清明節".into(),
                date: super::date(year, 4, qingming)?,
                holiday_type: HolidayType::National,
                region: None,
            },
        );
        for (name, md) in [
            ("農曆年初一", "01-01"),
            ("農曆年初二", "01-02"),
            ("農曆年初三", "01-03"),
            ("佛誕", "04-08"),
            ("端午節", "05-05"),
            ("中秋節翌日", "08-16"),
            ("重陽節", "09-09"),
        ] {
            holidays.push(Holiday {
                name: name.into(),
                date: chinese_to_gregorian_date(md, year)?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        for (name, offset) in [
            ("耶穌受難節", -2),
            ("耶穌受難節翌日", -1),
            ("復活節星期一", 1),
        ] {
            holidays.push(Holiday {
                name: name.into(),
                date: super::easter(year, false)? + Duration::days(offset),
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        holidays.sort_by_key(|h| h.date);
        let mut occupied: Vec<_> = holidays.iter().map(|h| h.date).collect();
        for holiday in &mut holidays {
            if !date_is(holiday.date, "sunday") {
                continue;
            }
            let mut shifted = holiday.date + Duration::days(1);
            while occupied.contains(&shifted) {
                shifted += Duration::days(1);
            }
            occupied.push(shifted);
            holiday.name = match holiday.name.as_str() {
                "農曆年初一" | "農曆年初二" | "農曆年初三" => {
                    let original = match holiday.name.as_str() {
                        "農曆年初一" => 1,
                        "農曆年初二" => 2,
                        _ => 3,
                    };
                    let landed = original + (shifted - holiday.date).num_days();
                    let numeral = match landed {
                        1 => "一",
                        2 => "二",
                        3 => "三",
                        4 => "四",
                        _ => {
                            return Err(Error::Calculation(
                                "Lunar New Year substitute name".into(),
                            ));
                        }
                    };
                    format!("農曆年初{numeral}")
                }
                "聖誕節" => "聖誕節後第二個周日".into(),
                "聖誕節後第一個周日" => holiday.name.clone(),
                _ if holiday.name.contains("翌日") => holiday.name.clone(),
                _ => format!("{}翌日", holiday.name),
            };
            holiday.date = shifted;
            holiday.holiday_type = HolidayType::Observed;
        }
        Ok(holidays)
    }
}
