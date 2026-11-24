use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "Año Nuevo".to_owned(),
                date: parse_date(&(format!("{}-01-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Viernes Santo".to_owned(),
                date: parse_date(&(format!("{}-03-29", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Sábado Santo".to_owned(),
                date: parse_date(&(format!("{}-03-30", year)))?,
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
                name: "Día Nacional de los Pueblos Indígenas".to_owned(),
                date: parse_date(&(format!("{}-06-20", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "San Pedro y San Pablo".to_owned(),
                date: parse_date(&(format!("{}-06-29", year)))?,
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
                name: "Feriado añadido de Fiestas Patrias".to_owned(),
                date: parse_date(&(format!("{}-09-20", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Encuentro de Dos Mundos".to_owned(),
                date: parse_date(&(format!("{}-10-12", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Día de las Iglesias Evangélicas".to_owned(),
                date: parse_date(&(format!("{}-10-31", year)))?,
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
        ])
    }
}
