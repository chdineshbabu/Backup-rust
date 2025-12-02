use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total = hours * 60 + minutes;
        Clock {
            minutes: Self::normalize(total),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: Self::normalize(self.minutes + minutes),
        }
    }

    fn normalize(mins: i32) -> i32 {
        const MINUTES_PER_DAY: i32 = 24 * 60;
        let mut m = mins % MINUTES_PER_DAY;
        if m < 0 {
            m += MINUTES_PER_DAY;
        }
        m
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hh = self.minutes / 60;
        let mm = self.minutes % 60;
        write!(f, "{hh:02}:{mm:02}")
    }
}
