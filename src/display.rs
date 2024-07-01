use embedded_graphics::{mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder}, pixelcolor::BinaryColor, prelude::Point, text::Text, Drawable, mono_font};
use rppal::i2c::I2c;
use ssd1306::{I2CDisplayInterface, mode::BufferedGraphicsMode, prelude::*, Ssd1306};

pub struct GraphicDisplay<'a, DI, DS: DisplaySize> {
    display: Ssd1306<DI, DS, BufferedGraphicsMode<DS>>,
    text_style: MonoTextStyle<'a, BinaryColor>,
}

pub struct DisplayInfo<'c> {
    pub pos: Point,
    pub style: MonoTextStyle<'c,BinaryColor>,
}

impl<'a, DI, DS> GraphicDisplay<'a, DI, DS>
where
    DI: WriteOnlyDataCommand, /* i2c interface*/
    DS: DisplaySize,
{
    pub fn new(i2c_interface: DI, size: DS, font: &'a MonoFont) -> Self {
        let mut display = Ssd1306::new(
            i2c_interface,
            size,
            DisplayRotation::Rotate0,
        )
        .into_buffered_graphics_mode();
        display.init().unwrap();
        display.clear();

        Self {
            display,
            text_style: MonoTextStyleBuilder::new()
                .font(&font)
                .text_color(BinaryColor::On)
                .build(),
        }
    }
    pub fn clear(&mut self) {
        self.display.clear();
    }
    pub fn flush(&mut self) {
        self.display.flush().expect("flush error wuwuw~");
    }

    pub fn write_text_with_style(
        &mut self,
        text: String,
        display_info: &DisplayInfo
    ) {
        Text::new(text.as_str(), display_info.pos, display_info.style).draw(&mut self.display).expect("error wuwuwu~");
    }

}

pub fn init_i2c_display<'a>(i2c_interface :I2CInterface<I2c>) -> GraphicDisplay<'a,I2CInterface<I2c>, DisplaySize128x64>{
    GraphicDisplay::new(
        i2c_interface,
        DisplaySize128x64,
        &mono_font::ascii::FONT_9X15_BOLD,
    )
}

pub fn cpu_display_info<'a>() -> (DisplayInfo<'a>, DisplayInfo<'a>, DisplayInfo<'a>, DisplayInfo<'a>) {
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

pub fn dht11_display_info<'a>() -> (DisplayInfo<'a>, DisplayInfo<'a>, DisplayInfo<'a>) {
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

