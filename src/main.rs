use std::sync::{Arc, Mutex};
use embedded_graphics::{mono_font, prelude::*};
use ssd1306::{size::DisplaySize128x64, I2CDisplayInterface};
use std::thread;
use std::time::Duration;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use ssd1306::prelude::I2CInterface;
use i2c_oled_rust::display::{DisplayInfo, GraphicDisplay};
use i2c_oled_rust::rsp_dht11::get_temperature_humidity;
use i2c_oled_rust::sysop::SystemOperation;

const LCD_ADDRESS: u16 = 0x3c;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _i2c = I2c::new()
        .and_then(|mut _i2c| {
            _i2c.set_slave_address(LCD_ADDRESS)?;
            return Ok(_i2c);
        })
        .unwrap();

    let interface = I2CDisplayInterface::new(_i2c);
    let mut display: _ = GraphicDisplay::new(
        interface,
        DisplaySize128x64,
        &mono_font::ascii::FONT_9X15_BOLD,
    );
    let sys = SystemOperation::new();


    let button_pin = 3; // 根据实际连接的引脚进行修改

    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let pin = gpio.get(button_pin).expect("Failed to get GPIO pin").into_input_pulldown();

    let pin = Arc::new(Mutex::new(pin));

    let pin_clone = Arc::clone(&pin);
    let show_switch = Arc::new(Mutex::new(true));
    let show_switch_clone = Arc::clone(&show_switch);
    thread::spawn(move || {
        println!("按下按钮以触发控制台打印");

        loop {
            let pin = pin_clone.lock().unwrap();
            if pin.is_low() { // 按钮按下时引脚电平为高电平
                println!("按钮已按下!");
                let mut switch = show_switch_clone.lock().unwrap();
                *switch = !*switch;
                thread::sleep(Duration::from_millis(200));
            }
            drop(pin);
            thread::sleep(Duration::from_millis(10));
        }
    });


    let cpu_show_position = cpu_display_info();
    let dht11_show_position = dht11_display_info();

    loop {
        let show_switch = *show_switch.lock().unwrap();
        display.clear();
        // ip 是一定要输出的
        match show_switch {
            // true展示CPU
            true => {show_cpu(&sys, &mut display, &cpu_show_position.0,
                              &cpu_show_position.1, &cpu_show_position.2, &cpu_show_position.3)}

            // false展示Dht11
            false => {show_dht11(&sys, &mut display, &dht11_show_position.0,
                                 &dht11_show_position.1, &dht11_show_position.2)}
        }
        display.flush();

        thread::sleep(Duration::from_secs(5));
    }


}
fn cpu_display_info() -> (DisplayInfo,DisplayInfo,DisplayInfo,DisplayInfo) {
    let top_display = DisplayInfo {
        pos: Point::new(0, 10),
        style: MonoTextStyleBuilder::new()
            .font(&mono_font::ascii::FONT_6X13_BOLD)
            .text_color(BinaryColor::On)
            .build(),
    };
    let cpu_display = DisplayInfo {
        pos: Point::new(0, 25),
        style: MonoTextStyleBuilder::new()
            .font(&mono_font::ascii::FONT_6X13_BOLD)
            .text_color(BinaryColor::On)
            .build(),
    };
    let mem_display = DisplayInfo {
        pos: Point::new(0, 40),
        style: MonoTextStyleBuilder::new()
            .font(&mono_font::ascii::FONT_6X13_BOLD)
            .text_color(BinaryColor::On)
            .build(),
    };
    let cpu_temperature_display = DisplayInfo {
        pos: Point::new(0, 60),
        style: MonoTextStyleBuilder::new()
            .font(&mono_font::ascii::FONT_6X13_BOLD)
            .text_color(BinaryColor::On)
            .build(),
    };
    // return 出去
    (top_display,cpu_display,mem_display,cpu_temperature_display)
}

fn dht11_display_info() -> (DisplayInfo,DisplayInfo,DisplayInfo) {
    let top_display = DisplayInfo {
        pos: Point::new(0, 10),
        style: MonoTextStyleBuilder::new()
            .font(&mono_font::ascii::FONT_6X13_BOLD)
            .text_color(BinaryColor::On)
            .build(),
    };
    let middle_display = DisplayInfo {
        pos: Point::new(0, 35),
        style: MonoTextStyleBuilder::new()
            .font(&mono_font::ascii::FONT_6X13_BOLD)
            .text_color(BinaryColor::On)
            .build(),
    };
    let bottom_display = DisplayInfo {
        pos: Point::new(0, 55),
        style: MonoTextStyleBuilder::new()
            .font(&mono_font::ascii::FONT_6X13_BOLD)
            .text_color(BinaryColor::On)
            .build(),
    };

    // return 出去
    (top_display,middle_display,bottom_display)
}

fn show_cpu(sys: &SystemOperation, display: &mut GraphicDisplay<I2CInterface<I2c>, DisplaySize128x64>,
            top_display: &DisplayInfo,
            middle_display: &DisplayInfo,
            bottom_display: &DisplayInfo,
            last_display: &DisplayInfo) {
    let _cpu_usage = sys.cpu_info();
    let _mem_usage = sys.memory_info();
    let _cpu_temp = sys.cpu_temperature();
    let ip_addr = sys.ip_addr();
    display.write_text_with_style(format!("ip: {: >4}", ip_addr), top_display);
    display.write_text_with_style(
        format!("CPU: {: >2}% ", _cpu_usage, ),
        middle_display,
    );
    display.write_text_with_style(
        format!("Memory: {: >2}%", _mem_usage),
        bottom_display,
    );
    display.write_text_with_style(format!("CPU_T: {: >3}C", _cpu_temp), last_display);
}


fn show_dht11(sys: &SystemOperation,
              display: &mut GraphicDisplay<I2CInterface<I2c>, DisplaySize128x64>,
              top_display: &DisplayInfo,
              middle_display: &DisplayInfo,
              bottom_display: &DisplayInfo) {
    let dht11_res = get_temperature_humidity();
    let ip_addr = sys.ip_addr();
    display.write_text_with_style(format!("ip: {: >4}", ip_addr), top_display);
    // 温湿度输出一下
    match dht11_res {
        Ok(data) => {
            display.write_text_with_style(format!("Temperature: {: >1}C", data.temperature), middle_display,
            );
            display.write_text_with_style(format!("Humidity: {: >1}%", data.humidity), bottom_display,
            );
        }
        Err(_e) => {
            display.write_text_with_style("Temperature: error".to_string(), middle_display,
            );
            display.write_text_with_style("Humidity: error".to_string(), middle_display,
            );
        }
    }
}