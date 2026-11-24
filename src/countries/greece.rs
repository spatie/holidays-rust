use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Πρωτοχρονιά".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Θεοφάνια".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "25η Μαρτίου".to_owned(),
                    date: parse_date(&(format!("{}-03-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Δεκαπενταύγουστος".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "28η Οκτωβρίου".to_owned(),
                    date: parse_date(&(format!("{}-10-28", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Χριστούγεννα".to_owned(),
                    date: parse_date(&(format!("{}-12-25", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Σύναξη της Θεοτόκου".to_owned(),
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
        let orthodox_easter = super::easter(year, true)?;
        let megali_tetarti = (orthodox_easter) + Duration::days(i64::from(-(4)));
        let megali_pempti = (orthodox_easter) + Duration::days(i64::from(-(3)));
        let megali_paraskevi = (orthodox_easter) + Duration::days(i64::from(-(2)));
        let megalo_savvato = (orthodox_easter) + Duration::days(i64::from(-(1)));
        let deutera_pasha = (orthodox_easter) + Duration::days(i64::from(1));
        let mut protomagia = super::date(year, 5, 1)?;
        let move_protomagia = vec![
            megali_tetarti,
            megali_pempti,
            megali_paraskevi,
            megalo_savvato,
            orthodox_easter,
            deutera_pasha,
        ];
        if (move_protomagia.clone()).contains(&(protomagia)) {
            protomagia = (orthodox_easter) + Duration::days(i64::from(2));
        }
        if date_is(protomagia, "sunday") {
            protomagia += Duration::days(i64::from(1));
        }
        Ok(vec![
            Holiday {
                name: "Καθαρά Δευτέρα".to_owned(),
                date: (orthodox_easter) + Duration::days(i64::from(-(48))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Πρωτομαγιά".to_owned(),
                date: protomagia,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Μεγάλη Παρασκευή".to_owned(),
                date: megali_paraskevi,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Κυριακή του Πάσχα".to_owned(),
                date: orthodox_easter,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Δευτέρα του Πάσχα".to_owned(),
                date: deutera_pasha,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Αγίου Πνεύματος".to_owned(),
                date: (orthodox_easter) + Duration::days(i64::from(50)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
