use crate::dongle::Dongle;

pub mod dongle;
pub mod information;

fn main() {
    let mut dongle = Dongle::new("COM3".to_string());
    dongle.set_timeout(300);
    println!("{:?}", dongle.wait_for_information());
}
