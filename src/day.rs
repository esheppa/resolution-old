use crate::DateResolution;
use chrono::Datelike;
#[cfg(with_serde)]
use serde::de;
use std::{fmt, str};

const DATE_FORMAT: &str = "%Y-%m-%d";

#[cfg(with_serde)]
impl<'de> de::Deserialize<'de> for Day {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Day, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date =
            chrono::NaiveDate::parse_from_str(&s, DATE_FORMAT).map_err(serde::de::Error::custom)?;
        Ok(date.into())
    }
}

#[cfg(with_serde)]
impl serde::Serialize for Day {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Day(i64);

fn base() -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd(0, 1, 1)
}

impl str::FromStr for Day {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = chrono::NaiveDate::parse_from_str(s, DATE_FORMAT)?;
        Ok(date.into())
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.start())
    }
}

impl crate::DateResolution for Day {
    fn start(&self) -> chrono::NaiveDate {
        base() + chrono::Duration::days(self.0)
    }
}

impl From<chrono::NaiveDate> for Day {
    fn from(d: chrono::NaiveDate) -> Day {
        Day((d - base()).num_days())
    }
}

impl From<chrono::NaiveDateTime> for Day {
    fn from(d: chrono::NaiveDateTime) -> Self {
        d.date().into()
    }
}

impl crate::TimeResolution for Day {
    fn between(&self, other: Self) -> i64 {
        other.0 - self.0
    }
    fn succ_n(&self, n: u32) -> Day {
        Day(self.0 + i64::from(n))
    }
    fn pred_n(&self, n: u32) -> Day {
        Day(self.0 - i64::from(n))
    }
    fn naive_date_time(&self) -> chrono::NaiveDateTime {
        self.start().and_hms(0, 0, 0)
    }
    fn to_monotonic(&self) -> i64 {
        self.0
    }
    fn from_monotonic(idx: i64) -> Self {
        Day(idx)
    }
    fn name(&self) -> String {
        "Day".to_string()
    }
}

impl Day {
    pub fn year(&self) -> crate::Year {
        self.start().into()
    }
    pub fn quarter(&self) -> crate::Quarter {
        self.start().into()
    }
    pub fn month(&self) -> crate::Month {
        self.start().into()
    }
    pub fn week<D: crate::StartDay>(&self) -> crate::Week<D> {
        self.start().into()
    }
    pub fn year_num(&self) -> i32 {
        self.start().year()
    }
    pub fn month_num(&self) -> u32 {
        self.start().month()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DateResolution, DateResolutionExt, TimeResolution};

    #[test]
    fn test_roundtrip() {
        let dt = chrono::NaiveDate::from_ymd(2021, 12, 6);

        let wk = Day::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);
    }

    #[test]
    fn test_parse_date_syntax() {
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap().start(),
            chrono::NaiveDate::from_ymd(2021, 1, 1),
        );
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap().succ().start(),
            chrono::NaiveDate::from_ymd(2021, 1, 2),
        );
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap().succ().pred().start(),
            chrono::NaiveDate::from_ymd(2021, 1, 1),
        );
    }

    #[test]
    fn test_start() {
        assert_eq!(Day(2).start(), chrono::NaiveDate::from_ymd(0, 1, 3));
        assert_eq!(Day(1).start(), chrono::NaiveDate::from_ymd(0, 1, 2));
        assert_eq!(Day(0).start(), chrono::NaiveDate::from_ymd(0, 1, 1));
        assert_eq!(Day(-1).start(), chrono::NaiveDate::from_ymd(-1, 12, 31));
        assert_eq!(Day(-2).start(), chrono::NaiveDate::from_ymd(-1, 12, 30));
    }
}
