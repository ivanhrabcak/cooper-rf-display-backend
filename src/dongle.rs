use std::{
    collections::HashMap,
    io::Write,
    thread,
    time::{Duration, Instant},
};

use serial::SerialPort;

use crate::information::Information;

pub struct Dongle {
    pub port: Box<dyn SerialPort>,
    pub id: String,
    pub timeout: i32,
}

unsafe impl Send for Dongle {}

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

        let mut dongle = Self {
            port: Box::new(port),
            timeout: 35,
            id: "".to_string(),
        };

        dongle.id = dongle.get_id().unwrap();

        dongle
    }

    pub fn set_timeout(&mut self, timeout: i32) {
        self.timeout = timeout;
    }

    pub fn get_id(&mut self) -> Result<String, String> {
        match self.port.write("AT+CGSN\r\n".as_bytes()) {
            Ok(_) => (),
            Err(x) => return Err(x.to_string()),
        };

        let result = match self.read_until_terminator() {
            Ok(x) => Ok(x.replace("+CGSN: ", "")),
            Err(e) => Err(e.to_string()),
        };

        match self.read_until_terminator() {
            Ok(_) => result,
            Err(e) => Err(e.to_string()),
        }
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

    pub fn wait_for_information(&mut self) -> Result<Information, String> {
        let start = Instant::now();
        while start.elapsed() <= Duration::from_millis((self.timeout * 1000).try_into().unwrap()) {
            let read = self.read_until_terminator();
            if read.is_ok() {
                return Information::parse(read.unwrap());
            }
            thread::sleep(Duration::from_millis(300));
        }

        Err("Timed out".to_string())
    }

    pub fn get_stations(&mut self) -> Result<HashMap<String, String>, String> {
        match self.port.write("AT$LIST\r\n".as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        };

        let mut buffer = vec![];
        let mut x = match self.read_until_terminator() {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };

        while x != "OK" {
            buffer.push(x);
            x = match self.read_until_terminator() {
                Ok(x) => x,
                Err(e) => return Err(e.to_string()),
            };
        }

        let mut stations = HashMap::new();

        for station in buffer {
            let station_id = match station.split(",").nth(0) {
                Some(x) => x.to_string(),
                None => return Err("Invalid data! (id)".to_string()),
            };

            let station_name = match station.split(",").nth(1) {
                Some(x) => x.to_string(),
                None => return Err("Invalid data! (name)".to_string()),
            }
            .replace("\"", "");

            stations.insert(station_id, station_name);
        }

        Ok(stations)
    }

    pub fn lookup_name_by_id(&mut self, id: &String) -> Result<String, String> {
        let stations = match self.get_stations() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

        let station_name = stations.get(id);

        match station_name {
            Some(x) => Ok(x.to_string()),
            None => Err("Station does not exist!".to_string()),
        }
    }
}
