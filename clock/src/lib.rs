use std::fmt;
use time::{Date, Duration, PrimitiveDateTime, Time};

pub struct Clock {
    time: PrimitiveDateTime,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let initial_time = PrimitiveDateTime::new(
            Date::from_ordinal_date(1990, 1).unwrap(),
            Time::from_hms(0, 0, 0).unwrap(),
        ) + Duration::hours(hours.try_into().unwrap())
            + Duration::minutes(minutes.try_into().unwrap());
        Clock { time: initial_time }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.time = self.time + Duration::minutes(minutes.try_into().unwrap());

        self
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.time.hour() == other.time.hour() && self.time.minute() == other.time.minute()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            format!("{:0>2}", self.time.hour()),
            format!("{:0>2}", self.time.minute()),
        )
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("hours", &self.time.hour())
            .field("minutes", &self.time.minute())
            .finish()
    }
}
