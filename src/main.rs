use std::sync::{Arc, Mutex};
use ssd1306::{size::DisplaySize128x64};
use std::thread;
use std::time::Duration;
use rppal::i2c::I2c;
use ssd1306::prelude::I2CInterface;
use i2c_oled_rust::button::button_init;
use i2c_oled_rust::display::{cpu_display_info, dht11_display_info, DisplayInfo, GraphicDisplay, init_i2c_display};
use i2c_oled_rust::event_queue::Event::BUTTON;
use i2c_oled_rust::event_queue::EventQueue;
use i2c_oled_rust::i2c::{init_i2c_interface};
use i2c_oled_rust::rsp_dht11::get_temperature_humidity;
use i2c_oled_rust::sysop::SystemOperation;


fn main() {
    // 初始化i2c接口
    let i2c_interface = init_i2c_interface();
    // 初始化display
    let mut display: _ = init_i2c_display(i2c_interface);
    let sys = SystemOperation::new();
    // 创建一个事件队列
    let queue = Arc::new(Mutex::new(EventQueue::new()));
    let pop_queue = queue.clone();
    // 初始化按钮的pin操作,设置按钮中断 低电平触发
    let _pin = button_init(queue);
    // 获取展示的position
    let cpu_show_position = cpu_display_info();
    let dht11_show_position = dht11_display_info();
    let mut show_switch = true;
    loop {
        if let Some(event) = pop_queue.lock().unwrap().pop() {
            if event == BUTTON {
                show_switch = !show_switch;
            }
        }
        display.clear();
        match show_switch {
            // true展示CPU
            true => {show_cpu(&sys, &mut display, &cpu_show_position.0,
                              &cpu_show_position.1, &cpu_show_position.2, &cpu_show_position.3)}
            // false展示Dht11
            false => {show_dht11(&sys, &mut display, &dht11_show_position.0,
                                 &dht11_show_position.1, &dht11_show_position.2)}
        }
        // 刷新显示屏
        display.flush();
        // 5秒一轮询
        thread::sleep(Duration::from_secs(5));
    }
}

// 展示cpu信息的配置
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

// 展示dht11的配置
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