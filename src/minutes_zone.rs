use crate::{TimeResolution, TimeResolutionZone};
#[cfg(with_serde)]
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

#[cfg(with_serde)]
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

#[cfg(with_serde)]
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
