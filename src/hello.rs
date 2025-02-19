//! Print a "Hello, World!" message to the debugger and blink the LED on GPIO1.

#![no_std]
#![no_main]
// Delete the following line after you're done implementing
// the solution.
#![allow(unused)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::yield_now;
use embassy_rp::{gpio::{Level, Output, Pin}, usb::{Driver, InterruptHandler}};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

// TODO 2.1 : Write a task that blinks the LED connected to GPIO1.
#[embassy_executor::task]
async fn blink(mut led: Output<'static>){
    loop{
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // TODO 0 : Set the timer to (a)wait 5 seconds before printing
    //          the "Hello, World!" message.
    Timer::after_secs(5).await;
    // TODO 1 : Print the "Hello, World!" message to the USB serial port.
    info!("Hello world!");
    // TODO 2.2 : Spawn the task that blinks the LED connected to GPIO1.
    let mut led : Output<'_> = Output::new(p.PIN_0, Level::Low);
    spawner.spawn(blink(led)).unwrap();

}
