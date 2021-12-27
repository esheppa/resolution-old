use std::{fmt, hash, marker, str};

use chrono::Datelike;

mod private {
    pub trait Sealed {}
    impl Sealed for super::Monday {}
    impl Sealed for super::Tuesday {}
    impl Sealed for super::Wednesday {}
    impl Sealed for super::Thursday {}
    impl Sealed for super::Friday {}
    impl Sealed for super::Saturday {}
    impl Sealed for super::Sunday {}
}

pub trait StartDay:
    private::Sealed
    + Send
    + Sync
    + 'static
    + Copy
    + Clone
    + fmt::Debug
    + hash::Hash
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
{
    const NAME: &'static str;
    fn weekday() -> chrono::Weekday;
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Monday;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tuesday;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Wednesday;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Thursday;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Friday;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Saturday;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sunday;

impl StartDay for Monday {
    const NAME: &'static str = "Monday";
    fn weekday() -> chrono::Weekday {
        chrono::Weekday::Mon
    }
}
impl StartDay for Tuesday {
    const NAME: &'static str = "Tuesday";
    fn weekday() -> chrono::Weekday {
        chrono::Weekday::Tue
    }
}
impl StartDay for Wednesday {
    const NAME: &'static str = "Wednesday";
    fn weekday() -> chrono::Weekday {
        chrono::Weekday::Wed
    }
}
impl StartDay for Thursday {
    const NAME: &'static str = "Thursday";
    fn weekday() -> chrono::Weekday {
        chrono::Weekday::Thu
    }
}
impl StartDay for Friday {
    const NAME: &'static str = "Friday";
    fn weekday() -> chrono::Weekday {
        chrono::Weekday::Fri
    }
}
impl StartDay for Saturday {
    const NAME: &'static str = "Saturday";
    fn weekday() -> chrono::Weekday {
        chrono::Weekday::Sat
    }
}
impl StartDay for Sunday {
    const NAME: &'static str = "Sunday";
    fn weekday() -> chrono::Weekday {
        chrono::Weekday::Sun
    }
}

#[derive(
    Clone, Copy, Debug, Eq, PartialOrd, PartialEq, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
#[serde(try_from = "Week_", into = "Week_")]
pub struct Week<D: StartDay> {
    n: i64,
    d: marker::PhantomData<D>,
}

impl<D: StartDay> TryFrom<Week_> for Week<D> {
    type Error = String;
    fn try_from(value: Week_) -> Result<Self, Self::Error> {
        if value.start_day == D::NAME {
            Ok(Week::new(value.n))
        } else {
            Err(format!(
                "To create a Week<{}>, the start_day field should be {} but was instead {}",
                D::NAME,
                D::NAME,
                value.start_day
            ))
        }
    }
}

impl<D: StartDay> From<Week<D>> for Week_ {
    fn from(w: Week<D>) -> Self {
        Week_ {
            n: w.n,
            start_day: D::NAME.to_string(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Week_ {
    n: i64,
    start_day: String,
}

impl<D: StartDay> fmt::Display for Week<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Week starting {}", crate::DateResolution::start(self))
    }
}

fn base(wd: chrono::Weekday) -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd(2021, 1, 4 + wd.num_days_from_monday())
}

impl<D: StartDay> Week<D> {
    fn new(num: i64) -> Week<D> {
        Week {
            n: num,
            d: marker::PhantomData,
        }
    }
}

impl<D: StartDay> str::FromStr for Week<D> {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 24 {
            return Err(crate::Error::UnexpectedInputLength {
                actual: s.len(),
                required: 24,
                format: "Week starting %Y-%m-%d",
            });
        }
        let date = chrono::NaiveDate::parse_from_str(&s[14..24], "%Y-%m-%d")?;
        match (date.weekday(), D::NAME) {
            (chrono::Weekday::Mon, "Monday") => {}
            (chrono::Weekday::Tue, "Tuesday") => {}
            (chrono::Weekday::Wed, "Wednesday") => {}
            (chrono::Weekday::Thu, "Thursday") => {}
            (chrono::Weekday::Fri, "Friday") => {}
            (chrono::Weekday::Sat, "Saturday") => {}
            (chrono::Weekday::Sun, "Sunday") => {}
            (parsed_day, required_day) => {
                return Err(crate::Error::UnexpectedStartDate {
                    date,
                    actual: parsed_day,
                    required: required_day,
                })
            }
        };

        let week_num = (date - base(date.weekday())).num_days() / 7;

        Ok(Week::new(week_num))
    }
}

impl<D: StartDay> crate::DateResolution for Week<D> {
    fn start(&self) -> chrono::NaiveDate {
        base(D::weekday()) + chrono::Duration::days(self.n * 7)
    }
}

impl<D: StartDay> crate::TimeResolution for Week<D> {
    fn between(&self, other: Self) -> i64 {
        i64::from(other.n - self.n)
    }
    fn succ_n(&self, n: u32) -> Week<D> {
        Week::new(self.n + i64::from(n))
    }
    fn pred_n(&self, n: u32) -> Week<D> {
        Week::new(self.n - i64::from(n))
    }
    fn naive_date_time(&self) -> chrono::NaiveDateTime {
        crate::DateResolution::start(self).and_hms(0, 0, 0)
    }
    fn from_monotonic(idx: i64) -> Self {
        Week::new(idx)
    }
    fn to_monotonic(&self) -> i64 {
        self.n
    }
    fn name(&self) -> String {
        format!("Week[StartDay:{}]", D::NAME)
    }
}


impl<D: StartDay> From<chrono::NaiveDate> for Week<D> {
    fn from(d: chrono::NaiveDate) -> Self {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DateResolution, TimeResolution};

    #[test]
    fn test_parse() {
        assert_eq!(
            "Week starting 2021-12-06"
                .parse::<Week<Monday>>()
                .unwrap()
                .start(),
            chrono::NaiveDate::from_ymd(2021, 12, 6),
        );
        assert_eq!(
            "Week starting 2021-12-06"
                .parse::<Week<Monday>>()
                .unwrap()
                .succ()
                .start(),
            chrono::NaiveDate::from_ymd(2021, 12, 13),
        );
        assert_eq!(
            "Week starting 2021-12-06"
                .parse::<Week<Monday>>()
                .unwrap()
                .succ()
                .pred()
                .start(),
            chrono::NaiveDate::from_ymd(2021, 12, 06),
        );

        assert!("Week starting 2021-12-06".parse::<Week<Tuesday>>().is_err(),);
        assert!("Week starting 2021-12-06"
            .parse::<Week<Wednesday>>()
            .is_err(),);
        assert!("Week starting 2021-12-06"
            .parse::<Week<Thursday>>()
            .is_err(),);
        assert!("Week starting 2021-12-06".parse::<Week<Friday>>().is_err(),);
        assert!("Week starting 2021-12-06"
            .parse::<Week<Saturday>>()
            .is_err(),);
        assert!("Week starting 2021-12-06".parse::<Week<Sunday>>().is_err(),);
    }
}
