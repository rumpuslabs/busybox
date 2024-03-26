use core::cell::Cell;
use switch_hal::InputSwitch;

pub trait SwitchInteractive {
    type Error;

    fn update(&self, time_passed_ms: usize) -> Result<(), Self::Error>;
    fn is_interactive(&self) -> bool;
}

struct DebouncedSwitch<S: InputSwitch> {
    state: Cell<bool>,
    state_changed: Cell<bool>,
    count: Cell<u8>,
    input: S,
}

impl<S: InputSwitch> DebouncedSwitch<S> {
    fn new(input: S) -> Self {
        Self {
            state: Cell::new(false),
            state_changed: Cell::new(false),
            count: Cell::new(0),
            input,
        }
    }

    fn update(&self, time_passed_ms: usize) -> Result<(), S::Error> {
        let new_state = self.input.is_active()?;
        if new_state != self.state.get() {
            let new_count = (self.count.get() as usize) + time_passed_ms;
            if new_count >= 3 {
                self.state.set(new_state);
                self.state_changed.set(true);
                self.count.set(0);
            } else {
                self.state_changed.set(false);
                self.count.set(new_count as u8);
            }
        } else {
            self.state_changed.set(false);
            self.count.set(self.count.get().saturating_sub(1));
        }
        Ok(())
    }
}

pub struct PressSwitch<S: InputSwitch> {
    inner: DebouncedSwitch<S>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressState {
    Idle,
    JustPressed,
    Held,
    Released,
}

impl<S: InputSwitch> PressSwitch<S> {
    pub fn new(input: S) -> Result<Self, S::Error> {
        Ok(Self {
            inner: DebouncedSwitch::new(input),
        })
    }

    pub fn state(&self) -> PressState {
        match (self.inner.state.get(), self.inner.state_changed.get()) {
            (false, false) => PressState::Idle,
            (true, true) => PressState::JustPressed,
            (true, false) => PressState::Held,
            (false, true) => PressState::Released,
        }
    }
}

impl<S: InputSwitch> SwitchInteractive for PressSwitch<S> {
    type Error = S::Error;

    fn update(&self, time_passed_ms: usize) -> Result<(), Self::Error> {
        self.inner.update(time_passed_ms)
    }

    fn is_interactive(&self) -> bool {
        self.inner.state.get()
    }
}

pub struct ToggleSwitch<S: InputSwitch> {
    inner: DebouncedSwitch<S>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleState {
    Off,
    TurnedOn,
    On,
    TurnedOff,
}

impl<S: InputSwitch> ToggleSwitch<S> {
    pub fn new(input: S) -> Result<Self, S::Error> {
        let s = Self {
            inner: DebouncedSwitch::new(input),
        };
        s.inner.state.set(s.inner.input.is_active()?);
        Ok(s)
    }

    pub fn state(&self) -> ToggleState {
        match (self.inner.state.get(), self.inner.state_changed.get()) {
            (false, false) => ToggleState::Off,
            (true, true) => ToggleState::TurnedOn,
            (true, false) => ToggleState::On,
            (false, true) => ToggleState::TurnedOff,
        }
    }
}

impl<S: InputSwitch> SwitchInteractive for ToggleSwitch<S> {
    type Error = S::Error;

    fn update(&self, time_passed_ms: usize) -> Result<(), Self::Error> {
        self.inner.update(time_passed_ms)?;
        Ok(())
    }

    fn is_interactive(&self) -> bool {
        self.inner.state_changed.get()
    }
}
