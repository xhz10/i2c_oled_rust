use rppal::gpio::Gpio;
pub use rppal::gpio::InputPin;

pub fn button_init() -> InputPin {
    let button_pin = 3; // 根据实际连接的引脚进行修改

    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    // 上拉电阻初始化一下 返回回去
    gpio.get(button_pin).expect("Failed to get GPIO pin")
        .into_input_pulldown()
}


pub fn user_put(pin: &InputPin) -> bool {
    loop {
        if pin.is_low() {
            true
        }
    }
}