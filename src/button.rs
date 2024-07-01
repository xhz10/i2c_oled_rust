use std::sync::{Arc, Mutex};

use rppal::gpio::{Gpio, Trigger};
pub use rppal::gpio::InputPin;
use crate::event_queue::Event::BUTTON;
use crate::event_queue::EventQueue;

pub fn button_init(queue: Arc<Mutex<EventQueue>>) -> Option<InputPin> {
    let button_pin = 17; // 根据实际连接的引脚进行修改

    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    // 上拉电阻初始化一下 返回回去
    let mut pin = gpio.get(button_pin).expect("Failed to get GPIO pin")
        .into_input_pulldown();
    let queue_clone = queue.clone();
    pin.set_async_interrupt(Trigger::FallingEdge, move |_| {
        let mut real_queue = queue_clone.lock().unwrap();
        real_queue.push(BUTTON);
    }).ok();
    Some(pin)
}
