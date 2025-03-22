// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                            Sing your own tune                             |
// +---------------------------------------------------------------------------+

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_mar_2025::music::{Note, OCTAVE};
use embassy_rp::pwm::{self, Pwm, SetDutyCycle};
use embassy_rp::pwm::Config as ConfigPwm;
use embassy_time::{Duration, Timer};
use embedded_hal_1::delay;
use embedded_hal_async::digital::Wait;
use fixed::traits::ToFixed;
// Use the `panic_probe` crate to provided the panic handler and the
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
#[allow(unused)]
use defmt::*;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());

    /// Beats per minute.
    const TEMPO: u64 = 100;
    /// A whole note duration in milliseconds.
    const WHOLE_NOTE: u64 = 4 * (60_000 / TEMPO);
    /// The microcontroller clock frequency
    const CLOCK_FREQ: u64 = 150_000_000;
    /// PWM clock divider
    const PWM_DIV: u64 = 64;

    let mut buzzer_cfg = pwm::Config::default();
    buzzer_cfg.divider = PWM_DIV.to_fixed();
    let mut buzzer = pwm::Pwm::new_output_b(
        peripherals.PWM_SLICE1,
        peripherals.PIN_3,
        buzzer_cfg.clone()
    );

    const SONG: [(Option<Note>, i8); 10] = [
        (Some(Note::D4), 8),
        (Some(Note::D4), 8),
        (Some(Note::D5), 4),
        (Some(Note::A4), 3),
        (Some(Note::GS4), 4),
        (Some(Note::G4), 4),
        (Some(Note::F4), 4),
        (Some(Note::D4), -8),
        (Some(Note::F4), -8),
        (Some(Note::G4), -8),
    ];

    loop {
        for (note, length) in SONG {
            // TODO: Compute the note's duration based on
            // the length variable.

            let duration = Duration::from_millis(WHOLE_NOTE / length.abs() as u64);
            

            match note {
                Some(note) => {
                    // TODO: Configure the `top` and `compare_X` registers
                    // based on the note's type and change the PWM's config.
                    // Keep in mind that we are aiming for a 50% duty cycle.
                    // "Play" the note for 90% of the duration, then insert
                    // a 10% pause before playing the next note.

                    let frequency= note;

                    buzzer_cfg.top = (CLOCK_FREQ / (PWM_DIV * frequency as u64)) as u16;

                    let playduration = duration * 9 / 10;
                    let pauseduration = duration - playduration;

                    buzzer_cfg.compare_b = buzzer_cfg.top / 2;

                    info!("playing {:?}", note);

                    buzzer.set_config(&buzzer_cfg);
                    Timer::after(playduration).await;

                    

                    // buzzer_cfg.compare_b = 0;
                    // buzzer.set_config(&buzzer_cfg);
                    // Timer::after(pauseduration).await;
                },
                None => {
                    // TODO: Just wait the whole duration.
                    Timer::after(Duration::from(duration)).await;
                }
            };
        }
    }
}
