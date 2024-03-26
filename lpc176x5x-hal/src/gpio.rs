use crate::pac;
use crate::pins::*;
use core::convert::Infallible;
use core::marker::PhantomData;

pub use crate::hal::digital::v2::*;

pub trait PinDir {}
pub struct Input;
impl PinDir for Input {}
pub struct Output;
impl PinDir for Output {}

impl<P, AF> Pin<P, AF>
where
    P: PinId,
    AF: AlternateFunction,
{
    fn read_pin(&self) -> bool {
        let gpio = pac::GPIO::ptr();
        unsafe { ((*gpio).pin[self.id.port()].read().bits() & (1 << self.id.pin())) != 0 }
    }
}

impl<P, IO> Pin<P, GPIO<IO>>
where
    P: PinId,
    IO: PinDir,
{
    pub fn into_input(self) -> Pin<P, GPIO<Input>> {
        let gpio = pac::GPIO::ptr();
        unsafe {
            (*gpio).dir[self.id.port()].modify(|r, w| {
                let mut bits = r.bits();
                bits &= !(1 << self.id.pin());
                w.bits(bits)
            })
        };
        Pin {
            id: self.id,
            _marker: PhantomData,
        }
    }

    pub fn into_output(self) -> Pin<P, GPIO<Output>> {
        let gpio = pac::GPIO::ptr();
        unsafe {
            (*gpio).dir[self.id.port()].modify(|r, w| {
                let mut bits = r.bits();
                bits |= 1 << self.id.pin();
                w.bits(bits)
            })
        };
        Pin {
            id: self.id,
            _marker: PhantomData,
        }
    }
}

impl<P> Pin<P, GPIO<Output>>
where
    P: PinId,
{
    fn set_pin(&mut self, value: bool) {
        let gpio = pac::GPIO::ptr();
        unsafe {
            if value {
                (*gpio).set[self.id.port()].write(|w| w.bits(1 << self.id.pin()))
            } else {
                (*gpio).clr[self.id.port()].write(|w| w.bits(1 << self.id.pin()))
            }
        }
    }
}

impl<P, IO> IoPin<Pin<P, GPIO<Input>>, Pin<P, GPIO<Output>>> for Pin<P, GPIO<IO>>
where
    P: PinId,
    IO: PinDir,
{
    type Error = Infallible;

    fn into_input_pin(self) -> Result<Pin<P, GPIO<Input>>, Self::Error> {
        Ok(self.into_input())
    }

    fn into_output_pin(self, state: PinState) -> Result<Pin<P, GPIO<Output>>, Self::Error> {
        let mut pin = self.into_output();
        match state {
            PinState::Low => pin.set_low()?,
            PinState::High => pin.set_high()?,
        };
        Ok(pin)
    }
}

impl<P> InputPin for Pin<P, GPIO<Input>>
where
    P: PinId,
{
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.read_pin())
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.read_pin())
    }
}

impl<P> OutputPin for Pin<P, GPIO<Output>>
where
    P: PinId,
{
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_pin(true);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_pin(false);
        Ok(())
    }
}

impl<P> StatefulOutputPin for Pin<P, GPIO<Output>>
where
    P: PinId,
{
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.read_pin())
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.read_pin())
    }
}

impl<P> toggleable::Default for Pin<P, GPIO<Output>> where P: PinId {}
