use std::fmt;

const MINUTES_PER_DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: (hours * 60 + minutes).rem_euclid(MINUTES_PER_DAY),
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock {
            minutes: (self.minutes + minutes).rem_euclid(MINUTES_PER_DAY),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clock_hours: i32 = self.minutes / 60;
        let clock_minutes: i32 = self.minutes % 60;
        write!(f, "{:02}:{:02}", clock_hours, clock_minutes)
    }
}
