use super::*;
pub(super) struct Rules<'a> {
    pub _region: &'a str,
}
impl Rules<'_> {
    pub(super) fn all_holidays(&self, _year: i32) -> Result<Vec<Holiday>, Error> {
        Err(Error::UnsupportedYear)
    }
}
