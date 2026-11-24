use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, year: i32) -> Result<Vec<Holiday>, Error> {
        Ok(vec![
            Holiday {
                name: "လွတ်လပ်ရေးနေ့".to_owned(),
                date: parse_date(&(format!("{}-01-04", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "ကရင်နှစ်သစ်ကူးနေ့".to_owned(),
                date: parse_date(&(format!("{}-01-11", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "ပြည်ထောင်စုနေ့".to_owned(),
                date: parse_date(&(format!("{}-02-12", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "တောင်သူလယ်သမားနေ့".to_owned(),
                date: parse_date(&(format!("{}-03-02", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "တပေါင်းလပြည့်နေ့".to_owned(),
                date: parse_date(&(format!("{}-03-24", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "တပ်မတော်နေ့".to_owned(),
                date: parse_date(&(format!("{}-03-27", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "မြန်မာနှစ်သစ်ကူးရုံးပိတ်ရက် ၁".to_owned(),
                date: parse_date(&(format!("{}-04-13", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "မြန်မာနှစ်သစ်ကူးရုံးပိတ်ရက် ၂".to_owned(),
                date: parse_date(&(format!("{}-04-14", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "မြန်မာနှစ်သစ်ကူးရုံးပိတ်ရက် ၃".to_owned(),
                date: parse_date(&(format!("{}-04-15", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "မြန်မာနှစ်သစ်ကူးရုံးပိတ်ရက် ၄".to_owned(),
                date: parse_date(&(format!("{}-04-16", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "မြန်မာနှစ်သစ်ကူးရုံးပိတ်ရက် ၅".to_owned(),
                date: parse_date(&(format!("{}-04-17", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "မြန်မာနှစ်သစ်ကူးရုံးပိတ်ရက် ၆".to_owned(),
                date: parse_date(&(format!("{}-04-18", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "မြန်မာနှစ်သစ်ကူးရုံးပိတ်ရက် ၇".to_owned(),
                date: parse_date(&(format!("{}-04-19", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "မြန်မာနှစ်သစ်ကူးရုံးပိတ်ရက် ၈".to_owned(),
                date: parse_date(&(format!("{}-04-20", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "မြန်မာနှစ်သစ်ကူးရုံးပိတ်ရက် ၉".to_owned(),
                date: parse_date(&(format!("{}-04-21", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "အလုပ်သမားနေ့".to_owned(),
                date: parse_date(&(format!("{}-05-01", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "ကဆုန်လပြည့်နေ့".to_owned(),
                date: parse_date(&(format!("{}-05-22", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "အာဇာနည်နေ့".to_owned(),
                date: parse_date(&(format!("{}-07-19", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "ဝါဆိုလပြည့်နေ့".to_owned(),
                date: parse_date(&(format!("{}-07-20", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "သီတင်းကျွတ်ရုံးပိတ်ရက် ၁".to_owned(),
                date: parse_date(&(format!("{}-10-16", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "သီတင်းကျွတ်ရုံးပိတ်ရက် ၂".to_owned(),
                date: parse_date(&(format!("{}-10-17", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "သီတင်းကျွတ်ရုံးပိတ်ရက် ၃".to_owned(),
                date: parse_date(&(format!("{}-10-18", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "တန်ဆောင်မုန်းလပြည့်နေ့".to_owned(),
                date: parse_date(&(format!("{}-11-15", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "အမျိုးသားအောင်ပွဲနေ့".to_owned(),
                date: parse_date(&(format!("{}-11-25", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
            Holiday {
                name: "ခရစ်စမတ်နေ့".to_owned(),
                date: parse_date(&(format!("{}-12-25", year)))?,
                holiday_type: HolidayType::National,
                region: None,
            },
        ])
    }
}
