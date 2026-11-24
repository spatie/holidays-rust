use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok([
            vec![
                Holiday {
                    name: "Dia de Ano Novo".to_owned(),
                    date: parse_date(&(format!("{}-01-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dia de Tiradentes".to_owned(),
                    date: parse_date(&(format!("{}-04-21", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dia do Trabalhador".to_owned(),
                    date: parse_date(&(format!("{}-05-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Independência do Brasil".to_owned(),
                    date: parse_date(&(format!("{}-09-07", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Nossa Senhora Aparecida".to_owned(),
                    date: parse_date(&(format!("{}-10-12", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Finados".to_owned(),
                    date: parse_date(&(format!("{}-11-02", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Proclamação da República".to_owned(),
                    date: parse_date(&(format!("{}-11-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dia Nacional de Zumbi e da Consciência Negra".to_owned(),
                    date: parse_date(&(format!("{}-11-20", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Natal".to_owned(),
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
        Ok(vec![
            Holiday {
                name: "Carnaval".to_owned(),
                date: (easter) + Duration::days(i64::from(-(47))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Sexta-feira Santa".to_owned(),
                date: (easter) + Duration::days(i64::from(-(2))),
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "Corpus Christi".to_owned(),
                date: (easter) + Duration::days(i64::from(60)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn region_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        if (self._region.to_owned()).is_empty() {
            return Ok(vec![]);
        }
        let easter = super::easter(year, false)?;
        if self._region == "BR-AC" {
            return Ok(vec![
                Holiday {
                    name: "Dia do Evangélico".to_owned(),
                    date: parse_date(&(format!("{}-01-23", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AC".to_owned()),
                },
                Holiday {
                    name: "Aniversário do Acre".to_owned(),
                    date: parse_date(&(format!("{}-06-15", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AC".to_owned()),
                },
                Holiday {
                    name: "Dia da Amazônia".to_owned(),
                    date: parse_date(&(format!("{}-09-05", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AC".to_owned()),
                },
                Holiday {
                    name: "Tratado de Petrópolis".to_owned(),
                    date: parse_date(&(format!("{}-11-17", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AC".to_owned()),
                },
            ]);
        }
        if self._region == "BR-AL" {
            return Ok(vec![
                Holiday {
                    name: "São João".to_owned(),
                    date: parse_date(&(format!("{}-06-24", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AL".to_owned()),
                },
                Holiday {
                    name: "São Pedro".to_owned(),
                    date: parse_date(&(format!("{}-06-29", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AL".to_owned()),
                },
                Holiday {
                    name: "Emancipação Política de Alagoas".to_owned(),
                    date: parse_date(&(format!("{}-09-16", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AL".to_owned()),
                },
            ]);
        }
        if self._region == "BR-AP" {
            return Ok(vec![
                Holiday {
                    name: "São José".to_owned(),
                    date: parse_date(&(format!("{}-03-19", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AP".to_owned()),
                },
                Holiday {
                    name: "São Tiago".to_owned(),
                    date: parse_date(&(format!("{}-07-25", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AP".to_owned()),
                },
                Holiday {
                    name: "Criação do Estado do Amapá".to_owned(),
                    date: parse_date(&(format!("{}-10-05", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AP".to_owned()),
                },
            ]);
        }
        if self._region == "BR-AM" {
            return Ok(vec![
                Holiday {
                    name: "Elevação do Amazonas à categoria de Província".to_owned(),
                    date: parse_date(&(format!("{}-09-05", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AM".to_owned()),
                },
                Holiday {
                    name: "Nossa Senhora da Conceição".to_owned(),
                    date: parse_date(&(format!("{}-12-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-AM".to_owned()),
                },
            ]);
        }
        if self._region == "BR-BA" {
            return Ok(vec![Holiday {
                name: "Independência da Bahia".to_owned(),
                date: parse_date(&(format!("{}-07-02", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some("BR-BA".to_owned()),
            }]);
        }
        if self._region == "BR-CE" {
            return Ok(vec![
                Holiday {
                    name: "São José".to_owned(),
                    date: parse_date(&(format!("{}-03-19", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-CE".to_owned()),
                },
                Holiday {
                    name: "Abolição da Escravidão no Ceará".to_owned(),
                    date: parse_date(&(format!("{}-03-25", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-CE".to_owned()),
                },
            ]);
        }
        if self._region == "BR-DF" {
            return Ok(vec![Holiday {
                name: "Dia do Evangélico".to_owned(),
                date: parse_date(&(format!("{}-11-30", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some("BR-DF".to_owned()),
            }]);
        }
        if self._region == "BR-ES" {
            return Ok(vec![Holiday {
                name: "Nossa Senhora da Penha".to_owned(),
                date: (easter) + Duration::days(i64::from(8)),
                holiday_type: HolidayType::Regional,
                region: Some("BR-ES".to_owned()),
            }]);
        }
        if self._region == "BR-GO" {
            return Ok(vec![
                Holiday {
                    name: "Nossa Senhora Auxiliadora".to_owned(),
                    date: parse_date(&(format!("{}-05-24", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-GO".to_owned()),
                },
                Holiday {
                    name: "Nossa Senhora de Sant'Ana".to_owned(),
                    date: parse_date(&(format!("{}-07-26", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-GO".to_owned()),
                },
                Holiday {
                    name: "Pedra Fundamental de Goiânia".to_owned(),
                    date: parse_date(&(format!("{}-10-24", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-GO".to_owned()),
                },
            ]);
        }
        if self._region == "BR-MA" {
            return Ok(vec![
                Holiday {
                    name: "Adesão do Maranhão à Independência do Brasil".to_owned(),
                    date: parse_date(&(format!("{}-07-28", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-MA".to_owned()),
                },
                Holiday {
                    name: "Nossa Senhora da Conceição".to_owned(),
                    date: parse_date(&(format!("{}-12-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-MA".to_owned()),
                },
            ]);
        }
        if self._region == "BR-MS" {
            return Ok(vec![Holiday {
                name: "Criação do Estado de Mato Grosso do Sul".to_owned(),
                date: parse_date(&(format!("{}-10-11", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some("BR-MS".to_owned()),
            }]);
        }
        if self._region == "BR-PA" {
            return Ok(vec![
                Holiday {
                    name: "Adesão do Grão-Pará à Independência do Brasil".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-PA".to_owned()),
                },
                Holiday {
                    name: "Nossa Senhora da Conceição".to_owned(),
                    date: parse_date(&(format!("{}-12-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-PA".to_owned()),
                },
            ]);
        }
        if self._region == "BR-PB" {
            return Ok(vec![Holiday {
                name: "Nossa Senhora das Neves".to_owned(),
                date: parse_date(&(format!("{}-08-05", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some("BR-PB".to_owned()),
            }]);
        }
        if self._region == "BR-PE" {
            return Ok(vec![
                Holiday {
                    name: "Revolução Pernambucana".to_owned(),
                    date: parse_date(&(format!("{}-03-06", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-PE".to_owned()),
                },
                Holiday {
                    name: "São João".to_owned(),
                    date: parse_date(&(format!("{}-06-24", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-PE".to_owned()),
                },
            ]);
        }
        if self._region == "BR-PI" {
            return Ok(vec![
                Holiday {
                    name: "Dia da Batalha do Jenipapo".to_owned(),
                    date: parse_date(&(format!("{}-03-13", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-PI".to_owned()),
                },
                Holiday {
                    name: "Dia do Piauí".to_owned(),
                    date: parse_date(&(format!("{}-10-19", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-PI".to_owned()),
                },
            ]);
        }
        if self._region == "BR-PR" {
            return Ok(vec![Holiday {
                name: "Emancipação Política do Paraná".to_owned(),
                date: parse_date(&(format!("{}-12-19", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some("BR-PR".to_owned()),
            }]);
        }
        if self._region == "BR-RJ" {
            return Ok(vec![Holiday {
                name: "São Jorge".to_owned(),
                date: parse_date(&(format!("{}-04-23", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some("BR-RJ".to_owned()),
            }]);
        }
        if self._region == "BR-RN" {
            return Ok(vec![
                Holiday {
                    name: "Dia do Rio Grande do Norte".to_owned(),
                    date: parse_date(&(format!("{}-08-07", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-RN".to_owned()),
                },
                Holiday {
                    name: "Mártires de Cunhaú e Uruaçu".to_owned(),
                    date: parse_date(&(format!("{}-10-03", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-RN".to_owned()),
                },
            ]);
        }
        if self._region == "BR-RO" {
            return Ok(vec![
                Holiday {
                    name: "Criação do Estado de Rondônia".to_owned(),
                    date: parse_date(&(format!("{}-01-04", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-RO".to_owned()),
                },
                Holiday {
                    name: "Dia do Evangélico".to_owned(),
                    date: parse_date(&(format!("{}-06-18", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-RO".to_owned()),
                },
            ]);
        }
        if self._region == "BR-RR" {
            return Ok(vec![Holiday {
                name: "Criação do Estado de Roraima".to_owned(),
                date: parse_date(&(format!("{}-10-05", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some("BR-RR".to_owned()),
            }]);
        }
        if self._region == "BR-RS" {
            return Ok(vec![Holiday {
                name: "Revolução Farroupilha".to_owned(),
                date: parse_date(&(format!("{}-09-20", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some("BR-RS".to_owned()),
            }]);
        }
        if self._region == "BR-SC" {
            return Ok(vec![
                Holiday {
                    name: "Criação da Capitania de Santa Catarina".to_owned(),
                    date: parse_date(&(format!("{}-08-11", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-SC".to_owned()),
                },
                Holiday {
                    name: "Santa Catarina de Alexandria".to_owned(),
                    date: parse_date(&(format!("{}-11-25", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-SC".to_owned()),
                },
            ]);
        }
        if self._region == "BR-SE" {
            return Ok(vec![Holiday {
                name: "Emancipação Política de Sergipe".to_owned(),
                date: parse_date(&(format!("{}-07-08", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some("BR-SE".to_owned()),
            }]);
        }
        if self._region == "BR-SP" {
            return Ok(vec![Holiday {
                name: "Revolução Constitucionalista de 1932".to_owned(),
                date: parse_date(&(format!("{}-07-09", year)))?,
                holiday_type: HolidayType::Regional,
                region: Some("BR-SP".to_owned()),
            }]);
        }
        if self._region == "BR-TO" {
            return Ok(vec![
                Holiday {
                    name: "Nossa Senhora da Natividade".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-TO".to_owned()),
                },
                Holiday {
                    name: "Criação do Estado do Tocantins".to_owned(),
                    date: parse_date(&(format!("{}-10-05", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some("BR-TO".to_owned()),
                },
            ]);
        }
        Ok(vec![])
    }
}
