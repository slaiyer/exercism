use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = self.minutes + minutes;
        let (add_hours, new_minutes) = (new_minutes / 60, new_minutes % 60);
        let new_hours = (self.hours + add_hours) % 12;

        Self {
            hours: new_hours,
            minutes: new_minutes,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
