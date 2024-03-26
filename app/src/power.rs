use switch_hal::OutputSwitch;

const POWER_OFF_TIMEOUT_MS: usize = 1000 * 60 * 10;

enum PowerState {
    On(usize),
    Off,
}

pub struct PowerCtl<S: OutputSwitch> {
    state: PowerState,
    output: S,
}

impl<S: OutputSwitch> PowerCtl<S> {
    pub fn new(output: S) -> Result<Self, S::Error> {
        let mut s = Self {
            state: PowerState::Off,
            output,
        };
        s.turn_on()?;
        Ok(s)
    }

    pub fn update(&mut self, time_passed_ms: usize) -> Result<(), S::Error> {
        match self.state {
            PowerState::On(ref mut timer_ms) => {
                *timer_ms = timer_ms.saturating_sub(time_passed_ms);
                if *timer_ms == 0 {
                    self.turn_off()?;
                }
            }

            PowerState::Off => {}
        };
        Ok(())
    }

    pub fn keep_on(&mut self) -> Result<(), S::Error> {
        self.turn_on()
    }

    pub fn is_on(&self) -> bool {
        matches!(self.state, PowerState::On(_))
    }

    fn turn_on(&mut self) -> Result<(), S::Error> {
        self.output.on()?;
        self.state = PowerState::On(POWER_OFF_TIMEOUT_MS);
        Ok(())
    }

    fn turn_off(&mut self) -> Result<(), S::Error> {
        self.output.off()?;
        self.state = PowerState::Off;
        Ok(())
    }
}
