use chrono::NaiveDateTime;
use prost_types::Timestamp;

pub trait ToProtoTimestamp {
    fn to_option_proto(self) -> Option<Timestamp>;
}

impl ToProtoTimestamp for NaiveDateTime {
    fn to_option_proto(self) -> Option<Timestamp> {
        Some(Timestamp {
            seconds: self.and_utc().timestamp(),
            nanos: 0,
        })
    }
}
