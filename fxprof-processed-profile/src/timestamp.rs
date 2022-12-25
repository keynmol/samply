use serde::ser::{Serialize, Serializer};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Timestamp {
    nanos: u64,
}

impl Timestamp {
    pub fn from_nanos_since_reference(nanos: u64) -> Self {
        Self { nanos }
    }

    pub fn from_millis_since_reference(millis: f64) -> Self {
        Self {
            nanos: (millis * 1_000_000.0) as u64,
        }
    }

    pub fn from_duration_since_reference(duration: Duration) -> Self {
        Self {
            nanos: duration.as_nanos() as u64,
        }
    }
}

impl Serialize for Timestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // In the profile JSON, timestamps are currently expressed as float milliseconds
        // since profile.meta.startTime.
        serializer.serialize_f64((self.nanos as f64) / 1_000_000.0)
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct InstantTimestampMaker {
    reference_instant: Instant,
}

impl From<Instant> for InstantTimestampMaker {
    fn from(instant: Instant) -> Self {
        Self {
            reference_instant: instant,
        }
    }
}

impl InstantTimestampMaker {
    pub fn make_ts(&self, instant: Instant) -> Timestamp {
        Timestamp::from_duration_since_reference(
            instant.saturating_duration_since(self.reference_instant),
        )
    }
}
