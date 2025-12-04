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


fn main() {
    println!("Hello, world!");
}
