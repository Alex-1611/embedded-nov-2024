//! Go to [Random Color Generator](https://randomwordgenerator.com/color.php)
//! Generate two colors and get the RGB encodings for them. These are the colors
//! you will need to display on the RGB LED.
//!
//! Your application should smoothly transition from one color to another. The colors will
//! be displayed sequentially for 3 seconds each, with a gradual transition period of 1 second.
//!
//! Keep in mind that the RGB LED is common anode.
//!
//! For displaying the color on the LED, PWM (Pulse Width Modulation) will need to be set up
//! on the pin. Connect them to pins: GPIO0 (Red), GPIO1 (Green), and
//! GPIO2 (Blue). (Hint: Pin 0 and 1 will share the same channel).

#![no_std]
#![no_main]
// Delete the following line after you're done implementing
// the solution.
#![allow(unused)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::peripherals::{PIN_0, PIN_1, PIN_2, PWM_SLICE0, PWM_SLICE1};
use embassy_rp::pwm::{Config as PwmConfig, Pwm, SetDutyCycle};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
      

    let mut pwm1 = Pwm::new_output_ab(p.PWM_SLICE0, p.PIN_0, p.PIN_1, embassy_rp::pwm::Config::default());
    let mut pwm2 = Pwm::new_output_a(p.PWM_SLICE1, p.PIN_2, embassy_rp::pwm::Config::default());
    // TODO 1 : Modify the RGB values and loop through the configs to create a transition.
    loop{
        let mut configrg = embassy_rp::pwm::Config::default();
        configrg.top = 255;
        configrg.compare_a = 0;
        configrg.compare_b = 255;
        pwm1.set_config(&configrg);
        
        let mut configb = embassy_rp::pwm::Config::default();
        configb.compare_a = 255;
        configb.top = 255; 
        pwm2.set_config(&configb);

        Timer::after_secs(3).await;

        configrg.compare_a = 255;
        configrg.compare_b = 0;
        
        configb.compare_a = 255;

        pwm1.set_config(&configrg);
        pwm2.set_config(&configb);
        Timer::after_secs(3).await;
    }
}
