use std::fmt;

const MINS_HOUR: i64 = 60;
const MINS_DAY: i64 = 24 * MINS_HOUR;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        Self {
            minutes: (((hours * MINS_HOUR + minutes) % MINS_DAY) + MINS_DAY) % MINS_DAY,
        }
    }

    pub fn add_minutes(self, minutes: i64) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MINS_HOUR,
            self.minutes % MINS_HOUR
        )
    }
}
