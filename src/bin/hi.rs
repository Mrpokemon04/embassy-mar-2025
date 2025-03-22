// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+
 
//! By default, this app prints a "Hello world" message with `defmt`.
 
#![no_std]
#![no_main]
 
use embassy_executor::Spawner;
use embassy_rp::{gpio::{Input, Level, Output, Pull}, pwm::Pwm};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
use defmt::*;
 
// PWM config
use embassy_rp::pwm::Config as ConfigPwm; 
 
// Import interrupts definition module
mod irqs;
 
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());

    let mut switch4 = Input::new(peripherals.PIN_6, Pull::None);
    let mut switch5 = Input::new(peripherals.PIN_7, Pull::None);
    let mut switch6 = Input::new(peripherals.PIN_8, Pull::None);
    let mut switch7 = Input::new(peripherals.PIN_9, Pull::None);    
 
    loop {
        if switch4.is_low() || switch5.is_low() || switch6.is_low() || switch7.is_low() {
            println!("Mr. Frog Supporters");
            Timer::after_millis(200).await;
        }
    }
}
