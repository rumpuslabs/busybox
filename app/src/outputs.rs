use switch_hal::OutputSwitch;

pub trait SettableSwitch {
    type Error;

    fn set(&mut self, state: bool) -> Result<(), Self::Error>;
}

impl<S: OutputSwitch> SettableSwitch for S {
    type Error = S::Error;

    fn set(&mut self, state: bool) -> Result<(), Self::Error> {
        if state {
            self.on()
        } else {
            self.off()
        }
    }
}
