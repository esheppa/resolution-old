use crate::{month, DateResolution, DateResolutionExt};
use chrono::Datelike;
use std::{convert::TryFrom, fmt, str};

#[derive(
    Clone, Copy, Debug, Eq, PartialOrd, PartialEq, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct Year(i64);

impl crate::DateResolution for Year {
    fn start(&self) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd(self.year_num(), 1, 1)
    }
}

impl crate::TimeResolution for Year {
    fn between(&self, other: Self) -> i64 {
        i64::from(other.0 - self.0)
    }
    fn succ_n(&self, n: u32) -> Year {
        Year(self.0 + i64::from(n))
    }
    fn pred_n(&self, n: u32) -> Year {
        Year(self.0 - i64::from(n))
    }
    fn naive_date_time(&self) -> chrono::NaiveDateTime {
        self.start().and_hms(0, 0, 0)
    }
    fn from_monotonic(idx: i64) -> Self {
        Year(idx)
    }
    fn to_monotonic(&self) -> i64 {
        self.0
    }
    fn name(&self) -> String {
        "Year".to_string()
    }
}

impl From<chrono::NaiveDate> for Year {
    fn from(d: chrono::NaiveDate) -> Self {
        Year(i64::from(d.year()))
    }
}

impl From<chrono::NaiveDateTime> for Year {
    fn from(d: chrono::NaiveDateTime) -> Self {
        d.date().into()
    }
}

impl Year {
    pub fn first_month(&self) -> month::Month {
        self.start().into()
    }
    pub fn first_quarter(&self) -> month::Month {
        self.start().into()
    }
    pub fn last_month(&self) -> month::Month {
        self.end().into()
    }
    pub fn last_quarter(&self) -> month::Month {
        self.end().into()
    }
    pub fn year_num(&self) -> i32 {
        i32::try_from(self.0).expect("Not pre/post historic")
    }
    pub fn new(year: i32) -> Self {
        Year(i64::from(year))
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for Year {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Year(s.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DateResolution, TimeResolution};

    #[test]
    fn test_roundtrip() {
        let dt = chrono::NaiveDate::from_ymd(2021, 12, 6);

        let wk = Year::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            "2021".parse::<Year>().unwrap().start(),
            chrono::NaiveDate::from_ymd(2021, 1, 1),
        );
        assert_eq!(
            "2021".parse::<Year>().unwrap().succ().start(),
            chrono::NaiveDate::from_ymd(2022, 1, 1),
        );
        assert_eq!(
            "2021".parse::<Year>().unwrap().succ().pred().start(),
            chrono::NaiveDate::from_ymd(2021, 1, 1),
        );

        assert!("a2021".parse::<Year>().is_err(),);
    }
}
