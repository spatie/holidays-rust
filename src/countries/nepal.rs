use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let mut holidays = Vec::new();
        for (supported, md, name, holiday_type) in [
            (2024, "04-13", "Nepali New Year", HolidayType::Religious),
            (
                2024,
                "05-28",
                "National Republic Day",
                HolidayType::National,
            ),
            (2025, "01-11", "Prithivi Jayanti", HolidayType::National),
            (2025, "01-29", "Martyrs' Day", HolidayType::National),
            (2025, "02-19", "Democracy Day", HolidayType::National),
            (2024, "09-19", "Constitution Day", HolidayType::National),
            (2025, "01-14", "Makar Sankranti", HolidayType::Religious),
            (
                2024,
                "10-03",
                "Ghatasthapana (Dashain)",
                HolidayType::Religious,
            ),
            (2024, "10-11", "Fulpati (Dashain)", HolidayType::Religious),
            (
                2024,
                "10-12",
                "Maha Nawami (Dashain)",
                HolidayType::Religious,
            ),
            (
                2024,
                "10-12",
                "Maha Asthami (Dashain)",
                HolidayType::Religious,
            ),
            (
                2024,
                "10-13",
                "Vijaya Dashami (Dashain)",
                HolidayType::Religious,
            ),
            (2024, "10-14", "Ekadashi (Dashain)", HolidayType::Religious),
            (2024, "11-01", "Laxmi Puja (Tihar)", HolidayType::Religious),
            (
                2024,
                "11-02",
                "Govardhan Puja / Mha Puja (Tihar)",
                HolidayType::Religious,
            ),
            (2024, "11-03", "Bhai Tika (Tihar)", HolidayType::Religious),
            (2024, "11-07", "Chhat Parva", HolidayType::Religious),
            (2024, "12-30", "Tamu Loshar", HolidayType::Religious),
            (2025, "01-30", "Sonam Losar", HolidayType::Religious),
            (2025, "02-28", "Gyalpo Loshar", HolidayType::Religious),
            (2024, "12-15", "Udhauli Parva", HolidayType::Religious),
            (2025, "02-26", "Maha Shivaratri", HolidayType::Religious),
            (2025, "03-13", "Holi / Fagu Purnima", HolidayType::Religious),
        ] {
            if supported == year {
                holidays.push(Holiday {
                    name: name.into(),
                    date: parse_date(&format!("{year}-{md}"))?,
                    holiday_type,
                    region: None,
                });
            }
        }
        for (md, name) in [("05-01", "International Labor Day"), ("12-25", "Christmas")] {
            holidays.push(Holiday {
                name: name.into(),
                date: parse_date(&format!("{year}-{md}"))?,
                holiday_type: HolidayType::National,
                region: None,
            });
        }
        Ok(holidays)
    }
}
