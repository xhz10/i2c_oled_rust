use rppal::i2c::I2c;
use ssd1306::I2CDisplayInterface;
use ssd1306::prelude::I2CInterface;

const LCD_ADDRESS: u16 = 0x3c;

pub fn init_i2c_interface() -> I2CInterface<I2c> {

    let i2c = I2c::new()
        .and_then(|mut _i2c| {
            _i2c.set_slave_address(LCD_ADDRESS)?;
            return Ok(_i2c);
        })
        .unwrap();
     I2CDisplayInterface::new(i2c)
}