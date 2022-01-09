use crate::{TimeResolution, TimeResolutionZone};
use serde::{de, ser};
use std::{cmp, fmt, hash, str};

const NUM_SECS: i64 = 60;

const PARSE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Clone, Copy, Debug)]
pub struct MinutesZ<Z, const N: u32>
where
    Z: crate::TimeZone,
{
    index: i64,
    zone: Z,
}

impl<Z, const N: u32> From<chrono::NaiveDateTime> for MinutesZ<Z, N>
where
    Z: crate::TimeZone,
{
    fn from(d: chrono::NaiveDateTime) -> Self {
        todo!()
    }
}

impl<Z, const N: u32> fmt::Display for MinutesZ<Z, N>
where
    Z: crate::TimeZone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if N == 1 {
            write!(f, "{} {}", self.naive_date_time(), self.zone,)
        } else {
            write!(
                f,
                "{} - {} {}",
                self.naive_date_time(),
                self.succ().naive_date_time(),
                self.zone,
            )
        }
    }
}

impl<Z, const N: u32> hash::Hash for MinutesZ<Z, N>
where
    Z: crate::TimeZone,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.zone.hash(state);
    }
}

impl<Z, const N: u32> PartialEq for MinutesZ<Z, N>
where
    Z: crate::TimeZone,
{
    fn eq(&self, other: &MinutesZ<Z, N>) -> bool {
        self.start_date_time() == other.start_date_time()
    }
}
impl<Z, const N: u32> Eq for MinutesZ<Z, N> where Z: crate::TimeZone {}

impl<Z, const N: u32> PartialOrd for MinutesZ<Z, N>
where
    Z: crate::TimeZone,
{
    fn partial_cmp(&self, other: &MinutesZ<Z, N>) -> Option<cmp::Ordering> {
        self.start_date_time().partial_cmp(&other.start_date_time())
    }
}
impl<Z, const N: u32> Ord for MinutesZ<Z, N>
where
    Z: crate::TimeZone,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.start_date_time().cmp(&other.start_date_time())
    }
}

impl<Z, const N: u32> crate::TimeResolution for MinutesZ<Z, N>
where
    Z: crate::TimeZone,
{
    fn between(&self, other: Self) -> i64 {
        other.index - self.index
    }
    fn succ_n(&self, n: u32) -> MinutesZ<Z, N> {
        MinutesZ {
            index: self.index + i64::from(n),
            zone: self.zone,
        }
    }
    fn pred_n(&self, n: u32) -> MinutesZ<Z, N> {
        MinutesZ {
            index: self.index - i64::from(n),
            zone: self.zone,
        }
    }
    fn naive_date_time(&self) -> chrono::NaiveDateTime {
        self.start_date_time().naive_local()
    }
    fn to_monotonic(&self) -> i64 {
        self.index
    }
    fn from_monotonic(index: i64) -> Self {
        MinutesZ {
            index,
            zone: Z::init(),
        }
    }
    fn name(&self) -> String {
        format!("MinutesZ[Length:{},Zone:{}]", N, self.zone)
    }
}
impl<Z, const N: u32> crate::TimeResolutionZone<Z> for MinutesZ<Z, N>
where
    Z: crate::TimeZone,
{
    fn start_date_time(&self) -> chrono::DateTime<Z::ChronoZone> {
        todo!()
    }
    fn from_date_time(time: chrono::DateTime<Z::ChronoZone>) -> Self {
        todo!()
    }
    fn get_zone(&self) -> Z {
        self.zone
    }
}

impl<Z, const N: u32> MinutesZ<Z, N> where Z: crate::TimeZone {}

impl<Z, const N: u32> crate::SubDateResolution for MinutesZ<Z, N>
where
    Z: crate::TimeZone,
{
    fn occurs_on_date(&self) -> chrono::NaiveDate {
        self.naive_date_time().date()
    }
    fn first_on_day(day: chrono::NaiveDate) -> Self {
        Self::from_monotonic(day.and_hms(0, 0, 0).timestamp() / (i64::from(N) * NUM_SECS))
    }
}

impl<'de, Z, const N: u32> serde::Deserialize<'de> for MinutesZ<Z, N>
where
    Z: crate::TimeZone,
{
    fn deserialize<D>(deserializer: D) -> Result<MinutesZ<Z, N>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        todo!()
    }
}

impl<Z, const N: u32> serde::Serialize for MinutesZ<Z, N>
where
    Z: crate::TimeZone,
{
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: ser::Serializer,
    {
        todo!()
    }
}

/// Note that for sensible behaviour, the N chosen should be a number that either:
/// 1. divides into an hour with no remainder (1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60)
/// 2. is exactly a whole number of hours that divides into a day with no remainder (60, 120, 180, 240, 360, 480, 1800)
/// Any other choice will result in unexpected / unuseful behaviour (eg the `Minutes` not cleanly fitting into parts of a day)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Minutes<const N: u32> {
    index: i64,
}

impl<const N: u32> From<chrono::NaiveDateTime> for Minutes<N> {
    fn from(d: chrono::NaiveDateTime) -> Self {
        Minutes {
            index: d.timestamp().div_euclid(60 * i64::from(N)),
        }
    }
}

impl<const N: u32> str::FromStr for Minutes<N> {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if N == 1 {
            let time = chrono::NaiveDateTime::parse_from_str(s, PARSE_FORMAT)?;

            Ok(time.into())
        } else {
            let mut splits = s.split("=>");

            let start = splits.next().ok_or_else(|| crate::Error::ParseCustom {
                ty_name: "Minutes",
                input: s.into(),
            })?;

            let end = splits.next().ok_or_else(|| crate::Error::ParseCustom {
                ty_name: "Minutes",
                input: s.into(),
            })?;

            let start = chrono::NaiveDateTime::parse_from_str(start, PARSE_FORMAT)?;

            let end = chrono::NaiveDateTime::parse_from_str(end, PARSE_FORMAT)?;

            if (start - end).num_minutes() != N.into() {
                return Err(crate::Error::ParseCustom {
                    ty_name: "Minutes",
                    input: format!(
                        "Invalid start-end combination for Minutes[Length:{}]: {}",
                        N, s
                    ),
                });
            }

            Ok(start.into())
        }
    }
}

impl<const N: u32> fmt::Display for Minutes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if N == 1 {
            write!(f, "{}", self.naive_date_time())
        } else {
            write!(
                f,
                "{}=>{}",
                self.naive_date_time(),
                self.succ().naive_date_time()
            )
        }
    }
}

impl<const N: u32> crate::TimeResolution for Minutes<N> {
    fn between(&self, other: Self) -> i64 {
        other.index - self.index
    }
    fn succ_n(&self, n: u32) -> Minutes<N> {
        Minutes {
            index: self.index + i64::from(n),
        }
    }
    fn pred_n(&self, n: u32) -> Minutes<N> {
        Minutes {
            index: self.index - i64::from(n),
        }
    }
    fn naive_date_time(&self) -> chrono::NaiveDateTime {
        chrono::NaiveDateTime::from_timestamp(self.index * NUM_SECS * i64::from(N), 0)
    }
    fn to_monotonic(&self) -> i64 {
        self.index
    }
    fn from_monotonic(index: i64) -> Self {
        Minutes { index }
    }
    fn name(&self) -> String {
        format!("Minutes[Length:{}]", N)
    }
}

impl<const N: u32> Minutes<N> {}

impl<const N: u32> crate::SubDateResolution for Minutes<N> {
    fn occurs_on_date(&self) -> chrono::NaiveDate {
        self.naive_date_time().date()
    }
    fn first_on_day(day: chrono::NaiveDate) -> Self {
        Self::from_monotonic(day.and_hms(0, 0, 0).timestamp() / (i64::from(N) * NUM_SECS))
    }
}

impl<'de, const N: u32> serde::Deserialize<'de> for Minutes<N> {
    fn deserialize<D>(deserializer: D) -> Result<Minutes<N>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        todo!()
    }
}

impl<const N: u32> serde::Serialize for Minutes<N> {
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: ser::Serializer,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SubDateResolution, TimeResolution};

    #[test]
    fn test_roundtrip() {
        let dt = chrono::NaiveDate::from_ymd(2021, 12, 6);
        let tm = dt.and_hms(0, 0, 0);

        let min = Minutes::<1>::from(tm);
        assert!(min.occurs_on_date() == dt);
        assert!(min.naive_date_time() == tm);

        let min = Minutes::<2>::from(tm);
        assert!(min.occurs_on_date() == dt);
        assert!(min.naive_date_time() == tm);

        let min = Minutes::<3>::from(tm);
        assert!(min.occurs_on_date() == dt);
        assert!(min.naive_date_time() == tm);

        let min = Minutes::<4>::from(tm);
        assert!(min.occurs_on_date() == dt);
        assert!(min.naive_date_time() == tm);

        let min = Minutes::<5>::from(tm);
        assert!(min.occurs_on_date() == dt);
        assert!(min.naive_date_time() == tm);
    }

    // #[test]
    // fn test_parse() {
    //     assert_eq!(
    //         "2021-01-01".parse::<Day>().unwrap().start(),
    //         chrono::NaiveDate::from_ymd(2021, 1, 1),
    //     );
    //     assert_eq!(
    //         "2021-01-01".parse::<Day>().unwrap().succ().start(),
    //         chrono::NaiveDate::from_ymd(2021, 1, 2),
    //     );
    //     assert_eq!(
    //         "2021-01-01"
    //             .parse::<Day>()
    //             .unwrap()
    //             .succ()
    //             .pred()
    //             .start(),
    //         chrono::NaiveDate::from_ymd(2021, 1, 1),
    //     );
    // }
}
