use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let holidays = IndexMap::from([
            (
                "Nový rok".to_owned(),
                ("01-01".to_owned(), (year) <= (2000)),
            ),
            (
                "Nový rok; Den obnovy samostatného českého státu".to_owned(),
                ("01-01".to_owned(), (year) >= (2001)),
            ),
            ("Svátek práce".to_owned(), ("05-01".to_owned(), true)),
            (
                "Výročí osvobození Československa Sovětskou armádou".to_owned(),
                ("05-09".to_owned(), (year) <= (1990)),
            ),
            (
                "Den osvobození od fašismu".to_owned(),
                (
                    if (year) == (1991) {
                        "05-09".to_owned()
                    } else {
                        "05-08".to_owned()
                    },
                    (1991..=2000).contains(&(year)),
                ),
            ),
            (
                "Den osvobození".to_owned(),
                ("05-08".to_owned(), (2001..=2003).contains(&(year))),
            ),
            (
                "Den vítězství".to_owned(),
                ("05-08".to_owned(), (year) >= (2004)),
            ),
            (
                "Den slovanských věrozvěstů Cyrila a Metoděje".to_owned(),
                ("07-05".to_owned(), (year) >= (1990)),
            ),
            (
                "Den upálení mistra Jana Husa".to_owned(),
                ("07-06".to_owned(), (year) >= (1990)),
            ),
            (
                "Den české státnosti".to_owned(),
                ("09-28".to_owned(), (year) >= (2000)),
            ),
            (
                "Vyhlášení samostatnosti ČSR; Schválení zákona o federaci".to_owned(),
                ("10-28".to_owned(), (year) <= (1971)),
            ),
            (
                "Den znárodnění".to_owned(),
                ("10-28".to_owned(), (year) == (1972)),
            ),
            (
                "Vyhlášení samostatnosti ČSR; Schválení zákona o federaci; Den znárodnění"
                    .to_owned(),
                ("10-28".to_owned(), (1973..=1974).contains(&(year))),
            ),
            (
                "Den vzniku samostatného československého státu".to_owned(),
                ("10-28".to_owned(), (year) >= (1988)),
            ),
            (
                "Den boje za svobodu a demokracii a Mezinárodní den studentstva".to_owned(),
                ("11-17".to_owned(), (year) >= (2000)),
            ),
            (
                "Štědrý den".to_owned(),
                ("12-24".to_owned(), (year) >= (1990)),
            ),
            ("1. svátek vánoční".to_owned(), ("12-25".to_owned(), true)),
            ("2. svátek vánoční".to_owned(), ("12-26".to_owned(), true)),
        ]);
        let mut filtered_holidays = vec![];
        for (name, holiday) in holidays.clone() {
            if (holiday.clone()).1 {
                filtered_holidays.push(Holiday {
                    name: name.clone(),
                    date: parse_date(&(format!("{}-{}", year, (holiday.clone()).0)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                });
            }
        }
        Ok([filtered_holidays.clone(), self.variable_holidays(year)?].concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        let variable_holidays = IndexMap::from([
            (
                "Velikonoční pondělí".to_owned(),
                ((easter) + Duration::days(i64::from(1)), true),
            ),
            (
                "Velký pátek".to_owned(),
                ((easter) + Duration::days(i64::from(-(2))), (year) >= (2016)),
            ),
        ]);
        let mut result = vec![];
        for (name, variable_holiday) in variable_holidays.clone() {
            if variable_holiday.1 {
                result.push(Holiday {
                    name: name.clone(),
                    date: variable_holiday.0,
                    holiday_type: HolidayType::National,
                    region: None,
                });
            }
        }
        Ok(result.clone())
    }
}
