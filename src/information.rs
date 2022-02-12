use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Information {
    pub rssi: i32,
    pub id: String,
    pub sequence: i32,
    pub altitude: i64,
    pub co2_concentration: i32,
    pub humidity: f32,
    pub illuminance: i32,
    pub motion_count: i32,
    pub orientation: i32,
    pub press_count: i32,
    pub sound_level: i32,
    pub temperature: f32,
    pub voc_conc: i32,
    pub voltage: f32,
    pub pressure: i32,
    pub uptime: i32,
}

impl Information {
    pub fn new() -> Self {
        Self {
            rssi: -1,
            id: "-1".to_string(),
            sequence: -1,
            altitude: -1,
            co2_concentration: -1,
            humidity: -1.0,
            illuminance: -1,
            motion_count: -1,
            orientation: -1,
            press_count: -1,
            sound_level: -1,
            temperature: -1.0,
            voc_conc: -1,
            voltage: -1.0,
            pressure: -1,
            uptime: -1,
        }
    }

    pub fn set_field_i64(&mut self, name: String, value: i64) {
        let name: &str = &name;
        match name {
            "altitude" => self.altitude = value,
            _ => println!("Warning! i64 field \"{name}\" was not found!"),
        }
    }

    pub fn set_field_i32(&mut self, name: String, value: i32) {
        let name: &str = &name;
        match name {
            "rssi" => self.rssi = value,
            "sequence" => self.sequence = value,
            "co2_conc" => self.co2_concentration = value,
            "illuminance" => self.illuminance = value,
            "motion_count" => self.motion_count = value,
            "orientation" => self.orientation = value,
            "press_count" => self.press_count = value,
            "pressure" => self.pressure = value,
            "sound_level" => self.sound_level = value,
            "voc_conc" => self.voc_conc = value,
            "header" => (),
            "uptime" => self.uptime = value,
            _ => println!("Warning! i32 field \"{}\" was not found!", name),
        }
    }

    pub fn set_field_string(&mut self, name: String, value: String) {
        let name: &str = &name;
        match name {
            "id" => self.id = value,
            _ => println!("Warning! string field \"{}\" was not found!", name),
        }
    }

    pub fn set_field_f32(&mut self, name: String, value: f32) {
        let name: &str = &name;
        match name {
            "humidity" => self.humidity = value,
            "temperature" => self.temperature = value,
            "voltage" => self.voltage = value,
            _ => println!("Warning! i32 field \"{}\" was not found!", name),
        }
    }

    pub fn parse(data: String) -> Result<Information, String> {
        if !data.contains("$RECV") {
            return Err("Invalid data format!".to_string());
        }

        let information_structure: Vec<[&'static str; 2]> = vec![
            ["rssi", "INT"],
            ["id", "STR"],
            ["header", "INT"],
            ["sequence", "INT"],
            ["uptime", "INT"],
            ["altitude", "INT64"],
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
            match *information_type {
                "INT" => {
                    if raw_data[i] == "" {
                        information.set_field_i32(name.to_string(), -1);
                    } else {
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
                }
                "STR" => information.set_field_string(name.to_string(), raw_data[i].to_string()),
                "F32" => {
                    if raw_data[i] == "" {
                        information.set_field_f32(name.to_string(), -1.0);
                    } else {
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
                }
                "INT64" => {
                    if raw_data[i] == "" {
                        information.set_field_i64(name.to_string(), -1);
                    } else {
                        let val: i64 = match raw_data[i].parse() {
                            Ok(x) => x,
                            Err(_) => {
                                return Err(format!(
                                    "Failed to read {name}, \"{}\" is not {information_type}",
                                    raw_data[i]
                                ))
                            }
                        };

                        information.set_field_i64(name.to_string(), val)
                    }
                }
                _ => unreachable!(),
            }
            i += 1;
        }

        return Ok(information);
    }
}
