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
                    name: "Dia da Liberdade".to_owned(),
                    date: parse_date(&(format!("{}-04-25", year)))?,
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
                    name: "Dia de Portugal".to_owned(),
                    date: parse_date(&(format!("{}-06-10", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Assunção da Nossa Senhora".to_owned(),
                    date: parse_date(&(format!("{}-08-15", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Implantação da República".to_owned(),
                    date: parse_date(&(format!("{}-10-05", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Dia de Todos os Santos".to_owned(),
                    date: parse_date(&(format!("{}-11-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Restauração da Independência".to_owned(),
                    date: parse_date(&(format!("{}-12-01", year)))?,
                    holiday_type: HolidayType::National,
                    region: None,
                },
                Holiday {
                    name: "Imaculada Conceição".to_owned(),
                    date: parse_date(&(format!("{}-12-08", year)))?,
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
                name: "Páscoa".to_owned(),
                date: easter,
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
                name: "Corpo de Deus".to_owned(),
                date: (easter) + Duration::days(i64::from(60)),
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
    pub(super) fn region_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        if self._region.is_empty() {
            return Ok(Vec::new());
        }
        match &self._region[3..5] {
            "01" => self.get_aveiro_holidays(year),
            "02" => self.get_beja_holidays(year),
            "03" => self.get_braga_holidays(year),
            "04" => self.get_braganca_holidays(year),
            "05" => self.get_castelo_branco_holidays(year),
            "06" => self.get_coimbra_holidays(year),
            "07" => self.get_evora_holidays(year),
            "08" => self.get_faro_holidays(year),
            "09" => self.get_guarda_holidays(year),
            "10" => self.get_leiria_holidays(year),
            "11" => self.get_lisboa_holidays(year),
            "12" => self.get_portalegre_holidays(year),
            "13" => self.get_porto_holidays(year),
            "14" => self.get_santarem_holidays(year),
            "15" => self.get_setubal_holidays(year),
            "16" => self.get_viana_do_castelo_holidays(year),
            "17" => self.get_vila_real_holidays(year),
            "18" => self.get_viseu_holidays(year),
            "31" | "32" => self.get_madeira_holidays(year),
            "41" | "42" | "43" | "44" | "45" | "46" | "47" | "48" | "49" => {
                self.get_acores_holidays(year)
            }
            _ => Ok(Vec::new()),
        }
    }
    pub(super) fn get_easter_based_date(&self, year: i32, days: i32) -> Result<Date, Error> {
        Ok((super::easter(year, false)?) + Duration::days(i64::from(days)))
    }
    pub(super) fn get_segunda_feira_de_pascoa_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "Segunda-feira de Páscoa".to_owned(),
            date: self.get_easter_based_date(year, 1)?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_quinta_feira_da_ascensao_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "Quinta-feira da Ascensão".to_owned(),
            date: self.get_easter_based_date(year, 39)?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_santo_antonio_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "Santo António".to_owned(),
            date: parse_date(&(format!("{}-06-13", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_joao_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São João Baptista".to_owned(),
            date: parse_date(&(format!("{}-06-24", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_pedro_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São Pedro".to_owned(),
            date: parse_date(&(format!("{}-06-29", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_vicente_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São Vicente".to_owned(),
            date: parse_date(&(format!("{}-01-22", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_jose_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São José".to_owned(),
            date: parse_date(&(format!("{}-03-19", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_bento_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São Bento".to_owned(),
            date: parse_date(&(format!("{}-07-11", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_santa_maria_madalena_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "Santa Maria Madalena".to_owned(),
            date: parse_date(&(format!("{}-07-22", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_tiago_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São Tiago".to_owned(),
            date: parse_date(&(format!("{}-07-25", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_lourenco_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São Lourenço".to_owned(),
            date: parse_date(&(format!("{}-08-10", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_nossa_senhora_do_socorro_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "Nossa Senhora do Socorro".to_owned(),
            date: parse_date(&(format!("{}-08-16", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_bernardo_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São Bernardo".to_owned(),
            date: parse_date(&(format!("{}-08-20", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_bartolomeu_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São Bartolomeu".to_owned(),
            date: parse_date(&(format!("{}-08-24", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_nossa_senhora_da_luz_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "Nossa Senhora da Luz".to_owned(),
            date: parse_date(&(format!("{}-09-08", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_mateus_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São Mateus".to_owned(),
            date: parse_date(&(format!("{}-09-21", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_miguel_arcanjo_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São Miguel Arcanjo".to_owned(),
            date: parse_date(&(format!("{}-09-29", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_sao_martinho_holiday(&self, year: i32) -> Result<Holiday, Error> {
        Ok(Holiday {
            name: "São Martinho".to_owned(),
            date: parse_date(&(format!("{}-11-11", year)))?,
            holiday_type: HolidayType::Regional,
            region: Some(self._region.to_owned()),
        })
    }
    pub(super) fn get_aveiro_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-0101" => {
                vec![Holiday {
                    name: "Feriado Municipal de Águeda".to_owned(),
                    date: self.get_easter_based_date(year, 50)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0102" => {
                vec![Holiday {
                    name: "Segunda-feira da Senhora do Socorro".to_owned(),
                    date: (parse_date(&(format!("third sunday of august {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0103" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-0104" => {
                vec![Holiday {
                    name: "Rainha Santa Mafalda".to_owned(),
                    date: parse_date(&(format!("{}-05-02", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0105" => {
                vec![Holiday {
                    name: "Santa Joana Princesa".to_owned(),
                    date: parse_date(&(format!("{}-05-12", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0106" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-0107" => {
                vec![Holiday {
                    name: "Dia da Cidade de Espinho".to_owned(),
                    date: parse_date(&(format!("{}-06-16", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0108" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-0110" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-0111" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-0112" => {
                vec![Holiday {
                    name: "Nossa Senhora da Natividade".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0113" => {
                vec![Holiday {
                    name: "Nossa Senhora de La Salette".to_owned(),
                    date: (parse_date(&(format!("second sunday of august {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0114" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-0115" => {
                vec![Holiday {
                    name: "São Cristóvão".to_owned(),
                    date: parse_date(&(format!("{}-07-25", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0109" => {
                vec![Holiday {
                    name: "São Sebastião".to_owned(),
                    date: parse_date(&(format!("{}-01-20", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0116" => {
                vec![Holiday {
                    name: "Elevação a cidade e fundação do município".to_owned(),
                    date: parse_date(&(format!("{}-10-11", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0117" => {
                vec![self.get_sao_mateus_holiday(year)?]
            }
            _ if self._region == "PT-0118" => {
                vec![Holiday {
                    name: "Dia do Município e Nossa Senhora de Vagos".to_owned(),
                    date: self.get_easter_based_date(year, 50)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0119" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_beja_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-0201" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-0202" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-0203" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-0204" => {
                vec![Holiday {
                    name: "Festa de Nossa Senhora da Conceição".to_owned(),
                    date: parse_date(&(format!("{}-08-28", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0205" => {
                vec![Holiday {
                    name: "Dia da Espiga".to_owned(),
                    date: self.get_easter_based_date(year, 39)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0206" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-0207" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-0208" => {
                vec![Holiday {
                    name: "Outorgação do foral manuelino".to_owned(),
                    date: parse_date(&(format!("{}-03-05", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0209" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-0210" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-0211" => {
                vec![Holiday {
                    name: "Nossa Senhora da Piedade".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0212" => {
                vec![Holiday {
                    name: "Nossa Senhora da Cola".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0213" => {
                vec![Holiday {
                    name: "Nossa Senhora de Guadalupe".to_owned(),
                    date: self.get_easter_based_date(year, 2)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0214" => {
                vec![Holiday {
                    name: "Dia da Espiga".to_owned(),
                    date: self.get_easter_based_date(year, 39)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_braga_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-0301" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-0302" => {
                vec![Holiday {
                    name: "Santa Cruz".to_owned(),
                    date: parse_date(&(format!("{}-05-03", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0303" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-0304" => {
                vec![self.get_sao_miguel_arcanjo_holiday(year)?]
            }
            _ if self._region == "PT-0305" => {
                vec![self.get_sao_tiago_holiday(year)?]
            }
            _ if self._region == "PT-0306" => {
                vec![Holiday {
                    name: "Elevação a vila".to_owned(),
                    date: parse_date(&(format!("{}-08-19", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0307" => {
                vec![Holiday {
                    name: "Feiras Francas".to_owned(),
                    date: parse_date(&(format!("{}-05-16", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0308" => {
                vec![Holiday {
                    name: "Batalha de São Mamede".to_owned(),
                    date: parse_date(&(format!("{}-06-24", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0309" => {
                vec![self.get_sao_jose_holiday(year)?]
            }
            _ if self._region == "PT-0310" => {
                vec![Holiday {
                    name: "Foral de D. Manuel I".to_owned(),
                    date: parse_date(&(format!("{}-10-20", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0311" => {
                vec![Holiday {
                    name: "Feira da Ladra".to_owned(),
                    date: (parse_date(&(format!("first saturday of october {}", year)))?)
                        + Duration::days(i64::from(2)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0312" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-0313" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-0314" => {
                vec![Holiday {
                    name: "Restauração do município".to_owned(),
                    date: parse_date(&(format!("{}-03-19", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_braganca_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-0401" => {
                vec![Holiday {
                    name: "Foral de D. Dinis".to_owned(),
                    date: parse_date(&(format!("{}-05-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0402" => {
                vec![Holiday {
                    name: "Nossa Senhora das Graças".to_owned(),
                    date: parse_date(&(format!("{}-08-22", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0403" => {
                vec![Holiday {
                    name: "Feriado Municipal de Carrazeda de Ansiães".to_owned(),
                    date: parse_date(&(format!("{}-08-31", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0404" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-0405" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-0406" => {
                vec![Holiday {
                    name: "Elevação a cidade".to_owned(),
                    date: parse_date(&(format!("{}-07-10", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0407" => {
                vec![Holiday {
                    name: "Outorgação do Foral de D. Afonso III".to_owned(),
                    date: parse_date(&(format!("{}-05-25", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0408" => {
                vec![Holiday {
                    name: "Feira dos Gorazes".to_owned(),
                    date: parse_date(&(format!("{}-10-15", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0409" => {
                vec![self.get_sao_jose_holiday(year)?]
            }
            _ if self._region == "PT-0410" => {
                vec![self.get_sao_bartolomeu_holiday(year)?]
            }
            _ if self._region == "PT-0411" => {
                vec![self.get_sao_lourenco_holiday(year)?]
            }
            _ if self._region == "PT-0412" => {
                vec![Holiday {
                    name: "Outorgação do Foral Manuelino".to_owned(),
                    date: parse_date(&(format!("{}-05-20", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_castelo_branco_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-0501" => {
                vec![Holiday {
                    name: "Primeira missa no Brasil".to_owned(),
                    date: parse_date(&(format!("{}-04-26", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0502" => {
                vec![Holiday {
                    name: "Nossa Senhora de Mércoles".to_owned(),
                    date: self.get_easter_based_date(year, 2)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0503" => {
                vec![Holiday {
                    name: "Elevação a cidade".to_owned(),
                    date: parse_date(&(format!("{}-10-20", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0504" => {
                vec![Holiday {
                    name: "Santa Luzia".to_owned(),
                    date: parse_date(&(format!("{}-09-15", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0505" => {
                vec![Holiday {
                    name: "Nossa Senhora do Almortão".to_owned(),
                    date: self.get_easter_based_date(year, 15)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0506" => {
                vec![Holiday {
                    name: "Santa Margarida".to_owned(),
                    date: (parse_date(&(format!("second sunday of august {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0507" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-0508" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-0509" => {
                vec![Holiday {
                    name: "São Nuno de Santa Maria".to_owned(),
                    date: parse_date(&(format!("{}-06-24", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0510" => {
                vec![Holiday {
                    name: "Recebimento do Foral de D. Dinis".to_owned(),
                    date: parse_date(&(format!("{}-09-19", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0511" => {
                vec![Holiday {
                    name: "Nossa Senhora da Alagada".to_owned(),
                    date: (parse_date(&(format!("fourth sunday of august {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_coimbra_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-0601" => {
                vec![Holiday {
                    name: "Nossa Senhora do Mont'Alto".to_owned(),
                    date: parse_date(&(format!("{}-09-07", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0602" => {
                vec![self.get_sao_tiago_holiday(year)?]
            }
            _ if self._region == "PT-0603" => {
                vec![Holiday {
                    name: "Rainha Santa Isabel".to_owned(),
                    date: parse_date(&(format!("{}-07-04", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0604" => {
                vec![Holiday {
                    name: "Santa Cristina".to_owned(),
                    date: parse_date(&(format!("{}-07-24", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0605" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-0606" => {
                vec![Holiday {
                    name: "Doação de Góis a Anaia Vestrares".to_owned(),
                    date: parse_date(&(format!("{}-08-13", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0607" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-0608" => {
                vec![Holiday {
                    name: "São Tomé".to_owned(),
                    date: parse_date(&(format!("{}-07-25", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0609" => {
                vec![Holiday {
                    name: "Nascimento de José Falcão".to_owned(),
                    date: parse_date(&(format!("{}-06-01", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0610" => {
                vec![Holiday {
                    name: "Nossa Senhora da Vitória".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0611" => {
                vec![Holiday {
                    name: "Recebimento da notícia da Implantação da República".to_owned(),
                    date: parse_date(&(format!("{}-10-07", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0612" => {
                vec![Holiday {
                    name: "Restauração dos títulos de vila isenta".to_owned(),
                    date: parse_date(&(format!("{}-04-10", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0613" => {
                vec![Holiday {
                    name: "Nascimento de António José de Almeida".to_owned(),
                    date: parse_date(&(format!("{}-07-17", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0614" => {
                vec![self.get_sao_miguel_arcanjo_holiday(year)?]
            }
            _ if self._region == "PT-0615" => {
                vec![self.get_sao_mateus_holiday(year)?]
            }
            _ if self._region == "PT-0616" => {
                vec![Holiday {
                    name: "Restauração da Comarca".to_owned(),
                    date: parse_date(&(format!("{}-04-10", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0617" => {
                vec![Holiday {
                    name: "Restauração do município".to_owned(),
                    date: parse_date(&(format!("{}-01-13", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_evora_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-0701" => {
                vec![Holiday {
                    name: "Segunda-feira de Pascoela (Nossa Senhora da Boa Nova)".to_owned(),
                    date: self.get_easter_based_date(year, 8)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0702" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-0703" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-0704" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-0705" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-0706" => {
                vec![Holiday {
                    name: "São João de Deus".to_owned(),
                    date: parse_date(&(format!("{}-03-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0707" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-0708" => {
                vec![Holiday {
                    name: "Nossa Senhora das Candeias".to_owned(),
                    date: parse_date(&(format!("{}-02-02", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0709" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-0710" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-0711" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-0712" => {
                vec![Holiday {
                    name: "Elevação a concelho".to_owned(),
                    date: parse_date(&(format!("{}-09-07", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0713" => {
                vec![Holiday {
                    name: "Restauração do concelho".to_owned(),
                    date: parse_date(&(format!("{}-01-13", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0714" => {
                vec![Holiday {
                    name: "Nascimento de Couto Jardim".to_owned(),
                    date: parse_date(&(format!("{}-08-16", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_faro_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-0801" => {
                vec![Holiday {
                    name: "Foral de D. Manuel I".to_owned(),
                    date: parse_date(&(format!("{}-08-20", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0802" => {
                vec![Holiday {
                    name: "Festas de Alcoutim".to_owned(),
                    date: parse_date(&(format!("second friday of september {}", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0803" => {
                vec![Holiday {
                    name: "Banho Santo".to_owned(),
                    date: parse_date(&(format!("{}-08-29", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0804" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-0805" => {
                vec![Holiday {
                    name: "Elevação a cidade".to_owned(),
                    date: parse_date(&(format!("{}-09-07", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0806" => {
                vec![self.get_nossa_senhora_da_luz_holiday(year)?]
            }
            _ if self._region == "PT-0807" => {
                vec![Holiday {
                    name: "São Gonçalo de Lagos".to_owned(),
                    date: parse_date(&(format!("{}-10-27", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0808" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-0809" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-0810" => {
                vec![Holiday {
                    name: "Revolta de Olhão contra os Franceses".to_owned(),
                    date: parse_date(&(format!("{}-06-16", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0811" => {
                vec![Holiday {
                    name: "Elevação a cidade".to_owned(),
                    date: parse_date(&(format!("{}-12-11", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0812" => {
                vec![Holiday {
                    name: "Elevação a município".to_owned(),
                    date: parse_date(&(format!("{}-06-01", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0813" => {
                vec![Holiday {
                    name: "Conquista da cidade aos Mouros".to_owned(),
                    date: parse_date(&(format!("{}-09-03", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0814" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-0815" => {
                vec![self.get_sao_vicente_holiday(year)?]
            }
            _ if self._region == "PT-0816" => {
                vec![Holiday {
                    name: "Elevação a cidade".to_owned(),
                    date: parse_date(&(format!("{}-05-13", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_guarda_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-0901" => {
                vec![Holiday {
                    name: "Restauração do concelho".to_owned(),
                    date: parse_date(&(format!("{}-02-10", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0902" => {
                vec![Holiday {
                    name: "Batalha do Côa".to_owned(),
                    date: parse_date(&(format!("{}-07-02", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0903" => {
                vec![Holiday {
                    name: "Nascimento de Sacadura Cabral".to_owned(),
                    date: parse_date(&(format!("{}-05-23", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0904" => {
                vec![Holiday {
                    name: "Batalha de Salgadela".to_owned(),
                    date: parse_date(&(format!("{}-07-07", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0905" => {
                vec![self.get_sao_miguel_arcanjo_holiday(year)?]
            }
            _ if self._region == "PT-0906" => {
                vec![Holiday {
                    name: "Nosso Senhor do Calvário".to_owned(),
                    date: (parse_date(&(format!("second sunday of august {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0907" => {
                vec![Holiday {
                    name: "Foral de D. Sancho I".to_owned(),
                    date: parse_date(&(format!("{}-11-27", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0908" => {
                vec![Holiday {
                    name: "Outorgação do Foral".to_owned(),
                    date: parse_date(&(format!("{}-03-04", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0909" => {
                vec![self.get_sao_martinho_holiday(year)?]
            }
            _ if self._region == "PT-0910" => {
                vec![Holiday {
                    name: "Elevação a cidade".to_owned(),
                    date: parse_date(&(format!("{}-08-25", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0911" => {
                vec![Holiday { name: "Segunda-feira de Pascoela (Nossa Senhora dos Prazeres / Nossa Senhora da Graça)".to_owned(), date: self.get_easter_based_date(year, 8)?, holiday_type: HolidayType::Regional, region: Some(self._region.to_owned()) }]
            }
            _ if self._region == "PT-0912" => {
                vec![Holiday {
                    name: "Elevação a cidade".to_owned(),
                    date: parse_date(&(format!("{}-07-03", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0913" => {
                vec![Holiday {
                    name: "Batalha de São Marcos".to_owned(),
                    date: parse_date(&(format!("{}-05-29", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-0914" => {
                vec![Holiday {
                    name: "Outorgação do foral de D. Dinis".to_owned(),
                    date: parse_date(&(format!("{}-05-21", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_leiria_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-1001" => {
                vec![self.get_sao_bernardo_holiday(year)?]
            }
            _ if self._region == "PT-1002" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-1003" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1004" => {
                vec![Holiday {
                    name: "Batalha de Aljubarrota".to_owned(),
                    date: parse_date(&(format!("{}-08-14", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1005" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-1006" => {
                vec![Holiday {
                    name: "Fundação da localidade pela rainha D. Leonor".to_owned(),
                    date: parse_date(&(format!("{}-05-15", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1007" => {
                vec![Holiday {
                    name: "Fundação do município".to_owned(),
                    date: parse_date(&(format!("{}-07-04", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1008" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1009" => {
                vec![Holiday {
                    name: "Elevação a cidade e fundação da diocese".to_owned(),
                    date: parse_date(&(format!("{}-05-22", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1010" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1011" => {
                vec![Holiday {
                    name: "Nossa Senhora das Areias (ou da Nazaré)".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1012" => {
                vec![Holiday {
                    name: "Tomada de Óbidos aos Mouros".to_owned(),
                    date: parse_date(&(format!("{}-01-11", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1013" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-07-24", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1014" => {
                vec![Holiday {
                    name: "Nossa Senhora da Boa Viagem".to_owned(),
                    date: (parse_date(&(format!("first sunday of august {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1015" => {
                vec![self.get_sao_martinho_holiday(year)?]
            }
            _ if self._region == "PT-1016" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_lisboa_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-1101" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1115" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-09-11", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1102" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1103" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1104" => {
                vec![Holiday {
                    name: "Restauração do município".to_owned(),
                    date: parse_date(&(format!("{}-01-13", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1105" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-1106" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-1107" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-07-26", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1108" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1109" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1116" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-11-19", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1110" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-06-07", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1111" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-1112" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1113" => {
                vec![self.get_sao_martinho_holiday(year)?]
            }
            _ if self._region == "PT-1114" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_portalegre_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-1201" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1202" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1203" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-1204" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-1205" => {
                vec![Holiday {
                    name: "Nossa Senhora da Luz".to_owned(),
                    date: self.get_easter_based_date(year, 1)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1206" => {
                vec![Holiday {
                    name: "São Gregório".to_owned(),
                    date: self.get_easter_based_date(year, 1)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1207" => {
                vec![Holiday {
                    name: "Batalha das Linhas de Elvas".to_owned(),
                    date: parse_date(&(format!("{}-01-14", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1208" => {
                vec![Holiday {
                    name: "Batalha dos Atoleiros".to_owned(),
                    date: parse_date(&(format!("{}-04-06", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1209" => {
                vec![Holiday {
                    name: "Recebimento do Foral Manuelino".to_owned(),
                    date: parse_date(&(format!("{}-11-23", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1210" => {
                vec![Holiday {
                    name: "Nossa Senhora da Estrela".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1211" => {
                vec![Holiday {
                    name: "Segunda-feira de Pascoela (Nossa Senhora dos Prazeres)".to_owned(),
                    date: self.get_easter_based_date(year, 8)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1212" => {
                vec![Holiday {
                    name: "Nossa Senhora da Graça".to_owned(),
                    date: self.get_easter_based_date(year, 1)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1213" => {
                vec![Holiday {
                    name: "Festa da Salgueirinha".to_owned(),
                    date: self.get_easter_based_date(year, 1)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1214" => {
                vec![Holiday {
                    name: "Elevação a cidade e fundação da diocese".to_owned(),
                    date: parse_date(&(format!("{}-05-23", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1215" => {
                vec![Holiday {
                    name: "Nossa Senhora do Carmo da Serra (de São Miguel)".to_owned(),
                    date: self.get_easter_based_date(year, 1)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_porto_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-1301" => {
                vec![Holiday {
                    name: "Elevação a cidade".to_owned(),
                    date: parse_date(&(format!("{}-07-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1302" => {
                vec![self.get_sao_bartolomeu_holiday(year)?]
            }
            _ if self._region == "PT-1303" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-1304" => {
                vec![Holiday {
                    name: "Nossa Senhora do Rosário".to_owned(),
                    date: (parse_date(&(format!("first sunday of october {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1305" => {
                vec![Holiday {
                    name: "Nosso Senhor dos Aflitos".to_owned(),
                    date: (parse_date(&(format!("last sunday of july {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1306" => {
                vec![Holiday {
                    name: "Nossa Senhora do Bom Despacho".to_owned(),
                    date: (parse_date(&(format!("second sunday of july {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1307" => {
                vec![Holiday {
                    name: "Nossa Senhora da Natividade do Castelinho".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1308" => {
                vec![Holiday {
                    name: "Terça-feira de Pentecostes".to_owned(),
                    date: self.get_easter_based_date(year, 51)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1309" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-11-06", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1310" => {
                vec![Holiday {
                    name: "Divino Salvador".to_owned(),
                    date: (parse_date(&(format!("third sunday of july {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1311" => {
                vec![self.get_sao_martinho_holiday(year)?]
            }
            _ if self._region == "PT-1312" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1313" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-1314" => {
                vec![self.get_sao_bento_holiday(year)?]
            }
            _ if self._region == "PT-1315" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1316" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1317" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1318" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-11-19", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_santarem_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-1401" => {
                vec![Holiday {
                    name: "Elevação a cidade".to_owned(),
                    date: parse_date(&(format!("{}-06-14", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1402" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1403" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1404" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-04-02", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1405" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1406" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1407" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1408" => {
                vec![Holiday {
                    name: "Nossa Senhora da Boa Viagem".to_owned(),
                    date: self.get_easter_based_date(year, 1)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1409" => {
                vec![Holiday {
                    name: "Dia do Campino do Sorraia".to_owned(),
                    date: parse_date(&(format!("{}-08-17", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1410" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-11-24", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1411" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-1412" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1413" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-1414" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-11-06", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1415" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1416" => {
                vec![self.get_sao_jose_holiday(year)?]
            }
            _ if self._region == "PT-1417" => {
                vec![Holiday {
                    name: "Elevação à categoria de vila".to_owned(),
                    date: parse_date(&(format!("{}-09-22", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1418" => {
                vec![Holiday {
                    name: "Fundação do Castelo Templário".to_owned(),
                    date: parse_date(&(format!("{}-03-01", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1419" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1420" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ if self._region == "PT-1421" => {
                vec![Holiday {
                    name: "Elevação a cidade".to_owned(),
                    date: parse_date(&(format!("{}-06-20", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_setubal_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-1501" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1502" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1503" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1504" => {
                vec![Holiday {
                    name: "Elevação a cidade".to_owned(),
                    date: parse_date(&(format!("{}-06-28", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1505" => {
                vec![Holiday {
                    name: "Criação do município através do foral".to_owned(),
                    date: parse_date(&(format!("{}-10-22", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1506" => {
                vec![Holiday {
                    name: "Festas em Honra de Nossa Senhora da Boa Viagem".to_owned(),
                    date: (parse_date(&(format!("second sunday of september {}", year)))?)
                        + Duration::days(i64::from(2)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1507" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-1508" => {
                vec![Holiday {
                    name: "Atribuição do Foral Manuelino".to_owned(),
                    date: parse_date(&(format!("{}-06-01", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1509" => {
                vec![self.get_sao_tiago_holiday(year)?]
            }
            _ if self._region == "PT-1510" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-1511" => {
                vec![Holiday {
                    name: "Nosso Senhor Jesus das Chagas".to_owned(),
                    date: parse_date(&(format!("{}-05-04", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1512" => {
                vec![Holiday {
                    name: "Nascimento de Bocage".to_owned(),
                    date: parse_date(&(format!("{}-09-15", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1513" => {
                vec![Holiday {
                    name: "Elevação a vila".to_owned(),
                    date: parse_date(&(format!("{}-11-24", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_viana_do_castelo_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-1601" => {
                vec![self.get_sao_bento_holiday(year)?]
            }
            _ if self._region == "PT-1602" => {
                vec![self.get_segunda_feira_de_pascoa_holiday(year)?]
            }
            _ if self._region == "PT-1603" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1604" => {
                vec![Holiday {
                    name: "Outorgação do Foral".to_owned(),
                    date: parse_date(&(format!("{}-03-12", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1605" => {
                vec![self.get_sao_lourenco_holiday(year)?]
            }
            _ if self._region == "PT-1606" => {
                vec![self.get_sao_bartolomeu_holiday(year)?]
            }
            _ if self._region == "PT-1607" => {
                vec![if (year) >= (2025) {
                    Holiday {
                        name: "Outorgação do Foral a vila por D. Teresa".to_owned(),
                        date: parse_date(&(format!("{}-03-04", year)))?,
                        holiday_type: HolidayType::Regional,
                        region: Some(self._region.to_owned()),
                    }
                } else {
                    Holiday {
                        name: "Nossa Senhora das Dores".to_owned(),
                        date: (parse_date(&(format!("second sunday of september {}", year)))?)
                            + Duration::days(i64::from(2)),
                        holiday_type: HolidayType::Regional,
                        region: Some(self._region.to_owned()),
                    }
                }]
            }
            _ if self._region == "PT-1608" => {
                vec![Holiday {
                    name: "São Teotónio".to_owned(),
                    date: parse_date(&(format!("{}-02-18", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1609" => {
                vec![Holiday {
                    name: "Nossa Senhora da Agonia".to_owned(),
                    date: parse_date(&(format!("{}-08-20", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1610" => {
                vec![Holiday {
                    name: "Dia do Município, dos Bombeiros Voluntários e da Proteção Civil"
                        .to_owned(),
                    date: parse_date(&(format!("{}-10-01", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_vila_real_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-1701" => {
                vec![self.get_sao_martinho_holiday(year)?]
            }
            _ if self._region == "PT-1702" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-11-06", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1703" => {
                vec![Holiday {
                    name: "Batalha de Chaves".to_owned(),
                    date: parse_date(&(format!("{}-07-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1704" => {
                vec![Holiday {
                    name: "Santo André".to_owned(),
                    date: parse_date(&(format!("{}-11-30", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1705" => {
                vec![self.get_sao_tiago_holiday(year)?]
            }
            _ if self._region == "PT-1706" => {
                vec![Holiday {
                    name: "Atribuição do foral".to_owned(),
                    date: parse_date(&(format!("{}-06-09", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1707" => {
                vec![Holiday {
                    name: "Fundação do município".to_owned(),
                    date: parse_date(&(format!("{}-05-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1708" => {
                vec![self.get_nossa_senhora_do_socorro_holiday(year)?]
            }
            _ if self._region == "PT-1709" => {
                vec![self.get_nossa_senhora_do_socorro_holiday(year)?]
            }
            _ if self._region == "PT-1710" => {
                vec![Holiday {
                    name: "Nossa Senhora do Rosário".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1711" => {
                vec![Holiday {
                    name: "Santa Marta".to_owned(),
                    date: parse_date(&(format!("{}-07-29", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1712" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-11-06", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1713" => {
                vec![Holiday {
                    name: "Atribuição do Foral Manuelino".to_owned(),
                    date: parse_date(&(format!("{}-05-16", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1714" => {
                vec![self.get_santo_antonio_holiday(year)?]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_viseu_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-1801" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1802" => {
                vec![Holiday {
                    name: "Nossa Senhora das Febres".to_owned(),
                    date: (parse_date(&(format!("third sunday of july {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1803" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-1804" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1805" => {
                vec![Holiday {
                    name: "Nossa Senhora dos Remédios".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1806" => {
                vec![Holiday {
                    name: "Nossa Senhora do Castelo".to_owned(),
                    date: parse_date(&(format!("{}-09-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1807" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1808" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1809" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1810" => {
                vec![Holiday {
                    name: "Restauração do município".to_owned(),
                    date: parse_date(&(format!("{}-10-07", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1811" => {
                vec![Holiday {
                    name: "São Genésio".to_owned(),
                    date: parse_date(&(format!("{}-08-25", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1812" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-1813" => {
                vec![self.get_sao_miguel_arcanjo_holiday(year)?]
            }
            _ if self._region == "PT-1814" => {
                vec![self.get_quinta_feira_da_ascensao_holiday(year)?]
            }
            _ if self._region == "PT-1815" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1816" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-1817" => {
                vec![self.get_sao_bernardo_holiday(year)?]
            }
            _ if self._region == "PT-1818" => {
                vec![Holiday {
                    name: "Festa de Nossa Senhora ao Pé da Cruz".to_owned(),
                    date: parse_date(&(format!("{}-05-03", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1819" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-1820" => {
                vec![self.get_sao_miguel_arcanjo_holiday(year)?]
            }
            _ if self._region == "PT-1821" => {
                vec![Holiday {
                    name: "Santa Eufémia".to_owned(),
                    date: parse_date(&(format!("{}-09-16", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1822" => {
                vec![Holiday {
                    name: "Estabelecimento e denominação da sede de concelho".to_owned(),
                    date: parse_date(&(format!("{}-03-02", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-1823" => {
                vec![self.get_sao_mateus_holiday(year)?]
            }
            _ if self._region == "PT-1824" => {
                vec![Holiday {
                    name: "São Frei Gil".to_owned(),
                    date: parse_date(&(format!("{}-05-14", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_madeira_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-3101" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-3102" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-10-04", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-3103" => {
                vec![Holiday {
                    name: "Dia da Cidade do Funchal".to_owned(),
                    date: parse_date(&(format!("{}-08-21", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-3104" => {
                vec![Holiday {
                    name: "Criação da Capitania de Machico".to_owned(),
                    date: parse_date(&(format!("{}-05-08", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-3105" => {
                vec![self.get_nossa_senhora_da_luz_holiday(year)?]
            }
            _ if self._region == "PT-3106" => {
                vec![self.get_santa_maria_madalena_holiday(year)?]
            }
            _ if self._region == "PT-3107" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-3108" => {
                vec![Holiday {
                    name: "Santo Amaro".to_owned(),
                    date: parse_date(&(format!("{}-01-15", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-3109" => {
                vec![Holiday {
                    name: "Fundação do município".to_owned(),
                    date: parse_date(&(format!("{}-05-25", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-3110" => {
                vec![self.get_sao_vicente_holiday(year)?]
            }
            _ if self._region == "PT-3201" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-06-24", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
    pub(super) fn get_acores_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(match () {
            _ if self._region == "PT-4101" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-4201" => {
                vec![Holiday {
                    name: "Criação do município".to_owned(),
                    date: parse_date(&(format!("{}-04-11", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-4202" => {
                vec![Holiday {
                    name: "Fundação do município".to_owned(),
                    date: parse_date(&(format!("{}-07-18", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-4203" => {
                vec![Holiday {
                    name: "Senhor Santo Cristo dos Milagres".to_owned(),
                    date: self.get_easter_based_date(year, 36)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-4204" => {
                vec![Holiday {
                    name: "Oitava do Corpo de Deus".to_owned(),
                    date: self.get_easter_based_date(year, 61)?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-4205" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-4206" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-4301" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-4302" => {
                vec![Holiday {
                    name: "Batalha da Praia".to_owned(),
                    date: parse_date(&(format!("{}-08-11", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-4401" => {
                vec![Holiday {
                    name: "Senhor Santo Cristo dos Milagres".to_owned(),
                    date: (parse_date(&(format!("second sunday of august {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-4501" => {
                vec![Holiday {
                    name: "Santa Catarina de Alexandria".to_owned(),
                    date: parse_date(&(format!("{}-11-25", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-4502" => {
                vec![Holiday {
                    name: "São Jorge".to_owned(),
                    date: parse_date(&(format!("{}-04-23", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-4601" => {
                vec![self.get_sao_pedro_holiday(year)?]
            }
            _ if self._region == "PT-4602" => {
                vec![self.get_santa_maria_madalena_holiday(year)?]
            }
            _ if self._region == "PT-4603" => {
                vec![Holiday {
                    name: "São Roque".to_owned(),
                    date: parse_date(&(format!("{}-08-16", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-4701" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-4801" => {
                vec![Holiday {
                    name: "Nossa Senhora do Rosário".to_owned(),
                    date: (parse_date(&(format!("third sunday of july {}", year)))?)
                        + Duration::days(i64::from(1)),
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ if self._region == "PT-4802" => {
                vec![self.get_sao_joao_holiday(year)?]
            }
            _ if self._region == "PT-4901" => {
                vec![Holiday {
                    name: "Elevação a vila e sede de concelho".to_owned(),
                    date: parse_date(&(format!("{}-06-20", year)))?,
                    holiday_type: HolidayType::Regional,
                    region: Some(self._region.to_owned()),
                }]
            }
            _ => {
                vec![]
            }
        })
    }
}
