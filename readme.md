# AHT10 Rust Library
[![crates.io](https://img.shields.io/crates/v/aht10-embedded)](https://crates.io/crates/aht10-embedded) [![MIT](https://img.shields.io/github/license/jnthbdn/rs-aht10)](https://opensource.org/licenses/MIT) [![GitHub](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/jnthbdn/rs-aht10)

## Why?
I haven't found a driver that works simply (without async 😉 ) for an embedded environment and `#no_std`.

## Usage
First, create the AHT10 struct, with your I2C
```rust
let mut aht = AHT10::new(i2c1);
```

Then, initialize the sensor
```rust
match aht.initialize() {
    Ok(_) => (),
    Err(e) => {
        // AAAAaarrrgggg.... 😵
    }
}
```

Finally, read the data:
```rust
match aht.read_data(&mut delay) {
    Ok(data) => {
        // Yay ! It works !
        let celsius = data.temperature_celsius();
        let fahrenheit = data.temperature_fahrenheit(); // 😶
        let humidity = data.humidity();
    }
    Err(e) => {
        // AAAAaarrrgggg... 😵
    }
};
```

## Example
There is only one example named `rp_pico_aht10`. It uses GPIOs 2 & 3 (SDA & SCL respectively) and print data on USB serial port.

