#![no_std]

pub use embedded_hal as hal;
pub use lpc176x5x_pac as pac;

pub mod dac;
pub mod gpio;
pub mod pins;

use pins::Pins;

pub struct Peripherals {
    pub dac: pac::DAC,
    pub pins: Pins,
    pub syscon: pac::SYSCON,
    pub systick: pac::SYST,
}

impl Peripherals {
    pub fn new() -> Peripherals {
        Self::take().unwrap()
    }

    pub fn take() -> Option<Peripherals> {
        Some(Peripherals::from((
            pac::Peripherals::take()?,
            pac::CorePeripherals::take()?,
        )))
    }
}

impl From<(pac::Peripherals, pac::CorePeripherals)> for Peripherals {
    fn from(pac: (pac::Peripherals, pac::CorePeripherals)) -> Self {
        let p = pac.0;
        let cp = pac.1;
        Peripherals {
            dac: p.DAC,
            pins: Pins::new(p.PINCONNECT, p.GPIO),
            syscon: p.SYSCON,
            systick: cp.SYST,
        }
    }
}
