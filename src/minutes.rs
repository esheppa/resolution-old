use crate::TimeResolution;
use serde::{
    de,
    ser::{self, SerializeStruct},
};
use std::{cmp, fmt};

const NUM_SECS: i64 = 60;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Minutes<const N: u32> {
    index: i64,
}

#[derive(Clone, Copy, Debug)]
pub struct MinutesZ<Z: chrono::TimeZone, const N: u32> {
    index: i64,
    zone: Z,
}

impl<Z: chrono::TimeZone, const N: u32> PartialEq for MinutesZ<Z, N> {
    fn eq(&self, other: &MinutesZ<Z, N>) -> bool {
        todo!()
    }
}
impl<Z: chrono::TimeZone, const N: u32> Eq for MinutesZ<Z, N> {
}
impl<Z: chrono::TimeZone, const N: u32> PartialOrd for MinutesZ<Z, N> {
    fn partial_cmp(&self, other: &MinutesZ<Z, N>) -> Option<cmp::Ordering> {
        todo!()
    }
}
impl<Z: chrono::TimeZone, const N: u32> Ord for MinutesZ<Z, N> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        todo!()
    }
}

/*
impl<Z: chrono::TimeZone, const N: u32> crate::TimeResolution for MinutesZ<Z, N> {
}
impl<Z: chrono::TimeZone, const N: u32> crate::TimeResolutionZone<Z> for MinutesZ<Z, N> {
}
*/


#[derive(Clone, Copy, Debug)]
pub struct MinutesTZ<const N: u32> {
    index: i64,
    zone: chrono_tz::Tz,
}

impl<const N: u32> PartialEq for MinutesTZ<N> {
    fn eq(&self, other: &MinutesTZ<N>) -> bool {
        todo!()
    }
}
impl<const N: u32> Eq for MinutesTZ<N> {
}
impl<const N: u32> PartialOrd for MinutesTZ<N> {
    fn partial_cmp(&self, other: &MinutesTZ<N>) -> Option<cmp::Ordering> {
        todo!()
    }
}
impl<const N: u32> Ord for MinutesTZ<N> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        todo!()
    }
}


impl<const N: u32> fmt::Display for Minutes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if N == 1 {
            write!(f, "{}", self.naive_date_time())
        } else {
            write!(f, "{} - {}", self.naive_date_time(), self.succ().naive_date_time())
        }
    }
}

impl<const N: u32> crate::TimeResolution for Minutes<N> {
    fn between(&self, other: Self) -> i64 {
        other.index - self.index
    }
    fn succ_n(&self, n: u32) -> Minutes<N> {
        Minutes { index: self.index + i64::from(n)}
    }
    fn pred_n(&self, n: u32) -> Minutes<N> {
        Minutes { index: self.index - i64::from(n)}
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
        format!("Minutes[{}]", N)
    }
}

impl<const N: u32> Minutes<N> {}

impl<const N: u32> crate::SubDateResolution for Minutes<N> {
    fn occurs_on_date(&self) -> chrono::NaiveDate {
        self.naive_date_time().date()
    }
    fn first_on_day(day: chrono::NaiveDate) -> Self {
        Self::from_monotonic(day.and_hms(0, 0, 0).timestamp() / (i64::from(N)*NUM_SECS))
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
