use std::time::Instant;

use crate::dongle::Dongle;

pub mod dongle;
pub mod information;

fn main() {
    let start = Instant::now();
    let mut dongle = Dongle::new("COM3".to_string());
    println!("{}", dongle.id);
}
