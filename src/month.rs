use crate::DateResolution;
use chrono::Datelike;
#[cfg(with_serde)]
use serde::de;
use std::{convert::TryFrom, fmt, str};

const DATE_FORMAT: &str = "%b-%Y";

#[cfg(with_serde)]
impl<'de> de::Deserialize<'de> for Month {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Month, D::Error>
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
impl serde::Serialize for Month {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

fn month_num_from_name(name: &str) -> Result<u32, crate::Error> {
    let num = match name {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        n => {
            return Err(crate::Error::ParseCustom {
                ty_name: "Month",
                input: format!("Unknown month name `{}`", n),
            })
        }
    };
    Ok(num)
}

impl str::FromStr for Month {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let month =
            month_num_from_name(split.next().ok_or_else(|| crate::Error::ParseCustom {
                ty_name: "Month",
                input: s.to_string(),
            })?)?;
        let year = split
            .next()
            .ok_or_else(|| crate::Error::ParseCustom {
                ty_name: "Month",
                input: s.to_string(),
            })?
            .parse()?;
        let date = chrono::NaiveDate::from_ymd(year, month, 1);
        Ok(date.into())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Month(i64); // number of months +- since 0AD

impl crate::TimeResolution for Month {
    fn between(&self, other: Self) -> i64 {
        other.0 - self.0
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
    fn start(&self) -> chrono::NaiveDate {
        let years = i32::try_from(self.0.div_euclid(12)).expect("Not pre/post historic");
        let months = u32::try_from(1 + self.0.rem_euclid(12)).unwrap();
        chrono::NaiveDate::from_ymd(years, months, 1)
    }
}

impl From<chrono::NaiveDate> for Month {
    fn from(d: chrono::NaiveDate) -> Self {
        Month(i64::from(d.month0()) + i64::from(d.year()) * 12)
    }
}

impl From<chrono::NaiveDateTime> for Month {
    fn from(d: chrono::NaiveDateTime) -> Self {
        d.date().into()
    }
}

impl Month {
    pub fn year(&self) -> crate::Year {
        self.start().into()
    }
    pub fn quarter(&self) -> crate::Quarter {
        self.start().into()
    }
    pub fn year_num(&self) -> i32 {
        self.start().year()
    }
    pub fn month_num(&self) -> u32 {
        self.start().month()
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.start().format(DATE_FORMAT))
    }
}

#[cfg(test)]
mod tests {
    use super::Month;
    use crate::{DateResolution, DateResolutionExt, TimeResolution};

    #[test]
    fn test_roundtrip() {
        let dt = chrono::NaiveDate::from_ymd(2021, 12, 6);

        let wk = Month::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);

        let dt = chrono::NaiveDate::from_ymd(2019, 7, 1);

        let m2 = Month::from(dt);

        assert!(m2.start() == dt)
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            "Jan-2021".parse::<Month>().unwrap().start(),
            chrono::NaiveDate::from_ymd(2021, 1, 1),
        );
        assert_eq!(
            "Jan-2021".parse::<Month>().unwrap().succ().start(),
            chrono::NaiveDate::from_ymd(2021, 2, 1),
        );
        assert_eq!(
            "Jan-2021".parse::<Month>().unwrap().succ().pred().start(),
            chrono::NaiveDate::from_ymd(2021, 1, 1),
        );
    }

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
