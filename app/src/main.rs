#![no_main]
#![no_std]

use core::convert::Infallible;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use hal::gpio::{IoPin, PinState};
use hal::pins::PinMode;
use lpc176x5x_hal as hal;
use switch_hal::{IntoSwitch, OutputSwitch, ToggleableOutputSwitch};

#[cfg(not(feature = "no-semihosting"))]
use panic_semihosting as _;

#[cfg(feature = "no-semihosting")]
use panic_reset as _;

mod clock;
mod inputs;
mod outputs;
mod power;

use crate::{inputs::*, outputs::*, power::*};

#[entry]
fn main() -> ! {
    inner_main();
    unreachable!()
}

const STARTUP_LED_CHASE_TIME_MS: usize = 200;

fn inner_main() -> Result<Infallible, Infallible> {
    let mut p = hal::Peripherals::new();

    clock::init(&mut p);

    hprintln!();
    hprintln!("Hello, world!");

    let mut power_ctl = PowerCtl::new(
        p.pins
            .p0_09
            .into_output_pin(PinState::High)?
            .into_active_low_switch(),
    )?;

    let ks1_input = ToggleSwitch::new(
        p.pins
            .p0_08
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let pb1_input = PressSwitch::new(
        p.pins
            .p0_07
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let mut d1_led = p
        .pins
        .p0_06
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let mut d2r_led = p
        .pins
        .p2_03
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let mut d2g_led = p
        .pins
        .p2_04
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let mut d2b_led = p
        .pins
        .p2_05
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let re1a_input = ToggleSwitch::new(
        p.pins
            .p0_00
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let re1b_input = ToggleSwitch::new(
        p.pins
            .p0_01
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let pb2_input = PressSwitch::new(
        p.pins
            .p0_18
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let mut d3_led = p
        .pins
        .p0_17
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let sw1_input = ToggleSwitch::new(
        p.pins
            .p0_15
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let sw2_input = PressSwitch::new(
        p.pins
            .p0_16
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let sw3_input = ToggleSwitch::new(
        p.pins
            .p0_23
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let mut sw3_led = p
        .pins
        .p0_24
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let sw4_input = ToggleSwitch::new(
        p.pins
            .p0_25
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let mut dac_out = p
        .pins
        .p0_26
        .into_aout()
        .into_dac_out(p.dac, 0.0056409, 0.36534);

    let mut sw4_led = p
        .pins
        .p1_30
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let pb3_input = ToggleSwitch::new(
        p.pins
            .p1_31
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let mut pb3_led = p
        .pins
        .p0_02
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let sw5a_input = ToggleSwitch::new(
        p.pins
            .p0_03
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let sw5b_input = ToggleSwitch::new(
        p.pins
            .p0_21
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let mut sw5_leds = p
        .pins
        .p2_13
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let mut pb4_led = p
        .pins
        .p2_12
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let pb4_input = PressSwitch::new(
        p.pins
            .p2_11
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let mut pb5_led = p
        .pins
        .p2_08
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let pb5_input = PressSwitch::new(
        p.pins
            .p2_07
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let mut pb6_led = p
        .pins
        .p2_06
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let pb6_input = PressSwitch::new(
        p.pins
            .p2_02
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let pb7_input = PressSwitch::new(
        p.pins
            .p2_01
            .set_mode(PinMode::PullUp)
            .into_input_pin()?
            .into_active_low_switch(),
    )?;

    let mut d4a_led = p
        .pins
        .p2_00
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let mut d4b_led = p
        .pins
        .p0_11
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let mut d4c_led = p
        .pins
        .p0_10
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let mut d4d_led = p
        .pins
        .p0_05
        .into_output_pin(PinState::Low)?
        .into_active_high_switch();

    let inputs: [&dyn SwitchInteractive<Error = Infallible>; 16] = [
        &ks1_input,
        &sw1_input,
        &sw2_input,
        &sw3_input,
        &sw4_input,
        &sw5a_input,
        &sw5b_input,
        &pb1_input,
        &pb2_input,
        &pb3_input,
        &pb4_input,
        &pb5_input,
        &pb6_input,
        &pb7_input,
        &re1a_input,
        &re1b_input,
    ];

    let mut outputs: [&mut dyn OutputSwitch<Error = Infallible>; 16] = [
        &mut d1_led,
        &mut d2r_led,
        &mut d2g_led,
        &mut d2b_led,
        &mut d3_led,
        &mut sw3_led,
        &mut sw4_led,
        &mut pb3_led,
        &mut sw5_leds,
        &mut pb4_led,
        &mut pb5_led,
        &mut pb6_led,
        &mut d4a_led,
        &mut d4b_led,
        &mut d4c_led,
        &mut d4d_led,
    ];

    let mut last_ms = 0;

    let mut count = 0;
    while count < (outputs.len() + 2) * STARTUP_LED_CHASE_TIME_MS {
        let now_ms = clock::now();
        let time_passed_ms = now_ms - last_ms;
        last_ms = now_ms;

        count += time_passed_ms;

        for (i, o) in outputs.iter_mut().enumerate() {
            if count < (STARTUP_LED_CHASE_TIME_MS * 2)
                || (count - STARTUP_LED_CHASE_TIME_MS * 2) / STARTUP_LED_CHASE_TIME_MS == i
            {
                o.on()?;
            } else {
                o.off()?;
            }
        }

        cortex_m::asm::wfi();
    }

    let mut dim_count = 0;
    let mut rgb_count = 0;
    let mut d4_count = 0;
    let mut gauge_count = 0;

    loop {
        let now_ms = clock::now();
        let time_passed_ms = now_ms - last_ms;
        last_ms = now_ms;

        for i in inputs {
            i.update(time_passed_ms)?;
        }

        if inputs.iter().any(|i| i.is_interactive()) {
            power_ctl.keep_on()?;
        }

        power_ctl.update(time_passed_ms)?;
        dim_count = (dim_count + 1) % 16;

        d1_led.set(power_ctl.is_on())?;

        if pb1_input.state() == PressState::JustPressed {
            rgb_count = (rgb_count + 1) % 8;
        }
        {
            let rgb_gray = rgb_count ^ (rgb_count >> 1);

            if dim_count <= 4 {
                d2r_led.set((rgb_gray & 1) != 0)?;
                d2g_led.set((rgb_gray & 2) != 0)?;
                d2b_led.set((rgb_gray & 4) != 0)?;
            } else {
                d2r_led.off()?;
                d2g_led.off()?;
                d2b_led.off()?;
            }
        }

        if pb2_input.state() == PressState::JustPressed {
            if d4_count == 0 || d4_count == 4 {
                d4_count = 4 - d4_count;
            } else {
                d4_count = -d4_count;
            }
        }

        if re1a_input.state() == ToggleState::TurnedOn {
            if re1b_input.state() == ToggleState::On {
                d4_count -= 1;
                if d4_count < -3 {
                    d4_count = 4;
                }
            } else {
                d4_count += 1;
                if d4_count > 4 {
                    d4_count = -3;
                }
            }
        }

        d4a_led.set(d4_count >= 1 || d4_count <= -4)?;
        d4b_led.set(d4_count >= 2 || d4_count <= -3)?;
        d4c_led.set(d4_count >= 3 || d4_count <= -2)?;
        d4d_led.set(d4_count >= 4 || d4_count <= -1)?;

        d3_led.set(
            (sw2_input.state() == PressState::Held && sw1_input.state() == ToggleState::Off)
                || (sw2_input.state() == PressState::Idle && sw1_input.state() == ToggleState::On),
        )?;

        sw3_led.set(sw3_input.state() == ToggleState::On)?;

        sw4_led.set(sw4_input.state() == ToggleState::On)?;

        if pb3_input.state() == ToggleState::On {
            pb3_led.set(dim_count <= 1)?;
        } else {
            pb3_led.off()?;
        }

        sw5_leds
            .set(sw5a_input.state() == ToggleState::On || sw5b_input.state() == ToggleState::On)?;

        pb4_led.set(pb4_input.state() == PressState::Held)?;

        if pb5_input.state() == PressState::JustPressed {
            pb5_led.toggle()?;
        }

        if pb6_input.state() == PressState::JustPressed {
            pb6_led.toggle()?;
        }

        gauge_count += time_passed_ms;
        gauge_count %= 100;
        if pb7_input.state() == PressState::JustPressed {
            dac_out.set_value(gauge_count as f32);
            gauge_count = 0;
        }

        cortex_m::asm::wfi();
    }
}
