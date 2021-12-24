use crate::{month, year, DateResolution};
use chrono::Datelike;
use serde::{
    de,
    ser::{self, SerializeStruct},
    Deserialize,
};
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

impl Year {
    pub fn first_month(&self) -> month::Month {
        todo!()
    }
    pub fn first_quarter(&self) -> month::Month {
        todo!()
    }
    pub fn year(&self) -> year::Year {
        todo!()
    }
    pub fn year_num(&self) -> i32 {
        i32::try_from(self.0).expect("Not pre/post historic")
    }
    pub fn new(year: i32) -> Self {
        Year(i64::from(year))
    }
    pub fn from_date(d: chrono::NaiveDate) -> Self {
        Year(i64::from(d.year()))
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
