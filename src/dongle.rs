use std::{
    thread,
    time::{Duration, Instant},
};

use serial::SerialPort;

use crate::information::Information;

pub struct Dongle {
    pub port: Box<dyn SerialPort>,
    pub timeout: i32,
}

impl Dongle {
    pub fn new(port: String) -> Self {
        let mut port = serial::open(&port).unwrap();
        port.reconfigure(&|settings| {
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            settings.set_baud_rate(serial::Baud115200)
        })
        .unwrap();

        Self {
            port: Box::new(port),
            timeout: 35,
        }
    }

    pub fn set_timeout(&mut self, timeout: i32) {
        self.timeout = timeout;
    }

    pub fn read_until_terminator(&mut self) -> Result<String, String> {
        let mut result = String::new();

        let mut buf: [u8; 1] = [0];

        let mut read = self.port.read_exact(&mut buf);
        while !read.is_err() {
            if buf[0] as char == '\n' {
                return Ok(result.replace("\r", ""));
            }

            result += &String::from_utf8(buf.to_vec()).unwrap();

            read = self.port.read_exact(&mut buf);
        }

        Err("Failed to read! (no data available)".to_string())
    }

    fn status(&mut self) -> Result<bool, String> {
        match self.port.write("AT$STATUS\r\n".as_bytes()) {
            Ok(_) => (),
            Err(x) => return Err(x.to_string()),
        };

        let data = self.read_until_terminator();

        println!("{:?}", data);

        return Ok(true);
    }

    pub fn parse_information(&mut self, data: String) -> Result<Information, String> {
        if !data.contains("$RECV") {
            return Err("Invalid data format!".to_string());
        }

        let information_structure: Vec<[&'static str; 2]> = vec![
            ["rssi", "INT"],
            ["id", "STR"],
            ["header", "INT"],
            ["sequence", "INT"],
            ["uptime", "INT"],
            ["altitude", "INT"],
            ["co2_conc", "INT"],
            ["humidity", "F32"],
            ["illuminance", "INT"],
            ["motion_count", "INT"],
            ["orientation", "INT"],
            ["press_count", "INT"],
            ["pressure", "INT"],
            ["sound_level", "INT"],
            ["temperature", "F32"],
            ["voc_conc", "INT"],
            ["voltage", "F32"],
        ];

        let mut information = Information::new();

        let mut i = 0;

        let data = data.replace("$RECV: ", "");
        let raw_data: Vec<&str> = data.split(",").collect();

        for [name, information_type] in information_structure.iter() {
            println!("{} {}", raw_data[i], name);
            match *information_type {
                "INT" => {
                    let val = match raw_data[i].parse() {
                        Ok(x) => x,
                        Err(_) => {
                            return Err(format!(
                                "Failed to read {}, \"{}\" is not {}",
                                name, raw_data[i], information_type
                            ))
                        }
                    };

                    information.set_field_i32(name.to_string(), val)
                }
                "STR" => information.set_field_string(name.to_string(), raw_data[i].to_string()),
                "F32" => {
                    let val = match raw_data[i].parse() {
                        Ok(x) => x,
                        Err(_) => {
                            return Err(format!(
                                "Failed to read {}, \"{}\" is not {}",
                                name, raw_data[i], information_type
                            ))
                        }
                    };

                    information.set_field_f32(name.to_string(), val);
                }
                _ => unreachable!(),
            }
            i += 1;
        }

        return Ok(information);
    }

    pub fn wait_for_information(&mut self) -> Result<Information, String> {
        let start = Instant::now();
        while start.elapsed() <= Duration::from_millis((self.timeout * 1000).try_into().unwrap()) {
            let read = self.read_until_terminator();
            if read.is_ok() {
                return self.parse_information(read.unwrap());
            }
            thread::sleep(Duration::from_millis(300));
        }

        Err("Timed out".to_string())
    }
}
