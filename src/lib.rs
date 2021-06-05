use any::TypeId;
use chrono::Datelike;
use serde::{
    de,
    ser::{self, SerializeStruct},
};
use std::{any, num, collections, convert::TryFrom, fmt, result};

mod minutes; 
pub use minutes::Minutes;

pub type Minute = Minutes<1>;
pub type FiveMinute = Minutes<5>;
pub type HalfHour = Minutes<30>;
pub type Hour = Minutes<60>;

mod date;
pub use date::Date;
mod month;
pub use month::Month;
mod quarter;
pub use quarter::Quarter;
mod year;
pub use year::Year;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Got new data for {point}: {new} different from data already in the cache {old}")]
    GotNonMatchingNewData {
        point: String,
        old: String,
        new: String,
    },
    #[error("Error parsing int: {0}")]
    ParseInt(#[from] num::ParseIntError),
    #[error("Error parsing date/time: {0}")]
    ParseDate(#[from] chrono::ParseError),
    #[error("Error parsing {ty_name} from input: {input}")]
    ParseCustom { ty_name: &'static str, input: String },
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait TimeResolutionZone<Z: chrono::TimeZone>: TimeResolution 
{
    fn date_time(&self) -> chrono::DateTime<Z>;
    fn get_zone() -> Z;
}

pub trait TimeResolution:
    Send
    + Sync
    + Clone
    + Copy
    + fmt::Debug
    + fmt::Display
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Sized
    + serde::Serialize
    + de::DeserializeOwned
{
    fn succ(&self) -> Self {
        self.succ_n(1)
    }
    fn pred(&self) -> Self {
        self.pred_n(1)
    }

    // we choose i64 rather than u64
    // as the behaviour on subtraction is nicer!
    fn to_monotonic(&self) -> i64;
    fn from_monotonic(idx: i64) -> Self;

    // the default impls are probably inefficient
    // makes sense to require just the n
    // and give the 1 for free
    fn succ_n(&self, n: u32) -> Self;
    fn pred_n(&self, n: u32) -> Self;

    fn between(&self, other: Self) -> i64;

    fn naive_date_time(&self) -> chrono::NaiveDateTime;
}

// This trait exists to be able to provide a trait
// bound for resolutions that are less than one day long
pub trait SubDateResolution: TimeResolution {
    fn occurs_on_date(&self) -> chrono::NaiveDate;
    // the first of the resolutions units that occurs on the day
    fn first_on_day(day: chrono::NaiveDate) -> Self;
    fn last_on_day(day: chrono::NaiveDate) -> Self {
        Self::first_on_day(day + chrono::Duration::days(1)).pred()
    }
}

// This trait exists to be able to provide a trait
// bound for resolutiopns that are one day long or longer.
// Due to this it can have a number of useful methods
pub trait DateResolution: TimeResolution {
    fn start(&self) -> chrono::NaiveDate;

    // free
    fn format<'a>(
        &self,
        fmt: &'a str,
    ) -> chrono::format::DelayedFormat<chrono::format::StrftimeItems<'a>> {
        self.start().format(fmt)
    }
    fn end(&self) -> chrono::NaiveDate {
        self.succ().start() - chrono::Duration::days(1)
    }
    fn num_days(&self) -> i64 {
        (self.end() - self.start()).num_days() + 1
    }
    fn to_sub_date_resolution<R: SubDateResolution>(&self) -> TimeRange<R> {
        TimeRange::from_start_end(R::first_on_day(self.start()), R::last_on_day(self.end())).expect("Will always have at least one within the day")
    }
    fn days(&self) -> collections::BTreeSet<chrono::NaiveDate> {
        (0..)
            .map(|n| self.start() + chrono::Duration::days(n))
            .filter(|d| d <= &self.end())
            .collect()
    }
    fn business_days(
        &self,
        weekend: collections::HashSet<chrono::Weekday>,
        holidays: collections::BTreeSet<chrono::NaiveDate>,
    ) -> collections::BTreeSet<chrono::NaiveDate> {
        let base_days = (0..)
            .map(|n| self.start() + chrono::Duration::days(n))
            .filter(|d| d <= &self.end())
            .filter(|d| !weekend.contains(&d.weekday()))
            .collect::<collections::BTreeSet<_>>();
        base_days.difference(&holidays).copied().collect()
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TimeRange<P: TimeResolution> {
    #[serde(bound(deserialize = "P: de::DeserializeOwned"))]
    start: P,
    len: u32,
}

pub trait AsDateRange {
    fn as_date_range(&self) -> TimeRange<Date>;
}
pub trait AsMonthRange {
    fn as_month_range(&self) -> TimeRange<Month>;
}
pub trait AsQuarterRange {
    fn as_quarter_range(&self) -> TimeRange<Quarter>;
}

impl AsDateRange for Quarter {
    fn as_date_range(&self) -> TimeRange<Date> {
        todo!()
    }
}

impl<D: AsDateRange + TimeResolution> AsDateRange for TimeRange<D> {
    fn as_date_range(&self) -> TimeRange<Date> {
        todo!()
    }
}

pub trait Rescale<Out: DateResolution> {
    fn rescale(&self) -> TimeRange<Out>;
}

impl Rescale<Date> for Quarter {
    fn rescale(&self) -> TimeRange<Date> {
        todo!()
    }
}
impl Rescale<Month> for Quarter {
    fn rescale(&self) -> TimeRange<Month> {
        todo!()
    }
}

//impl<'de, P> serde::Deserialize<'de> for TimeRange<P>
//where
//    P: TimeResolution,
//{
//    fn deserialize<D>(deserializer: D) -> result::Result<TimeRange<P>, D::Error>
//    where
//        D: de::Deserializer<'de>,
//    {
//        todo!()
//    }
//}
//
//impl<P> serde::Serialize for TimeRange<P>
//where
//    P: TimeResolution,
//{
//    fn serialize<SER>(&self, serializer: SER) -> result::Result<SER::Ok, SER::Error>
//    where
//        SER: ser::Serializer,
//    {
//        todo!()
//    }
//}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeRangeComparison {
    Superset,
    Subset,
    Earlier,
    Later,
}

impl<P: SubDateResolution> TimeRange<P> {}

impl<P: DateResolution> TimeRange<P> {
    pub fn to_sub_date_resolution<S: SubDateResolution>(&self) -> TimeRange<S> {
         // get first start 
         let first_start = S::first_on_day(self.start.start());
         // get last end
         let last_end = S::last_on_day(self.end().end());
         // do from_start_end and expect it
         TimeRange::from_start_end(first_start, last_end).expect("Original range is contigious so new will also be contigious")
    }
}


impl<P: TimeResolution> TimeRange<P> {
    // use with the cacheresponse!
    pub fn from_indexes(idx: &[i64]) -> Result<TimeRange<P>> {
        todo!()
    }
    pub fn to_indexes(&self) -> collections::BTreeSet<i64> {
        self.iter().map(|p| p.to_monotonic()).collect()
    }

    pub fn new(start: P, len: u32) -> TimeRange<P> {
        TimeRange { start, len }
    }
    pub fn index_of(&self, point: P) -> Option<usize> {
        if point < self.start || point > self.end() {
            None
        } else { 
            Some(usize::try_from(self.start.between(point)).expect("Point is earlier than end so this is always ok"))
        }
    }
    pub fn from_start_end(start: P, end: P) -> Option<TimeRange<P>> {
        if start <= end {
            Some(TimeRange {
                start,
                len: 1 + u32::try_from(start.between(end))
                    .expect("Start is earlier than End so difference is positive"),
            })
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        usize::try_from(self.len).unwrap()
    }

    pub fn intersect(&self, other: TimeRange<P>) -> Option<TimeRange<P>> {
        let max_start = self.start().max(other.start());
        let min_end = self.end().min(other.end());
        TimeRange::from_start_end(max_start, min_end)
    }
    pub fn union(&self, other: TimeRange<P>) -> Option<TimeRange<P>> {
        if let Some(_) = self.intersect(other) {
            let min_start = self.start().min(other.start());
            let max_end = self.end().max(other.end());
            TimeRange::from_start_end(min_start, max_end)
        } else {
            None
        }
    }

    pub fn difference(&self, other: TimeRange<P>) -> (Option<TimeRange<P>>, Option<TimeRange<P>>) {
        todo!()
    }
    pub fn compare(&self, other: TimeRange<P>) -> TimeRangeComparison {
        match self.difference(other) {
            (Some(_), Some(_)) => TimeRangeComparison::Superset,
            (Some(_), None) => TimeRangeComparison::Earlier,
            (None, Some(_)) => TimeRangeComparison::Later,
            (None, None) => TimeRangeComparison::Subset,
        }
    }
    pub fn from_set(set: &collections::BTreeSet<P>) -> Option<TimeRange<P>> {
        if u32::try_from(set.len()).is_err() {
            return None;
        }
        if set.is_empty() {
            return None;
        }
        Some(TimeRange {
            start: set.iter().next().copied()?,
            len: u32::try_from(set.len()).ok()?,
        })
    }
    pub fn start(&self) -> P {
        self.start
    }
    pub fn end(&self) -> P {
        self.start.succ_n(self.len)
    }
    pub fn set(&self) -> collections::BTreeSet<P> {
        self.iter().collect()
    }
    pub fn iter(&self) -> TimeRangeIter<P> {
        TimeRangeIter {
            current: self.start(),
            end: self.end(),
        }
    }
}

pub struct TimeRangeIter<P: TimeResolution> {
    current: P,
    end: P,
}

impl<P: TimeResolution> Iterator for TimeRangeIter<P> {
    type Item = P;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let ret = self.current.clone();
            self.current = self.current.succ();
            Some(ret)
        } else {
            None
        }
    }
}

pub struct Cache<K: Ord + fmt::Debug + Copy, T: Send + fmt::Debug + Eq + Copy> {
    // The actual data in the cache
    data: collections::BTreeMap<K, T>,
    // The requests for data which has been cached
    requests: collections::BTreeSet<K>,
}

// merge a request into a set of requests, grouping contigious on the way
fn missing_pieces<K: Ord + fmt::Debug + Copy>(
    _request: collections::BTreeSet<K>,
    _requests: &collections::BTreeSet<K>,
) -> Vec<collections::BTreeSet<K>> {
    todo!()
}

// No concept of partial, becuse we will simply request the missing data, then ask the cache again.
pub enum CacheResponse<K: Ord + fmt::Debug + Copy, T: Send + fmt::Debug + Eq + Copy> {
    Hit(collections::BTreeMap<K, T>), // means the whole request as able to be replied, doesn't necessarily mean the whole range of data is filled
    Miss(Vec<collections::BTreeSet<K>>), // will be a minimal reasonable set of time ranges to request from the provider
}

impl<K: Ord + fmt::Debug + Copy, T: Send + fmt::Debug + Eq + Copy> Cache<K, T> {
    pub fn get(&self, request: collections::BTreeSet<K>) -> CacheResponse<K, T> {
        if request.is_empty() {
            CacheResponse::Hit(collections::BTreeMap::new())
        } else if self.requests.is_superset(&request) {
            CacheResponse::Hit(
                self.data
                    .iter()
                    // mustn't be empty othewise we would have returned out of the first arm of the `if`
                    .filter(|(k, _)| request.iter().next().unwrap() <= *k)
                    .filter(|(k, _)| request.iter().rev().next().unwrap() >= *k)
                    .map(|(k, v)| (*k, *v))
                    .collect(),
            )
        } else {
            CacheResponse::Miss(missing_pieces(request, &self.requests))
        }
    }
    pub fn empty() -> Cache<K, T> {
        Cache {
            data: collections::BTreeMap::new(),
            requests: collections::BTreeSet::new(),
        }
    }
    // could also store versioned data, with a DateTIme<Utc> associated with each T at each P?
    // or allow overwriting, etc
    // but this default seems better for now
    pub fn add(
        &mut self,
        mut request_range: collections::BTreeSet<K>,
        data: collections::BTreeMap<K, T>,
    ) {
        self.requests.append(&mut request_range);
        for (point, datum) in data {
            // should we check if the data point already exists?
            // if it does exist, what should we do?
            // for now, ignoring, as otherwise
            // this function would need to be fallible
            self.data.insert(point, datum);
        }
    }
}

