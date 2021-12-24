use crate::{quarter, year, DateResolution};
use chrono::Datelike;
use serde::{
    de,
    ser::{self, SerializeStruct},
};
use std::{cmp, convert::TryFrom, fmt, str};

const DATE_FORMAT: &str = "%b-%Y";

impl<'de> de::Deserialize<'de> for Month {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Month, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date =
            chrono::NaiveDate::parse_from_str(&s, DATE_FORMAT).map_err(serde::de::Error::custom)?;
        Ok(Month::from_date(date))
    }
}

impl serde::Serialize for Month {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl str::FromStr for Month {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = chrono::NaiveDate::parse_from_str(s, DATE_FORMAT)?;
        Ok(Month::from_date(date))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Month(i64); // number of months +- since 0AD

impl crate::TimeResolution for Month {
    fn between(&self, other: Self) -> i64 {
        i64::from(other.0 - self.0)
    }
    fn succ_n(&self, n: u32) -> Self {
        Month(self.0 + i64::from(n))
    }
    fn pred_n(&self, n: u32) -> Self {
        Month(self.0 - i64::from(n))
    }
    fn naive_date_time(&self) -> chrono::NaiveDateTime {
        self.start().and_hms(0, 0, 0)
    }
    fn to_monotonic(&self) -> i64 {
        self.0
    }
    fn from_monotonic(idx: i64) -> Self {
        Month(idx)
    }
    fn name(&self) -> String {
        "Month".to_string()
    }
}

impl crate::DateResolution for Month {
    // TODO: Fix??
    fn start(&self) -> chrono::NaiveDate {
        let years = i32::try_from(self.0.div_euclid(12)).expect("Not pre/post historic");
        let months = u32::try_from(1 + self.0.rem_euclid(12)).unwrap();
        dbg!(months);
        chrono::NaiveDate::from_ymd(years, months, 1)
    }
}

impl Month {
    pub fn year(&self) -> year::Year {
        year::Year::from_date(self.start())
    }
    pub fn quarter(&self) -> quarter::Quarter {
        quarter::Quarter::from_date(self.start())
    }
    pub fn year_num(&self) -> i32 {
        self.start().year()
    }
    pub fn month_num(&self) -> u32 {
        self.start().month()
    }
    pub fn from_date(d: chrono::NaiveDate) -> Self {
        todo!()
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.start().format("%b-%Y"))
    }
}

#[cfg(test)]
mod tests {
    use super::Month;
    use crate::DateResolution;

    #[test]
    fn test_start() {
        assert_eq!(
            Month(24240).start(),
            chrono::NaiveDate::from_ymd(2020, 1, 1)
        );
        assert_eq!(
            Month(24249).start(),
            chrono::NaiveDate::from_ymd(2020, 10, 1)
        );
        assert_eq!(Month(15).start(), chrono::NaiveDate::from_ymd(1, 4, 1));
        assert_eq!(Month(2).start(), chrono::NaiveDate::from_ymd(0, 3, 1));
        assert_eq!(Month(1).start(), chrono::NaiveDate::from_ymd(0, 2, 1));
        assert_eq!(Month(0).start(), chrono::NaiveDate::from_ymd(0, 1, 1));
        assert_eq!(Month(-1).start(), chrono::NaiveDate::from_ymd(-1, 12, 1));
        assert_eq!(Month(-2).start(), chrono::NaiveDate::from_ymd(-1, 11, 1));
        assert_eq!(Month(-15).start(), chrono::NaiveDate::from_ymd(-2, 10, 1));
    }
}
