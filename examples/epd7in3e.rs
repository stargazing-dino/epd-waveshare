//! A simple example for the usage of Waveshare 7.3" E-Paper Display (E)
//!
//! This example uses the 7-color version of the 7.3" E-Paper display from Waveshare
//! It demonstrates drawing colored shapes and text, as well as showing a test pattern.
//! The display supports 7 colors: black, white, red, yellow, blue, green, and orange.

use embedded_hal_mock::eh1::*;
use epd_waveshare::{
    color::OctColor,
    epd7in3e::{Display7in3e, Epd7in3e},
    prelude::*,
};

#[cfg(feature = "graphics")]
fn run_graphics_demo() -> Result<(), core::convert::Infallible> {
    // Create a new Mockup for SPI
    let mut spi = spi::Mock::new(&[]);

    // Create a new Mockup for Digital I/O
    let busy_in = pin::Mock::new(&[]);
    let dc = pin::Mock::new(&[]);
    let rst = pin::Mock::new(&[]);

    // No delay needed in this test environment
    let mut delay = delay::NoopDelay::new();

    // Initialize EPD with the mocks from above
    let mut epd = Epd7in3e::new(&mut spi, busy_in, dc, rst, &mut delay, None).unwrap();

    // Create a full-size display buffer
    let mut display = Display7in3e::default();

    // Use the `display` to draw on it using embedded-graphics
    use embedded_graphics::{
        mono_font::MonoTextStyle,
        prelude::*,
        primitives::{Circle, PrimitiveStyle, Rectangle, Triangle},
        text::Text,
    };

    // Clear the display to white
    display.clear(OctColor::White)?;

    // Draw some shapes with different colors
    // A black rectangle
    Rectangle::new(Point::new(10, 10), Size::new(100, 100))
        .into_styled(PrimitiveStyle::with_fill(OctColor::Black))
        .draw(&mut display)?;

    // A red circle
    Circle::new(Point::new(180, 60), 50)
        .into_styled(PrimitiveStyle::with_fill(OctColor::Red))
        .draw(&mut display)?;

    // A blue triangle
    Triangle::new(
        Point::new(300, 10),
        Point::new(350, 100),
        Point::new(250, 100),
    )
    .into_styled(PrimitiveStyle::with_fill(OctColor::Blue))
    .draw(&mut display)?;

    // A green rectangle
    Rectangle::new(Point::new(400, 10), Size::new(100, 100))
        .into_styled(PrimitiveStyle::with_fill(OctColor::Green))
        .draw(&mut display)?;

    // A yellow rectangle 
    Rectangle::new(Point::new(550, 10), Size::new(100, 100))
        .into_styled(PrimitiveStyle::with_fill(OctColor::Yellow))
        .draw(&mut display)?;

    // An orange rectangle at the bottom
    Rectangle::new(Point::new(10, 150), Size::new(780, 100))
        .into_styled(PrimitiveStyle::with_fill(OctColor::Orange))
        .draw(&mut display)?;

    // Send the drawing to the display and refresh the screen
    epd.update_and_display_frame(&mut spi, display.buffer(), &mut delay).unwrap();

    // Put the EPD to sleep to save power
    epd.sleep(&mut spi, &mut delay).unwrap();

    Ok(())
}

#[cfg(feature = "graphics")]
fn run_test_pattern() -> Result<(), core::convert::Infallible> {
    // Create a new Mockup for SPI
    let mut spi = spi::Mock::new(&[]);

    // Create a new Mockup for Digital I/O
    let busy_in = pin::Mock::new(&[]);
    let dc = pin::Mock::new(&[]);
    let rst = pin::Mock::new(&[]);

    // No delay needed in this test environment
    let mut delay = delay::NoopDelay::new();

    // Initialize EPD with the mocks from above
    let mut epd = Epd7in3e::new(&mut spi, busy_in, dc, rst, &mut delay, None).unwrap();

    // Display a test pattern showing all 7 available colors
    epd.show_colors_test(&mut spi, &mut delay).unwrap();

    // Put the EPD to sleep to save power
    epd.sleep(&mut spi, &mut delay).unwrap();

    Ok(())
}

fn main() {
    #[cfg(feature = "graphics")]
    {
        run_graphics_demo().unwrap();
        // Uncomment to run the test pattern instead
        // run_test_pattern().unwrap();
    }

    #[cfg(not(feature = "graphics"))]
    {
        println!("This example requires the 'graphics' feature to be enabled.");
    }
}