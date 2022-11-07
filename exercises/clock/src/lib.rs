use std::{cmp::PartialEq, fmt::Debug};

#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: (60 * hours + minutes).rem_euclid(24 * 60),
        }
    }

    pub fn to_string(&self) -> String {
        let hours: i32 = &self.minutes / 60;
        let minutes: i32 = &self.minutes % 60;
        return format!("{:02}:{:02}", hours, minutes);
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, &self.minutes + minutes)
    }
}
