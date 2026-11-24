use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Año Nuevo".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Epifanía del Señor".to_owned(),
                    date: parse_date(&(format!("{}-01-06", year)))?,
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
                    name: "Asunción de la Virgen".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Fiesta Nacional de España".to_owned(),
                    date: parse_date(&(format!("{}-10-12", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Todos los Santos".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Día de la Constitución Española".to_owned(),
                    date: parse_date(&(format!("{}-12-06", year)))?,
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
            ],
            self.variable_holidays(year)?,
            self.region_holidays(year)?,
        ]
        .concat())
    }
    pub(super) fn variable_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        let easter = super::easter(year, false)?;
        Ok(vec![Holiday {
            name: "Viernes Santo".to_owned(),
            date: (easter) + Duration::days(i64::from(-(2))),
            holiday_type: HolidayType::National,
            region: None,
        }])
    }
    pub(super) fn region_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        if self._region.is_empty() {
            return Ok(Vec::new());
        }
        let dates = match year {
            2022 => self.region_holidays2022(year)?,
            2023 => self.region_holidays2023(year)?,
            2024 => self.region_holidays2024(year)?,
            2025 => self.region_holidays2025(year)?,
            2026 => self.region_holidays2026(year)?,
            2027 => self.region_holidays2027(year)?,
            _ => return Err(Error::UnsupportedYear),
        };
        Ok(dates
            .into_iter()
            .map(|(name, date)| Holiday {
                name,
                date,
                holiday_type: HolidayType::Regional,
                region: Some(self._region.into()),
            })
            .collect())
    }
    pub(super) fn region_holidays2022(&self, year: i32) -> Result<IndexMap<String, Date>, Error> {
        let jueves_santo = IndexMap::from([("Jueves Santo".to_owned(), super::date(year, 4, 14)?)]);
        let lunes_pascua =
            IndexMap::from([("Lunes de Pascua".to_owned(), super::date(year, 4, 18)?)]);
        let fiesta_trabajo = IndexMap::from([(
            "Lunes siguiente a la Fiesta del Trabajo".to_owned(),
            super::date(year, 5, 2)?,
        )]);
        let san_juan = IndexMap::from([("San Juan".to_owned(), super::date(year, 6, 24)?)]);
        let santiago_apostol =
            IndexMap::from([("Santiago Apóstol".to_owned(), super::date(year, 7, 25)?)]);
        let navidad = IndexMap::from([(
            "Lunes siguiente a Navidad".to_owned(),
            super::date(year, 12, 26)?,
        )]);
        Ok(match () {
            _ if self._region == "es-an" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de Andalucía".to_owned(),
                            super::date(year, 2, 28)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    fiesta_trabajo.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-ar" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Lunes siguiente a San Jorge, Día de Aragón".to_owned(),
                            super::date(year, 4, 23)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    fiesta_trabajo.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-as" => union(
                union(
                    union(
                        IndexMap::from([("Día de Asturias".to_owned(), super::date(year, 9, 8)?)]),
                        jueves_santo.clone(),
                    ),
                    fiesta_trabajo.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-cb" => union(
                union(
                    IndexMap::from([
                        (
                            "Día de las Instituciones de Cantabria".to_owned(),
                            super::date(year, 7, 28)?,
                        ),
                        ("La Bien Aparecida".to_owned(), super::date(year, 9, 15)?),
                    ]),
                    jueves_santo.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-ce" => union(
                IndexMap::from([
                    (
                        "Fiesta del Sacrificio Eid al-Adha".to_owned(),
                        super::date(year, 7, 9)?,
                    ),
                    (
                        "Nuestra Señora de África".to_owned(),
                        super::date(year, 8, 5)?,
                    ),
                    ("Día de Ceuta".to_owned(), super::date(year, 9, 2)?),
                ]),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-cl" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de Castilla y León".to_owned(),
                            super::date(year, 4, 23)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    fiesta_trabajo.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-cm" => union(
                union(
                    IndexMap::from([
                        (
                            "Día de Castilla-La Mancha".to_owned(),
                            super::date(year, 5, 31)?,
                        ),
                        ("Corpus Christi".to_owned(), super::date(year, 6, 16)?),
                    ]),
                    jueves_santo.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-cn" => union(
                union(
                    IndexMap::from([("Día de Canarias".to_owned(), super::date(year, 5, 30)?)]),
                    jueves_santo.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-ct" => union(
                union(
                    union(
                        IndexMap::from([("Pascua Granada".to_owned(), super::date(year, 6, 6)?)]),
                        lunes_pascua.clone(),
                    ),
                    san_juan.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-ex" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de Extremadura".to_owned(),
                            super::date(year, 9, 8)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    fiesta_trabajo.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-ga" => union(
                union(
                    IndexMap::from([
                        (
                            "Día de las Letras Gallegas".to_owned(),
                            super::date(year, 5, 17)?,
                        ),
                        (
                            "Santiago Apóstol / Día de Galicia".to_owned(),
                            super::date(year, 7, 25)?,
                        ),
                    ]),
                    jueves_santo.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == "es-ib" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de les Illes Balears".to_owned(),
                            super::date(year, 3, 1)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-mc" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de la Región de Murcia".to_owned(),
                            super::date(year, 6, 9)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    fiesta_trabajo.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-md" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Fiesta de la Comunidad de Madrid".to_owned(),
                            super::date(year, 5, 2)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    santiago_apostol.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-ml" => union(
                union(
                    IndexMap::from([
                        (
                            "Fiesta del Eid al-Fitr".to_owned(),
                            super::date(year, 5, 3)?,
                        ),
                        (
                            "Fiesta del Sacrificio Eid al-Adha".to_owned(),
                            super::date(year, 7, 11)?,
                        ),
                    ]),
                    jueves_santo.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-nc" => union(
                union(
                    union(jueves_santo.clone(), lunes_pascua.clone()),
                    santiago_apostol.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-pv" => union(
                union(
                    union(
                        IndexMap::from([(
                            "V Centenario de la Primera Vuelta al Mundo".to_owned(),
                            super::date(year, 9, 6)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-ri" => union(
                union(
                    union(
                        IndexMap::from([("Día de La Rioja".to_owned(), super::date(year, 6, 9)?)]),
                        jueves_santo.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                navidad.clone(),
            ),
            _ if self._region == "es-vc" => union(
                union(
                    union(
                        IndexMap::from([
                            (
                                "Día de la Comunidad Valenciana".to_owned(),
                                super::date(year, 10, 9)?,
                            ),
                            ("San José".to_owned(), super::date(year, 3, 19)?),
                        ]),
                        jueves_santo.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == (String::new()) => IndexMap::new(),
            _ => return Err(Error::UnsupportedYear),
        })
    }
    pub(super) fn region_holidays2023(&self, year: i32) -> Result<IndexMap<String, Date>, Error> {
        let ano_nuevo = IndexMap::from([(
            "Lunes siguiente a la Fiesta de Año Nuevo".to_owned(),
            super::date(year, 1, 2)?,
        )]);
        let jueve_santo = IndexMap::from([("Jueves Santo".to_owned(), super::date(year, 4, 6)?)]);
        let lunes_pascua =
            IndexMap::from([("Lunes de Pascua".to_owned(), super::date(year, 4, 10)?)]);
        let san_juan = IndexMap::from([("San Juan".to_owned(), super::date(year, 6, 24)?)]);
        let fiesta_sacrificio = IndexMap::from([(
            "Fiesta del Sacrificio Eid al-Adha".to_owned(),
            super::date(year, 6, 29)?,
        )]);
        let santiago_apostol =
            IndexMap::from([("Santiago Apóstol".to_owned(), super::date(year, 7, 25)?)]);
        Ok(match () {
            _ if self._region == "es-an" => union(
                union(
                    IndexMap::from([("Día de Andalucía".to_owned(), super::date(year, 2, 28)?)]),
                    ano_nuevo.clone(),
                ),
                jueve_santo.clone(),
            ),
            _ if self._region == "es-ar" => union(
                union(
                    IndexMap::from([(
                        "Lunes siguiente a San Jorge, Día de Aragón".to_owned(),
                        super::date(year, 4, 24)?,
                    )]),
                    ano_nuevo.clone(),
                ),
                jueve_santo.clone(),
            ),
            _ if self._region == "es-as" => union(
                union(
                    IndexMap::from([("Día de Asturias".to_owned(), super::date(year, 9, 8)?)]),
                    ano_nuevo.clone(),
                ),
                jueve_santo.clone(),
            ),
            _ if self._region == "es-cb" => union(
                IndexMap::from([
                    (
                        "Día de las Instituciones de Cantabria".to_owned(),
                        super::date(year, 7, 28)?,
                    ),
                    ("La Bien Aparecida".to_owned(), super::date(year, 9, 15)?),
                ]),
                jueve_santo.clone(),
            ),
            _ if self._region == "es-ce" => union(
                union(
                    IndexMap::from([
                        (
                            "Nuestra Señora de África".to_owned(),
                            super::date(year, 8, 5)?,
                        ),
                        ("Día de Ceuta".to_owned(), super::date(year, 9, 2)?),
                    ]),
                    jueve_santo.clone(),
                ),
                fiesta_sacrificio.clone(),
            ),
            _ if self._region == "es-cl" => union(
                union(ano_nuevo.clone(), jueve_santo.clone()),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-cm" => union(
                IndexMap::from([
                    (
                        "Día de Castilla-La Mancha".to_owned(),
                        super::date(year, 5, 31)?,
                    ),
                    ("Corpus Christi".to_owned(), super::date(year, 6, 8)?),
                ]),
                jueve_santo.clone(),
            ),
            _ if self._region == "es-cn" => union(
                IndexMap::from([("Día de Canarias".to_owned(), super::date(year, 5, 30)?)]),
                jueve_santo.clone(),
            ),
            _ if self._region == "es-ct" => union(
                union(
                    IndexMap::from([
                        (
                            "Fiesta Nacional de Cataluña".to_owned(),
                            super::date(year, 9, 11)?,
                        ),
                        ("San Esteban".to_owned(), super::date(year, 12, 26)?),
                    ]),
                    lunes_pascua.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == "es-ex" => union(
                IndexMap::from([
                    ("Carnaval".to_owned(), super::date(year, 2, 13)?),
                    ("Día de Extremadura".to_owned(), super::date(year, 9, 8)?),
                ]),
                jueve_santo.clone(),
            ),
            _ if self._region == "es-ga" => union(
                IndexMap::from([
                    (
                        "Día de las Letras Gallegas".to_owned(),
                        super::date(year, 5, 17)?,
                    ),
                    (
                        "Santiago Apóstol / Día de Galicia".to_owned(),
                        super::date(year, 7, 25)?,
                    ),
                ]),
                jueve_santo.clone(),
            ),
            _ if self._region == "es-ib" => union(
                IndexMap::from([(
                    "Día de les Illes Balears".to_owned(),
                    super::date(year, 3, 1)?,
                )]),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-mc" => union(
                union(
                    IndexMap::from([(
                        "Día de la Región de Murcia".to_owned(),
                        super::date(year, 6, 9)?,
                    )]),
                    ano_nuevo.clone(),
                ),
                jueve_santo.clone(),
            ),
            _ if self._region == "es-md" => union(
                IndexMap::from([
                    (
                        "Lunes siguiente A San José".to_owned(),
                        super::date(year, 3, 20)?,
                    ),
                    (
                        "Fiesta de la Comunidad de Madrid".to_owned(),
                        super::date(year, 5, 2)?,
                    ),
                ]),
                jueve_santo.clone(),
            ),
            _ if self._region == "es-ml" => union(
                union(
                    IndexMap::from([(
                        "Fiesta del Eid al-Fitr".to_owned(),
                        super::date(year, 4, 21)?,
                    )]),
                    jueve_santo.clone(),
                ),
                fiesta_sacrificio.clone(),
            ),
            _ if self._region == "es-nc" => union(
                union(jueve_santo.clone(), lunes_pascua.clone()),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-pv" => union(
                union(jueve_santo.clone(), lunes_pascua.clone()),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-ri" => union(
                union(
                    IndexMap::from([("Día de La Rioja".to_owned(), super::date(year, 6, 9)?)]),
                    jueve_santo.clone(),
                ),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-vc" => union(
                union(
                    IndexMap::from([(
                        "Día de la Comunidad Valenciana".to_owned(),
                        super::date(year, 10, 9)?,
                    )]),
                    lunes_pascua.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == (String::new()) => IndexMap::new(),
            _ => return Err(Error::UnsupportedYear),
        })
    }
    pub(super) fn region_holidays2024(&self, year: i32) -> Result<IndexMap<String, Date>, Error> {
        let san_jose = IndexMap::from([("San José".to_owned(), super::date(year, 3, 19)?)]);
        let jueves_santo = IndexMap::from([("Jueves Santo".to_owned(), super::date(year, 3, 28)?)]);
        let lunes_pascua =
            IndexMap::from([("Lunes de Pascua".to_owned(), super::date(year, 4, 1)?)]);
        let fiesta_sacrificio = IndexMap::from([(
            "Fiesta del Sacrificio Eid al-Adha".to_owned(),
            super::date(year, 6, 17)?,
        )]);
        let san_juan = IndexMap::from([("San Juan".to_owned(), super::date(year, 6, 24)?)]);
        let santiago_apostol =
            IndexMap::from([("Santiago Apóstol".to_owned(), super::date(year, 7, 25)?)]);
        let inmaculada_concepcion = IndexMap::from([(
            "Lunes siguiente a la Inmaculada Concepción".to_owned(),
            super::date(year, 12, 9)?,
        )]);
        Ok(match () {
            _ if self._region == "es-an" => union(
                union(
                    IndexMap::from([("Día de Andalucía".to_owned(), super::date(year, 2, 28)?)]),
                    jueves_santo.clone(),
                ),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-ar" => union(
                union(
                    IndexMap::from([(
                        "San Jorge / Día de Aragón".to_owned(),
                        super::date(year, 4, 23)?,
                    )]),
                    jueves_santo.clone(),
                ),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-as" => union(
                union(
                    IndexMap::from([(
                        "Lunes siguiente al Día de Asturias".to_owned(),
                        super::date(year, 9, 9)?,
                    )]),
                    jueves_santo.clone(),
                ),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-cb" => union(
                union(jueves_santo.clone(), lunes_pascua.clone()),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-ce" => union(
                union(
                    IndexMap::from([(
                        "Nuestra Señora de África".to_owned(),
                        super::date(year, 8, 5)?,
                    )]),
                    jueves_santo.clone(),
                ),
                fiesta_sacrificio.clone(),
            ),
            _ if self._region == "es-cl" => union(
                union(
                    IndexMap::from([(
                        "Fiesta de Castilla y León".to_owned(),
                        super::date(year, 4, 23)?,
                    )]),
                    jueves_santo.clone(),
                ),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-cm" => union(
                IndexMap::from([
                    ("Corpus Christi".to_owned(), super::date(year, 5, 30)?),
                    (
                        "Día de Castilla-La Mancha".to_owned(),
                        super::date(year, 5, 31)?,
                    ),
                ]),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-cn" => union(
                IndexMap::from([("Día de Canarias".to_owned(), super::date(year, 5, 30)?)]),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-ct" => union(
                union(
                    IndexMap::from([
                        (
                            "Fiesta Nacional de Cataluña".to_owned(),
                            super::date(year, 9, 11)?,
                        ),
                        ("San Esteban".to_owned(), super::date(year, 12, 26)?),
                    ]),
                    lunes_pascua.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == "es-ex" => union(
                union(
                    IndexMap::from([("Carnaval".to_owned(), super::date(year, 2, 13)?)]),
                    jueves_santo.clone(),
                ),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-ga" => union(
                IndexMap::from([
                    (
                        "Día de las Letras Gallegas".to_owned(),
                        super::date(year, 5, 17)?,
                    ),
                    (
                        "Santiago Apóstol / Día de Galicia".to_owned(),
                        super::date(year, 7, 25)?,
                    ),
                ]),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-ib" => union(
                union(
                    IndexMap::from([(
                        "Día de les Illes Balears".to_owned(),
                        super::date(year, 3, 1)?,
                    )]),
                    jueves_santo.clone(),
                ),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-mc" => union(
                union(san_jose.clone(), jueves_santo.clone()),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-md" => union(
                union(
                    IndexMap::from([(
                        "Fiesta de la Comunidad de Madrid".to_owned(),
                        super::date(year, 5, 2)?,
                    )]),
                    jueves_santo.clone(),
                ),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-ml" => union(
                union(jueves_santo.clone(), fiesta_sacrificio.clone()),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-nc" => union(
                union(jueves_santo.clone(), lunes_pascua.clone()),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-pv" => union(
                union(jueves_santo.clone(), lunes_pascua.clone()),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-ri" => union(
                union(
                    IndexMap::from([(
                        "Lunes siguiente al Día de La Rioja".to_owned(),
                        super::date(year, 6, 10)?,
                    )]),
                    jueves_santo.clone(),
                ),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-vc" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de la Comunidad Valenciana".to_owned(),
                            super::date(year, 10, 9)?,
                        )]),
                        san_jose.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == (String::new()) => IndexMap::new(),
            _ => return Err(Error::UnsupportedYear),
        })
    }
    pub(super) fn region_holidays2025(&self, year: i32) -> Result<IndexMap<String, Date>, Error> {
        let san_jose = IndexMap::from([("San José".to_owned(), super::date(year, 3, 19)?)]);
        let jueves_santo = IndexMap::from([("Jueves Santo".to_owned(), super::date(year, 4, 17)?)]);
        let lunes_pascua =
            IndexMap::from([("Lunes de Pascua".to_owned(), super::date(year, 4, 21)?)]);
        let fiesta_sacrificio = IndexMap::from([(
            "Fiesta del Sacrificio Eid al-Adha".to_owned(),
            super::date(year, 6, 7)?,
        )]);
        let san_juan = IndexMap::from([("San Juan".to_owned(), super::date(year, 6, 24)?)]);
        let santiago_apostol =
            IndexMap::from([("Santiago Apóstol".to_owned(), super::date(year, 7, 25)?)]);
        let inmaculada_concepcion = IndexMap::from([(
            "Lunes siguiente a la Inmaculada Concepción".to_owned(),
            super::date(year, 12, 8)?,
        )]);
        Ok(match () {
            _ if self._region == "es-an" => union(
                union(
                    IndexMap::from([("Día de Andalucía".to_owned(), super::date(year, 2, 28)?)]),
                    jueves_santo.clone(),
                ),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-ar" => union(
                union(
                    IndexMap::from([(
                        "San Jorge / Día de Aragón".to_owned(),
                        super::date(year, 4, 23)?,
                    )]),
                    jueves_santo.clone(),
                ),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-as" => union(
                union(
                    IndexMap::from([("Día de Asturias".to_owned(), super::date(year, 9, 8)?)]),
                    jueves_santo.clone(),
                ),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-cb" => union(
                union(jueves_santo.clone(), lunes_pascua.clone()),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-ce" => union(
                union(
                    IndexMap::from([(
                        "Nuestra Señora de África".to_owned(),
                        super::date(year, 8, 5)?,
                    )]),
                    jueves_santo.clone(),
                ),
                fiesta_sacrificio.clone(),
            ),
            _ if self._region == "es-cl" => union(
                union(
                    IndexMap::from([(
                        "Fiesta de Castilla y León".to_owned(),
                        super::date(year, 4, 23)?,
                    )]),
                    jueves_santo.clone(),
                ),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-cm" => union(
                IndexMap::from([
                    ("Corpus Christi".to_owned(), super::date(year, 6, 19)?),
                    (
                        "Día de Castilla-La Mancha".to_owned(),
                        super::date(year, 5, 31)?,
                    ),
                ]),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-cn" => union(
                IndexMap::from([("Día de Canarias".to_owned(), super::date(year, 5, 30)?)]),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-ct" => union(
                union(
                    IndexMap::from([
                        (
                            "Fiesta Nacional de Cataluña".to_owned(),
                            super::date(year, 9, 11)?,
                        ),
                        ("San Esteban".to_owned(), super::date(year, 12, 26)?),
                    ]),
                    lunes_pascua.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == "es-ex" => union(
                union(
                    IndexMap::from([("Día de Extremadura".to_owned(), super::date(year, 9, 8)?)]),
                    jueves_santo.clone(),
                ),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-ga" => union(
                IndexMap::from([
                    (
                        "Día de las Letras Gallegas".to_owned(),
                        super::date(year, 5, 17)?,
                    ),
                    (
                        "Santiago Apóstol / Día de Galicia".to_owned(),
                        super::date(year, 7, 25)?,
                    ),
                ]),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-ib" => union(
                union(
                    IndexMap::from([
                        (
                            "Día de les Illes Balears".to_owned(),
                            super::date(year, 3, 1)?,
                        ),
                        ("San Esteban".to_owned(), super::date(year, 12, 26)?),
                    ]),
                    jueves_santo.clone(),
                ),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-mc" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de la Región de Murcia".to_owned(),
                            super::date(year, 6, 9)?,
                        )]),
                        san_jose.clone(),
                    ),
                    jueves_santo.clone(),
                ),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-md" => union(
                union(
                    IndexMap::from([(
                        "Fiesta de la Comunidad de Madrid".to_owned(),
                        super::date(year, 5, 2)?,
                    )]),
                    jueves_santo.clone(),
                ),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-ml" => union(
                union(jueves_santo.clone(), fiesta_sacrificio.clone()),
                inmaculada_concepcion.clone(),
            ),
            _ if self._region == "es-nc" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de Navarra: San Francisco Javier".to_owned(),
                            super::date(year, 12, 3)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-pv" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día del País Vasco / Euskadiko Eguna".to_owned(),
                            super::date(year, 10, 25)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-ri" => union(
                union(
                    IndexMap::from([(
                        "Lunes siguiente al Día de La Rioja".to_owned(),
                        super::date(year, 6, 9)?,
                    )]),
                    jueves_santo.clone(),
                ),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-vc" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de la Comunidad Valenciana".to_owned(),
                            super::date(year, 10, 9)?,
                        )]),
                        san_jose.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == (String::new()) => IndexMap::new(),
            _ => return Err(Error::UnsupportedYear),
        })
    }
    pub(super) fn region_holidays2026(&self, year: i32) -> Result<IndexMap<String, Date>, Error> {
        let san_jose = IndexMap::from([("San José".to_owned(), super::date(year, 3, 19)?)]);
        let jueves_santo = IndexMap::from([("Jueves Santo".to_owned(), super::date(year, 4, 2)?)]);
        let lunes_pascua =
            IndexMap::from([("Lunes de Pascua".to_owned(), super::date(year, 4, 6)?)]);
        let eid_al_adha = IndexMap::from([(
            "Eid al-Adha (Fiesta del Sacrificio)".to_owned(),
            super::date(year, 5, 27)?,
        )]);
        let san_juan = IndexMap::from([("San Juan".to_owned(), super::date(year, 6, 24)?)]);
        let santiago_apostol =
            IndexMap::from([("Santiago Apóstol".to_owned(), super::date(year, 7, 25)?)]);
        let traslado_todos_los_santos = IndexMap::from([(
            "Traslado de Todos los Santos".to_owned(),
            super::date(year, 11, 2)?,
        )]);
        let traslado_constitucion = IndexMap::from([(
            "Traslado del Día de la Constitución".to_owned(),
            super::date(year, 12, 7)?,
        )]);
        Ok(match () {
            _ if self._region == "es-an" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de Andalucía".to_owned(),
                            super::date(year, 2, 28)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    traslado_todos_los_santos.clone(),
                ),
                traslado_constitucion.clone(),
            ),
            _ if self._region == "es-ar" => union(
                union(
                    union(
                        IndexMap::from([(
                            "San Jorge, Día de Aragón".to_owned(),
                            super::date(year, 4, 23)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    traslado_todos_los_santos.clone(),
                ),
                traslado_constitucion.clone(),
            ),
            _ if self._region == "es-as" => union(
                union(
                    union(
                        IndexMap::from([("Día de Asturias".to_owned(), super::date(year, 9, 8)?)]),
                        jueves_santo.clone(),
                    ),
                    traslado_todos_los_santos.clone(),
                ),
                traslado_constitucion.clone(),
            ),
            _ if self._region == "es-ib" => union(
                union(
                    IndexMap::from([
                        (
                            "Lunes siguiente Día de les Illes Balears".to_owned(),
                            super::date(year, 3, 2)?,
                        ),
                        (
                            "Segunda Fiesta de Navidad (Sant Esteve)".to_owned(),
                            super::date(year, 12, 26)?,
                        ),
                    ]),
                    jueves_santo.clone(),
                ),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-pv" => union(
                union(
                    union(san_jose.clone(), jueves_santo.clone()),
                    lunes_pascua.clone(),
                ),
                santiago_apostol.clone(),
            ),
            _ if self._region == "es-cn" => union(
                union(
                    IndexMap::from([("Día de Canarias".to_owned(), super::date(year, 5, 30)?)]),
                    jueves_santo.clone(),
                ),
                traslado_todos_los_santos.clone(),
            ),
            _ if self._region == "es-cb" => union(
                union(
                    IndexMap::from([
                        (
                            "Día de las Instituciones de Cantabria".to_owned(),
                            super::date(year, 7, 28)?,
                        ),
                        ("La Bien Aparecida".to_owned(), super::date(year, 9, 15)?),
                    ]),
                    jueves_santo.clone(),
                ),
                traslado_constitucion.clone(),
            ),
            _ if self._region == "es-cm" => union(
                union(
                    union(
                        IndexMap::from([("Corpus Christi".to_owned(), super::date(year, 6, 4)?)]),
                        jueves_santo.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                traslado_todos_los_santos.clone(),
            ),
            _ if self._region == "es-cl" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Fiesta de la Comunidad Autónoma".to_owned(),
                            super::date(year, 4, 23)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    traslado_todos_los_santos.clone(),
                ),
                traslado_constitucion.clone(),
            ),
            _ if self._region == "es-ct" => union(
                union(
                    IndexMap::from([
                        (
                            "Diada Nacional de Cataluña".to_owned(),
                            super::date(year, 9, 11)?,
                        ),
                        ("Sant Esteve".to_owned(), super::date(year, 12, 26)?),
                    ]),
                    lunes_pascua.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == "es-ex" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de Extremadura".to_owned(),
                            super::date(year, 9, 8)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    traslado_todos_los_santos.clone(),
                ),
                traslado_constitucion.clone(),
            ),
            _ if self._region == "es-ga" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día Nacional de Galicia (Santiago Apóstol)".to_owned(),
                            super::date(year, 7, 25)?,
                        )]),
                        san_jose.clone(),
                    ),
                    jueves_santo.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == "es-ri" => union(
                union(
                    union(
                        IndexMap::from([("Día de La Rioja".to_owned(), super::date(year, 6, 9)?)]),
                        jueves_santo.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                traslado_constitucion.clone(),
            ),
            _ if self._region == "es-md" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Fiesta de la Comunidad de Madrid".to_owned(),
                            super::date(year, 5, 2)?,
                        )]),
                        jueves_santo.clone(),
                    ),
                    traslado_todos_los_santos.clone(),
                ),
                traslado_constitucion.clone(),
            ),
            _ if self._region == "es-mc" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de la Región de Murcia".to_owned(),
                            super::date(year, 6, 9)?,
                        )]),
                        san_jose.clone(),
                    ),
                    jueves_santo.clone(),
                ),
                traslado_constitucion.clone(),
            ),
            _ if self._region == "es-nc" => union(
                union(
                    union(
                        union(
                            IndexMap::from([(
                                "San Francisco Javier".to_owned(),
                                super::date(year, 12, 3)?,
                            )]),
                            san_jose.clone(),
                        ),
                        jueves_santo.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                traslado_todos_los_santos.clone(),
            ),
            _ if self._region == "es-vc" => union(
                union(
                    union(
                        IndexMap::from([(
                            "Día de la Comunitat Valenciana".to_owned(),
                            super::date(year, 10, 9)?,
                        )]),
                        san_jose.clone(),
                    ),
                    lunes_pascua.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == "es-ce" => union(
                union(
                    IndexMap::from([
                        (
                            "Nuestra Señora de África".to_owned(),
                            super::date(year, 8, 5)?,
                        ),
                        ("Día de Ceuta".to_owned(), super::date(year, 9, 2)?),
                    ]),
                    jueves_santo.clone(),
                ),
                eid_al_adha.clone(),
            ),
            _ if self._region == "es-ml" => union(
                union(
                    union(
                        IndexMap::from([
                            ("Eid al-Fitr".to_owned(), super::date(year, 3, 20)?),
                            ("Virgen de la Victoria".to_owned(), super::date(year, 9, 8)?),
                            ("Día de Melilla".to_owned(), super::date(year, 9, 17)?),
                        ]),
                        jueves_santo.clone(),
                    ),
                    eid_al_adha.clone(),
                ),
                traslado_constitucion.clone(),
            ),
            _ if self._region == (String::new()) => IndexMap::new(),
            _ => return Err(Error::UnsupportedYear),
        })
    }
    pub(super) fn region_holidays2027(&self, year: i32) -> Result<IndexMap<String, Date>, Error> {
        let san_jose = IndexMap::from([("San José".to_owned(), super::date(year, 3, 19)?)]);
        let jueves_santo = IndexMap::from([("Jueves Santo".to_owned(), super::date(year, 3, 25)?)]);
        let lunes_pascua =
            IndexMap::from([("Lunes de Pascua".to_owned(), super::date(year, 3, 29)?)]);
        let eid_al_adha = IndexMap::from([(
            "Eid al-Adha (Fiesta del Sacrificio)".to_owned(),
            super::date(year, 5, 17)?,
        )]);
        let san_juan = IndexMap::from([("San Juan".to_owned(), super::date(year, 6, 24)?)]);
        let lunes_asuncion = IndexMap::from([(
            "Lunes siguiente a la Asunción de la Virgen".to_owned(),
            super::date(year, 8, 16)?,
        )]);
        Ok(match () {
            _ if self._region == "es-an" => union(
                union(
                    IndexMap::from([("Día de Andalucía".to_owned(), super::date(year, 3, 1)?)]),
                    jueves_santo.clone(),
                ),
                lunes_asuncion.clone(),
            ),
            _ if self._region == "es-ar" => union(
                union(
                    IndexMap::from([(
                        "San Jorge / Día de Aragón".to_owned(),
                        super::date(year, 4, 23)?,
                    )]),
                    jueves_santo.clone(),
                ),
                lunes_asuncion.clone(),
            ),
            _ if self._region == "es-as" => union(
                union(
                    IndexMap::from([("Día de Asturias".to_owned(), super::date(year, 9, 8)?)]),
                    jueves_santo.clone(),
                ),
                lunes_asuncion.clone(),
            ),
            _ if self._region == "es-ib" => union(
                union(
                    IndexMap::from([(
                        "Día de les Illes Balears".to_owned(),
                        super::date(year, 3, 1)?,
                    )]),
                    jueves_santo.clone(),
                ),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-pv" => union(
                union(
                    IndexMap::from([(
                        "Aniversario del primer Gobierno de Euskadi".to_owned(),
                        super::date(year, 10, 7)?,
                    )]),
                    jueves_santo.clone(),
                ),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-cn" => union(jueves_santo.clone(), lunes_asuncion.clone()),
            _ if self._region == "es-cb" => union(
                IndexMap::from([
                    (
                        "Día de las Instituciones de Cantabria".to_owned(),
                        super::date(year, 7, 28)?,
                    ),
                    ("La Bien Aparecida".to_owned(), super::date(year, 9, 15)?),
                ]),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-cm" => union(
                IndexMap::from([
                    ("Corpus Christi".to_owned(), super::date(year, 5, 27)?),
                    (
                        "Día de Castilla-La Mancha".to_owned(),
                        super::date(year, 5, 31)?,
                    ),
                ]),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-cl" => union(
                union(
                    IndexMap::from([(
                        "Fiesta de Castilla y León".to_owned(),
                        super::date(year, 4, 23)?,
                    )]),
                    jueves_santo.clone(),
                ),
                lunes_asuncion.clone(),
            ),
            _ if self._region == "es-ct" => union(
                union(
                    IndexMap::from([(
                        "Fiesta Nacional de Cataluña".to_owned(),
                        super::date(year, 9, 11)?,
                    )]),
                    lunes_pascua.clone(),
                ),
                san_juan.clone(),
            ),
            _ if self._region == "es-ex" => union(
                IndexMap::from([
                    ("Día de Extremadura".to_owned(), super::date(year, 9, 8)?),
                    (
                        "Sustitución del 15 de agosto".to_owned(),
                        super::date(year, 10, 11)?,
                    ),
                ]),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-ga" => union(
                union(
                    IndexMap::from([(
                        "Día de las Letras Gallegas".to_owned(),
                        super::date(year, 5, 17)?,
                    )]),
                    san_jose.clone(),
                ),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-ri" => union(
                union(
                    IndexMap::from([("Día de La Rioja".to_owned(), super::date(year, 6, 9)?)]),
                    jueves_santo.clone(),
                ),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-md" => union(
                union(san_jose.clone(), jueves_santo.clone()),
                lunes_asuncion.clone(),
            ),
            _ if self._region == "es-mc" => union(
                union(
                    IndexMap::from([(
                        "Día de la Región de Murcia".to_owned(),
                        super::date(year, 6, 9)?,
                    )]),
                    san_jose.clone(),
                ),
                jueves_santo.clone(),
            ),
            _ if self._region == "es-nc" => union(
                union(san_jose.clone(), jueves_santo.clone()),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-vc" => union(
                union(
                    IndexMap::from([(
                        "Día de la Comunitat Valenciana".to_owned(),
                        super::date(year, 10, 9)?,
                    )]),
                    san_jose.clone(),
                ),
                lunes_pascua.clone(),
            ),
            _ if self._region == "es-ce" => union(
                union(
                    IndexMap::from([(
                        "Nuestra Señora de África".to_owned(),
                        super::date(year, 8, 5)?,
                    )]),
                    jueves_santo.clone(),
                ),
                eid_al_adha.clone(),
            ),
            _ if self._region == "es-ml" => union(
                union(
                    IndexMap::from([("Eid al-Fitr".to_owned(), super::date(year, 3, 10)?)]),
                    jueves_santo.clone(),
                ),
                eid_al_adha.clone(),
            ),
            _ if self._region == (String::new()) => IndexMap::new(),
            _ => return Err(Error::UnsupportedYear),
        })
    }
}
