// Sid A.
// CS523 Rust - This is the main file for the Rust School Scheduler app.
use std::collections::BTreeMap;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Day {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl Day {
    fn short(&self) -> &'static str {
        match self {
            Day::Mon => "Mon",
            Day::Tue => "Tue",
            Day::Wed => "Wed",
            Day::Thu => "Thu",
            Day::Fri => "Fri",
            Day::Sat => "Sat",
            Day::Sun => "Sun",
        }
    }
    fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "mon" | "monday" => Some(Day::Mon),
            "tue" | "tuesday" => Some(Day::Tue),
            "wed" | "wednesday" => Some(Day::Wed),
            "thu" | "thursday" => Some(Day::Thu),
            "fri" | "friday" => Some(Day::Fri),
            "sat" | "saturday" => Some(Day::Sat),
            "sun" | "sunday" => Some(Day::Sun),
            _ => None,
        }
    }
    fn all() -> [Day; 7] {
        [
            Day::Mon,
            Day::Tue,
            Day::Wed,
            Day::Thu,
            Day::Fri,
            Day::Sat,
            Day::Sun,
        ]
    }
}


fn main() {
    println!("Hello, world!");
}
