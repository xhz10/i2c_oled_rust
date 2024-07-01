# i2c_oled_rust

#### 介绍
利用rust和树莓派实现显示屏根据按钮切换展示系统信息还是dht11传感器信息

#### 软件架构
软件架构说明


#### 安装教程

1.  树莓派支持开启I2C协议和使用GPIO引脚
2.  打开button.rs和rsp_dht11.rs 记得修改一下GPIO引脚号
2.  树莓派环境下
    `cargo clean`
    `cargo run --release`

#### 使用说明

1.  显示屏上最先显示的是你的IP、CPU信息、内存信息、CPU温度信息
2.  按动按钮后5秒内转换为dht11传感器的温湿度信息
3.  由于是利用硬件中断控制的按钮点击事件，可能会出现按动一次触发两次的情况，后续会做好防抖

