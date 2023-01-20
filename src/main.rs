extern crate core;

mod rbe;
mod exercism;
use std::time::Duration;

fn main() {
    println!("{}", exercism::gigaseconds::after(Duration::from_secs(1)).as_secs())
}