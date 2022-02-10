#[derive(Debug, Clone)]
pub struct Information {
    pub rssi: i32,
    pub id: String,
    pub sequence: i32,
    pub altitude: i32,
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

    pub fn set_field_i32(&mut self, name: String, value: i32) {
        let name: &str = &name;
        match name {
            "rssi" => self.rssi = value,
            "sequence" => self.sequence = value,
            "altitude" => self.altitude = value,
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
}
