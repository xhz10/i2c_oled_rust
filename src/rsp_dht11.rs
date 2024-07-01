use dht11_gpio::{DHT11Controller, Sensor};


pub struct Dht11Res {
    pub temperature: f64,
    pub humidity: f64,
}

pub fn get_temperature_humidity() -> Result<Dht11Res,String> {
    const DHT11_PIN: u8 = 22;

    let mut sensor = match DHT11Controller::new(DHT11_PIN) {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("Failed to initialize sensor: {:?}", e));
        }
    };
    match sensor.read_sensor_data() {
        Ok(data) => {
            let result = Dht11Res {
                temperature: data.temperature,
                humidity: data.humidity,
            };
            Ok(result)
        },
        Err(e) => {
            Err(format!("Failed to read sensor data: {:?}", e))
        }
    }
}