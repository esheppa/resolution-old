use crate::{month, year, DateResolution};
use chrono::Datelike;
use serde::{
    de,
    ser::{self, SerializeStruct},
};
use std::{str, cmp, convert::TryFrom, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Quarter(i64);

impl crate::TimeResolution for Quarter {
    fn between(&self, other: Self) -> i64 {
        i64::from(other.0 - self.0)
    }
    fn succ_n(&self, n: u32) -> Self {
        Quarter(self.0 + i64::from(n))
    }
    fn pred_n(&self, n: u32) -> Self {
        Quarter(self.0 - i64::from(n))
    }
    fn naive_date_time(&self) -> chrono::NaiveDateTime {
        self.start().and_hms(0, 0, 0)
    }
    fn from_monotonic(idx: i64) -> Self {
        Quarter(idx)
    }
    fn to_monotonic(&self) -> i64 {
        self.0
    }
}

impl crate::DateResolution for Quarter {
    fn start(&self) -> chrono::NaiveDate {
        let years = i32::try_from(self.0.div_euclid(4)).expect("Not pre/post historic");
        let months = u32::try_from(1 + self.0.rem_euclid(4)).unwrap();
        chrono::NaiveDate::from_ymd(years, months * 3 - 2, 1)
    }
}

impl Quarter {
    pub fn first_month(&self) -> month::Month {
        todo!()
    }
    pub fn year(&self) -> year::Year {
        todo!()
    }
    pub fn year_num(&self) -> i32 {
        self.start().year()
    }
    pub fn quarter_num(&self) -> u32 {
        u32::try_from(self.0.rem_euclid(4)).expect("Range of 1-4")
    }
    pub fn from_date(d: chrono::NaiveDate) -> Self {
        todo!()
    }
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("Q{}-{:4}", self.quarter_num(), self.year_num())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Quarter;
    use crate::DateResolution;

    #[test]
    fn test_start() {
        assert_eq!(Quarter(2).start(), chrono::NaiveDate::from_ymd(0, 7, 1));
        assert_eq!(Quarter(1).start(), chrono::NaiveDate::from_ymd(0, 4, 1));
        assert_eq!(Quarter(0).start(), chrono::NaiveDate::from_ymd(0, 1, 1));
        assert_eq!(Quarter(-1).start(), chrono::NaiveDate::from_ymd(-1, 10, 1));
        assert_eq!(Quarter(-2).start(), chrono::NaiveDate::from_ymd(-1, 7, 1));
    }
}

impl<'de> de::Deserialize<'de> for Quarter 
{
    fn deserialize<D>(
        deserializer: D,
    ) -> std::result::Result<Quarter, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date = chrono::NaiveDate::parse_from_str(&s, "Q%m-%Y")
            .map_err(serde::de::Error::custom)?;
        Ok(Quarter(i64::from(date.year()) * 4 + i64::try_from(date.month()).unwrap()))
    }
}

impl serde::Serialize for Quarter {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl str::FromStr for Quarter {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(parsed) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Ok(Quarter::from_date(parsed))
        } else {
            let split = s.split('-').map(ToString::to_string).collect::<Vec<String>>();
            if split.len() == 2 {
                let qtr = split[0].parse::<u32>()?;
                let year = split[1].parse()?;
                let date = chrono::NaiveDate::from_ymd(year, qtr * 3 - 2, 1);
                Ok(Quarter::from_date(date))
            } else {
                Err(crate::Error::ParseCustom { ty_name: "Quarter", input: s.to_string() })
            }
        }
    }
}

