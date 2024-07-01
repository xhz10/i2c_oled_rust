use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::Point,
    text::Text,
    Drawable,
};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};

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
