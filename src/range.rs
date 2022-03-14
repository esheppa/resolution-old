use crate::{DateResolution, DateResolutionExt, SubDateResolution, TimeResolution};
use serde::de;
use std::{collections, fmt, mem, num};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
pub struct TimeRange<P: TimeResolution> {
    #[serde(bound(deserialize = "P: de::DeserializeOwned"))]
    start: P,
    len: num::NonZeroU32,
}

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
        TimeRange::from_start_end(first_start, last_end)
            .expect("Original range is contigious so new will also be contigious")
    }
}

impl<P: TimeResolution> TimeRange<P> {
    pub fn from_map(map: collections::BTreeSet<i64>) -> Option<collections::HashSet<TimeRange<P>>> {
        if map.is_empty() {
            return None;
        }

        let mut iter = map.into_iter();

        let mut prev = iter.next()?;
        let mut current_range = TimeRange {
            start: P::from_monotonic(prev),
            len: num::NonZeroU32::new(1).unwrap(),
        };
        let mut ranges = collections::HashSet::new();
        for val in iter {
            if val == prev + 1 {
                current_range.len =
                    num::NonZeroU32::new(current_range.len.get().saturating_add(1)).unwrap();
            } else {
                let mut old_range = TimeRange {
                    start: P::from_monotonic(val),
                    len: num::NonZeroU32::new(1).unwrap(),
                };
                mem::swap(&mut current_range, &mut old_range);
                ranges.insert(old_range);
            }

            prev = val;
        }

        Some(ranges)
    }
    pub fn to_indexes(&self) -> collections::BTreeSet<i64> {
        self.iter().map(|p| p.to_monotonic()).collect()
    }

    pub fn maybe_new(start: P, len: u32) -> Option<TimeRange<P>> {
        Some(TimeRange {
            start,
            len: num::NonZeroU32::new(len)?,
        })
    }
    pub fn new(start: P, len: num::NonZeroU32) -> TimeRange<P> {
        TimeRange { start, len }
    }
    pub fn index_of(&self, point: P) -> Option<usize> {
        if point < self.start || point > self.end() {
            None
        } else {
            Some(
                usize::try_from(self.start.between(point))
                    .expect("Point is earlier than end so this is always ok"),
            )
        }
    }
    pub fn from_start_end(start: P, end: P) -> Option<TimeRange<P>> {
        if start <= end {
            Some(TimeRange {
                start,
                len: num::NonZeroU32::new(1 + u32::try_from(start.between(end)).ok()?).unwrap(),
            })
        } else {
            None
        }
    }

    pub fn len(&self) -> num::NonZeroU32 {
        self.len
    }

    pub fn intersect(&self, other: &TimeRange<P>) -> Option<TimeRange<P>> {
        let max_start = self.start().max(other.start());
        let min_end = self.end().min(other.end());
        TimeRange::from_start_end(max_start, min_end)
    }
    pub fn union(&self, other: &TimeRange<P>) -> Option<TimeRange<P>> {
        if self.intersect(other).is_some() {
            let min_start = self.start().min(other.start());
            let max_end = self.end().max(other.end());
            TimeRange::from_start_end(min_start, max_end)
        } else {
            None
        }
    }

    pub fn subtract(&self, other: &TimeRange<P>) -> (Option<TimeRange<P>>, Option<TimeRange<P>>) {
        (
            TimeRange::from_start_end(self.start(), other.start().pred().min(self.end())),
            TimeRange::from_start_end(other.end().succ().max(self.start()), self.end()),
        )
    }
    pub fn compare(&self, other: &TimeRange<P>) -> TimeRangeComparison {
        match self.subtract(other) {
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
            len: num::NonZeroU32::new(u32::try_from(set.len()).ok()?)?,
        })
    }
    pub fn start(&self) -> P {
        self.start
    }
    pub fn end(&self) -> P {
        self.start.succ_n(self.len.get())
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
            let ret = self.current;
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
    request: collections::BTreeSet<K>,
    requests: &collections::BTreeSet<K>,
) -> Vec<collections::BTreeSet<K>> {
    let mut to_request = Vec::new();
    let mut current_request = collections::BTreeSet::new();

    // there is a fundamental assumption that `request` is contigious
    // as long as `request` is contigious, each of the returned requests
    // will also be contigious
    // there is no need to worry about filling gaps to reduce the total number
    // of requests - the consumer will handle this
    for requested in request {
        if !requests.contains(&requested) {
            current_request.insert(dbg!(requested));
        } else if !current_request.is_empty() {
            to_request.push(mem::take(&mut current_request));
        }
    }

    if !current_request.is_empty() {
        to_request.push(current_request);
    }

    to_request
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_pieces() {
        let pieces = missing_pieces(
            collections::BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            &collections::BTreeSet::from([2, 3, 7, 8]),
        );
        assert_eq!(
            pieces,
            Vec::from([
                collections::BTreeSet::from([1]),
                collections::BTreeSet::from([4, 5, 6]),
                collections::BTreeSet::from([9, 10]),
            ])
        )
    }
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
