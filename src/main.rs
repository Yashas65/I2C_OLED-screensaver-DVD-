use esp_idf_svc::hal::{delay::FreeRtos, i2c::*, peripherals::Peripherals, units::FromValueType};
use embedded_graphics::{
    mono_font::{ascii::{FONT_10X20}, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

use sh1106::{prelude::*, Builder};


fn main() {

    esp_idf_svc::sys::link_patches();

    
    let peripherals = Peripherals::take().unwrap();
    //i2c
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;
    
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();
    
    let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    display.init().unwrap();
    display.flush().unwrap();
    
    let text_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    
    //aimation
    let mut y  = 10;
    let mut x = 20;
    let mut dy  =2;
    let mut dx = 2;
    
    
    log::info!("Hello, world!");
    
    log::info!("this is gonna be crazy shit");
    
    let text = "DVD";
    log::info!("{}",text);
    
    loop {
        display.clear();
        x += dx;
        y += dy;
        if x < 0 || x > 100 {dx = -dx;}
        if y < 10 || y > 60 {dy = -dy;}

        Text::new(text, Point::new(10, y), text_style)
        .draw(&mut display)
        .unwrap();
        display.flush().unwrap();
        
        Text::new("___", Point::new(10, y+2), text_style)
        .draw(&mut display)
        .unwrap();
        display.flush().unwrap();
        
        FreeRtos::delay_ms(50);
        
    }

}