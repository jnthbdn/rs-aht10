pub struct Aht10Data {
    temp: f32,
    humidity: f32,
}

impl Aht10Data {
    pub fn new(raw: [u8; 5]) -> Self {
        let raw_h: u32 =
            (raw[0] as u32) << 12 | (raw[1] as u32) << 4 | ((raw[2] as u32) & 0xF0) >> 4;
        let raw_t: u32 = ((raw[2] as u32) & 0x0F) << 16 | (raw[3] as u32) << 8 | (raw[4] as u32);

        Self {
            temp: (raw_t as f32) * 0.000191 - 50.0,
            humidity: (raw_h as f32) * 0.000095,
        }
    }

    pub fn temperature_celsius(&self) -> f32 {
        self.temp
    }

    pub fn temperature_fahrenheit(&self) -> f32 {
        self.temp * 1.8 + 32.0
    }

    pub fn humidity(&self) -> f32 {
        self.humidity
    }
}
