#[cfg(with_serde)]
use serde::de;

use std::{any, fmt, hash, num};

// #TODO
// mod minutes_zone;
// pub use minutes_zone::MinutesZ;

mod minutes;
pub use minutes::Minutes;

pub type Minute = Minutes<1>;
pub type FiveMinute = Minutes<5>;
pub type HalfHour = Minutes<30>;
pub type Hour = Minutes<60>;

mod day;
pub use day::Day;
mod month;
pub use month::Month;
mod quarter;
pub use quarter::Quarter;
mod year;
pub use year::Year;

pub mod range;

mod week;
pub use week::{StartDay, Week};

pub fn format_erased_resolution(
    handle_unknown: fn(any::TypeId, i64) -> String,
    tid: any::TypeId,
    val: i64,
) -> String {
    if tid == any::TypeId::of::<Minute>() {
        format!("Minute:{}", Minute::from_monotonic(val))
    } else if tid == any::TypeId::of::<FiveMinute>() {
        format!("FiveMinute:{}", FiveMinute::from_monotonic(val))
    } else if tid == any::TypeId::of::<HalfHour>() {
        format!("HalfHour:{}", HalfHour::from_monotonic(val))
    } else if tid == any::TypeId::of::<Hour>() {
        format!("Hour:{}", Hour::from_monotonic(val))
    } else if tid == any::TypeId::of::<Day>() {
        format!("Day:{}", Day::from_monotonic(val))
    } else if tid == any::TypeId::of::<Week<week::Monday>>() {
        format!("Week:{}", Week::<week::Monday>::from_monotonic(val))
    } else if tid == any::TypeId::of::<Week<week::Tuesday>>() {
        format!("Week:{}", Week::<week::Tuesday>::from_monotonic(val))
    } else if tid == any::TypeId::of::<Week<week::Wednesday>>() {
        format!("Week:{}", Week::<week::Wednesday>::from_monotonic(val))
    } else if tid == any::TypeId::of::<Week<week::Thursday>>() {
        format!("Week:{}", Week::<week::Thursday>::from_monotonic(val))
    } else if tid == any::TypeId::of::<Week<week::Friday>>() {
        format!("Week:{}", Week::<week::Friday>::from_monotonic(val))
    } else if tid == any::TypeId::of::<Week<week::Saturday>>() {
        format!("Week:{}", Week::<week::Saturday>::from_monotonic(val))
    } else if tid == any::TypeId::of::<Week<week::Sunday>>() {
        format!("Week:{}", Week::<week::Sunday>::from_monotonic(val))
    } else if tid == any::TypeId::of::<Month>() {
        format!("Month:{}", Month::from_monotonic(val))
    } else if tid == any::TypeId::of::<Quarter>() {
        format!("Quarter:{}", Quarter::from_monotonic(val))
    } else if tid == any::TypeId::of::<Year>() {
        format!("Year:{}", Year::from_monotonic(val))
    } else {
        handle_unknown(tid, val)
    }
}

#[derive(Debug)]
pub enum Error {
    GotNonMatchingNewData {
        point: String,
        old: String,
        new: String,
    },
    ParseInt(num::ParseIntError),
    ParseDate(chrono::ParseError),
    ParseCustom {
        ty_name: &'static str,
        input: String,
    },
    EmptyRange,
    UnexpectedStartDate {
        date: chrono::NaiveDate,
        required: chrono::Weekday,
        actual: chrono::Weekday,
    },
    UnexpectedInputLength {
        required: usize,
        actual: usize,
        format: &'static str,
    },
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Error {
        Error::ParseInt(e)
    }
}
impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Error {
        Error::ParseDate(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            GotNonMatchingNewData { point, old, new } => write!(
                f,
                "Got new data for {point}: {new} different from data already in the cache {old}"
            ),
            ParseInt(e) => write!(f, "Error parsing int: {e}"),
            ParseDate(e) => write!(f, "Error parsing date/time: {e}"),
            ParseCustom { ty_name, input } => {
                write!(f, "Error parsing {ty_name} from input: {input}")
            }
            EmptyRange => write!(
                f,
                "Time range cannot be created from an empty set of periods"
            ),
            UnexpectedStartDate {
                date,
                required,
                actual,
            } => write!(
                f,
                "Unexpected input length for date {date}, got {actual} but needed {required}"
            ),
            UnexpectedInputLength {
                required,
                actual,
                format,
            } => write!(
                f,
                "Unexpected input length for format {format}, got {actual} but needed {required}"
            ),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

// this is required so that the trait can work with from_monotonic and to_monotonic. While this restricts
// the options for the user (forces one TZ type to map to one specific offset), this is required for use of
// the from/to_monotonic functions and all the other utility of this library - eg caching.
pub trait TimeZone:
    chrono::TimeZone + hash::Hash + fmt::Debug + fmt::Display + Copy + Clone + Send + Sync
{
    type ChronoZone: chrono::TimeZone;
    fn init() -> Self;
    fn zone(&self) -> Self::ChronoZone;
}

pub trait TimeResolutionZone<Z: TimeZone>: TimeResolution {
    fn start_date_time(&self) -> chrono::DateTime<Z::ChronoZone>;
    fn from_date_time(time: chrono::DateTime<Z::ChronoZone>) -> Self;
    fn get_zone(&self) -> Z;
}

#[cfg(with_serde)]
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
    + hash::Hash
    + Sized
    + serde::Serialize
    + de::DeserializeOwned
    + From<chrono::NaiveDateTime>
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

    fn name(&self) -> String;
}

#[cfg(not(with_serde))]
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
    + hash::Hash
    + Sized
    + From<chrono::NaiveDateTime>
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

    fn name(&self) -> String;
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
pub trait DateResolution: TimeResolution + From<chrono::NaiveDate> {
    fn start(&self) -> chrono::NaiveDate;
}

pub trait DateResolutionExt: DateResolution {
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
    fn to_sub_date_resolution<R: SubDateResolution>(&self) -> range::TimeRange<R> {
        range::TimeRange::from_start_end(R::first_on_day(self.start()), R::last_on_day(self.end()))
            .expect("Will always have at least one within the day")
    }
    fn rescale<R: DateResolution>(&self) -> range::TimeRange<R> {
        range::TimeRange::from_start_end(self.start().into(), self.end().into())
            .expect("Will always have at least one day")
    }
    // fn days(&self) -> collections::BTreeSet<chrono::NaiveDate> {
    //     (0..)
    //         .map(|n| self.start() + chrono::Duration::days(n))
    //         .filter(|d| d <= &self.end())
    //         .collect()
    // }
    // fn business_days(
    //     &self,
    //     weekend: collections::HashSet<chrono::Weekday>,
    //     holidays: collections::BTreeSet<chrono::NaiveDate>,
    // ) -> collections::BTreeSet<chrono::NaiveDate> {
    //     let base_days = (0..)
    //         .map(|n| self.start() + chrono::Duration::days(n))
    //         .filter(|d| d <= &self.end())
    //         .filter(|d| !weekend.contains(&d.weekday()))
    //         .collect::<collections::BTreeSet<_>>();
    //     base_days.difference(&holidays).copied().collect()
    // }
}

impl<T> DateResolutionExt for T where T: DateResolution {}
