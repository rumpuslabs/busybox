#[doc = "Register `CON` reader"]
pub type R = crate::R<CON_SPEC>;
#[doc = "Field `RUN0` reader - Stops/starts timer channel 0."]
pub type RUN0_R = crate::BitReader<RUN0_A>;
#[doc = "Stops/starts timer channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN0_A {
    #[doc = "0: Stop."]
    STOP = 0,
    #[doc = "1: Run."]
    RUN = 1,
}
impl From<RUN0_A> for bool {
    #[inline(always)]
    fn from(variant: RUN0_A) -> Self {
        variant as u8 != 0
    }
}
impl RUN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RUN0_A {
        match self.bits {
            false => RUN0_A::STOP,
            true => RUN0_A::RUN,
        }
    }
    #[doc = "Stop."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RUN0_A::STOP
    }
    #[doc = "Run."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == RUN0_A::RUN
    }
}
#[doc = "Field `CENTER0` reader - Edge/center aligned operation for channel 0."]
pub type CENTER0_R = crate::BitReader<CENTER0_A>;
#[doc = "Edge/center aligned operation for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENTER0_A {
    #[doc = "0: Edge-aligned."]
    EDGE_ALIGNED = 0,
    #[doc = "1: Center-aligned."]
    CENTER_ALIGNED = 1,
}
impl From<CENTER0_A> for bool {
    #[inline(always)]
    fn from(variant: CENTER0_A) -> Self {
        variant as u8 != 0
    }
}
impl CENTER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CENTER0_A {
        match self.bits {
            false => CENTER0_A::EDGE_ALIGNED,
            true => CENTER0_A::CENTER_ALIGNED,
        }
    }
    #[doc = "Edge-aligned."]
    #[inline(always)]
    pub fn is_edge_aligned(&self) -> bool {
        *self == CENTER0_A::EDGE_ALIGNED
    }
    #[doc = "Center-aligned."]
    #[inline(always)]
    pub fn is_center_aligned(&self) -> bool {
        *self == CENTER0_A::CENTER_ALIGNED
    }
}
#[doc = "Field `POLA0` reader - Selects polarity of the MCOA0 and MCOB0 pins."]
pub type POLA0_R = crate::BitReader<POLA0_A>;
#[doc = "Selects polarity of the MCOA0 and MCOB0 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLA0_A {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    LOW = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    HIGH = 1,
}
impl From<POLA0_A> for bool {
    #[inline(always)]
    fn from(variant: POLA0_A) -> Self {
        variant as u8 != 0
    }
}
impl POLA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POLA0_A {
        match self.bits {
            false => POLA0_A::LOW,
            true => POLA0_A::HIGH,
        }
    }
    #[doc = "Passive state is LOW, active state is HIGH."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POLA0_A::LOW
    }
    #[doc = "Passive state is HIGH, active state is LOW."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POLA0_A::HIGH
    }
}
#[doc = "Field `DTE0` reader - Controls the dead-time feature for channel 0."]
pub type DTE0_R = crate::BitReader<DTE0_A>;
#[doc = "Controls the dead-time feature for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTE0_A {
    #[doc = "0: Dead-time disabled."]
    DEAD_TIME_DISABLED = 0,
    #[doc = "1: Dead-time enabled."]
    DEAD_TIME_ENABLED = 1,
}
impl From<DTE0_A> for bool {
    #[inline(always)]
    fn from(variant: DTE0_A) -> Self {
        variant as u8 != 0
    }
}
impl DTE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTE0_A {
        match self.bits {
            false => DTE0_A::DEAD_TIME_DISABLED,
            true => DTE0_A::DEAD_TIME_ENABLED,
        }
    }
    #[doc = "Dead-time disabled."]
    #[inline(always)]
    pub fn is_dead_time_disabled(&self) -> bool {
        *self == DTE0_A::DEAD_TIME_DISABLED
    }
    #[doc = "Dead-time enabled."]
    #[inline(always)]
    pub fn is_dead_time_enabled(&self) -> bool {
        *self == DTE0_A::DEAD_TIME_ENABLED
    }
}
#[doc = "Field `DISUP0` reader - Enable/disable updates of functional registers for channel 0 (see Section 24.8.2)."]
pub type DISUP0_R = crate::BitReader<DISUP0_A>;
#[doc = "Enable/disable updates of functional registers for channel 0 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISUP0_A {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    NOUPDATE = 1,
}
impl From<DISUP0_A> for bool {
    #[inline(always)]
    fn from(variant: DISUP0_A) -> Self {
        variant as u8 != 0
    }
}
impl DISUP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DISUP0_A {
        match self.bits {
            false => DISUP0_A::UPDATE,
            true => DISUP0_A::NOUPDATE,
        }
    }
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == DISUP0_A::UPDATE
    }
    #[doc = "Functional registers remain the same as long as the timer is running."]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == DISUP0_A::NOUPDATE
    }
}
#[doc = "Field `RUN1` reader - Stops/starts timer channel 1."]
pub type RUN1_R = crate::BitReader<RUN1_A>;
#[doc = "Stops/starts timer channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN1_A {
    #[doc = "0: Stop."]
    STOP = 0,
    #[doc = "1: Run."]
    RUN = 1,
}
impl From<RUN1_A> for bool {
    #[inline(always)]
    fn from(variant: RUN1_A) -> Self {
        variant as u8 != 0
    }
}
impl RUN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RUN1_A {
        match self.bits {
            false => RUN1_A::STOP,
            true => RUN1_A::RUN,
        }
    }
    #[doc = "Stop."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RUN1_A::STOP
    }
    #[doc = "Run."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == RUN1_A::RUN
    }
}
#[doc = "Field `CENTER1` reader - Edge/center aligned operation for channel 1."]
pub type CENTER1_R = crate::BitReader<CENTER1_A>;
#[doc = "Edge/center aligned operation for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENTER1_A {
    #[doc = "0: Edge-aligned."]
    EDGE_ALIGNED = 0,
    #[doc = "1: Center-aligned."]
    CENTER_ALIGNED = 1,
}
impl From<CENTER1_A> for bool {
    #[inline(always)]
    fn from(variant: CENTER1_A) -> Self {
        variant as u8 != 0
    }
}
impl CENTER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CENTER1_A {
        match self.bits {
            false => CENTER1_A::EDGE_ALIGNED,
            true => CENTER1_A::CENTER_ALIGNED,
        }
    }
    #[doc = "Edge-aligned."]
    #[inline(always)]
    pub fn is_edge_aligned(&self) -> bool {
        *self == CENTER1_A::EDGE_ALIGNED
    }
    #[doc = "Center-aligned."]
    #[inline(always)]
    pub fn is_center_aligned(&self) -> bool {
        *self == CENTER1_A::CENTER_ALIGNED
    }
}
#[doc = "Field `POLA1` reader - Selects polarity of the MCOA1 and MCOB1 pins."]
pub type POLA1_R = crate::BitReader<POLA1_A>;
#[doc = "Selects polarity of the MCOA1 and MCOB1 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLA1_A {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    LOW = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    HIGH = 1,
}
impl From<POLA1_A> for bool {
    #[inline(always)]
    fn from(variant: POLA1_A) -> Self {
        variant as u8 != 0
    }
}
impl POLA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POLA1_A {
        match self.bits {
            false => POLA1_A::LOW,
            true => POLA1_A::HIGH,
        }
    }
    #[doc = "Passive state is LOW, active state is HIGH."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POLA1_A::LOW
    }
    #[doc = "Passive state is HIGH, active state is LOW."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POLA1_A::HIGH
    }
}
#[doc = "Field `DTE1` reader - Controls the dead-time feature for channel 1."]
pub type DTE1_R = crate::BitReader<DTE1_A>;
#[doc = "Controls the dead-time feature for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTE1_A {
    #[doc = "0: Dead-time disabled."]
    DEAD_TIME_DISABLED = 0,
    #[doc = "1: Dead-time enabled."]
    DEAD_TIME_ENABLED = 1,
}
impl From<DTE1_A> for bool {
    #[inline(always)]
    fn from(variant: DTE1_A) -> Self {
        variant as u8 != 0
    }
}
impl DTE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTE1_A {
        match self.bits {
            false => DTE1_A::DEAD_TIME_DISABLED,
            true => DTE1_A::DEAD_TIME_ENABLED,
        }
    }
    #[doc = "Dead-time disabled."]
    #[inline(always)]
    pub fn is_dead_time_disabled(&self) -> bool {
        *self == DTE1_A::DEAD_TIME_DISABLED
    }
    #[doc = "Dead-time enabled."]
    #[inline(always)]
    pub fn is_dead_time_enabled(&self) -> bool {
        *self == DTE1_A::DEAD_TIME_ENABLED
    }
}
#[doc = "Field `DISUP1` reader - Enable/disable updates of functional registers for channel 1 (see Section 24.8.2)."]
pub type DISUP1_R = crate::BitReader<DISUP1_A>;
#[doc = "Enable/disable updates of functional registers for channel 1 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISUP1_A {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    NOUPDATE = 1,
}
impl From<DISUP1_A> for bool {
    #[inline(always)]
    fn from(variant: DISUP1_A) -> Self {
        variant as u8 != 0
    }
}
impl DISUP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DISUP1_A {
        match self.bits {
            false => DISUP1_A::UPDATE,
            true => DISUP1_A::NOUPDATE,
        }
    }
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == DISUP1_A::UPDATE
    }
    #[doc = "Functional registers remain the same as long as the timer is running."]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == DISUP1_A::NOUPDATE
    }
}
#[doc = "Field `RUN2` reader - Stops/starts timer channel 2."]
pub type RUN2_R = crate::BitReader<RUN2_A>;
#[doc = "Stops/starts timer channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN2_A {
    #[doc = "0: Stop."]
    STOP = 0,
    #[doc = "1: Run."]
    RUN = 1,
}
impl From<RUN2_A> for bool {
    #[inline(always)]
    fn from(variant: RUN2_A) -> Self {
        variant as u8 != 0
    }
}
impl RUN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RUN2_A {
        match self.bits {
            false => RUN2_A::STOP,
            true => RUN2_A::RUN,
        }
    }
    #[doc = "Stop."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RUN2_A::STOP
    }
    #[doc = "Run."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == RUN2_A::RUN
    }
}
#[doc = "Field `CENTER2` reader - Edge/center aligned operation for channel 2."]
pub type CENTER2_R = crate::BitReader<CENTER2_A>;
#[doc = "Edge/center aligned operation for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENTER2_A {
    #[doc = "0: Edge-aligned."]
    EDGE_ALIGNED = 0,
    #[doc = "1: Center-aligned."]
    CENTER_ALIGNED = 1,
}
impl From<CENTER2_A> for bool {
    #[inline(always)]
    fn from(variant: CENTER2_A) -> Self {
        variant as u8 != 0
    }
}
impl CENTER2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CENTER2_A {
        match self.bits {
            false => CENTER2_A::EDGE_ALIGNED,
            true => CENTER2_A::CENTER_ALIGNED,
        }
    }
    #[doc = "Edge-aligned."]
    #[inline(always)]
    pub fn is_edge_aligned(&self) -> bool {
        *self == CENTER2_A::EDGE_ALIGNED
    }
    #[doc = "Center-aligned."]
    #[inline(always)]
    pub fn is_center_aligned(&self) -> bool {
        *self == CENTER2_A::CENTER_ALIGNED
    }
}
#[doc = "Field `POLA2` reader - Selects polarity of the MCOA2 and MCOB2 pins."]
pub type POLA2_R = crate::BitReader<POLA2_A>;
#[doc = "Selects polarity of the MCOA2 and MCOB2 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLA2_A {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    LOW = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    HIGH = 1,
}
impl From<POLA2_A> for bool {
    #[inline(always)]
    fn from(variant: POLA2_A) -> Self {
        variant as u8 != 0
    }
}
impl POLA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POLA2_A {
        match self.bits {
            false => POLA2_A::LOW,
            true => POLA2_A::HIGH,
        }
    }
    #[doc = "Passive state is LOW, active state is HIGH."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POLA2_A::LOW
    }
    #[doc = "Passive state is HIGH, active state is LOW."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POLA2_A::HIGH
    }
}
#[doc = "Field `DTE2` reader - Controls the dead-time feature for channel 1."]
pub type DTE2_R = crate::BitReader<DTE2_A>;
#[doc = "Controls the dead-time feature for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTE2_A {
    #[doc = "0: Dead-time disabled."]
    DEAD_TIME_DISABLED = 0,
    #[doc = "1: Dead-time enabled."]
    DEAD_TIME_ENABLED = 1,
}
impl From<DTE2_A> for bool {
    #[inline(always)]
    fn from(variant: DTE2_A) -> Self {
        variant as u8 != 0
    }
}
impl DTE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTE2_A {
        match self.bits {
            false => DTE2_A::DEAD_TIME_DISABLED,
            true => DTE2_A::DEAD_TIME_ENABLED,
        }
    }
    #[doc = "Dead-time disabled."]
    #[inline(always)]
    pub fn is_dead_time_disabled(&self) -> bool {
        *self == DTE2_A::DEAD_TIME_DISABLED
    }
    #[doc = "Dead-time enabled."]
    #[inline(always)]
    pub fn is_dead_time_enabled(&self) -> bool {
        *self == DTE2_A::DEAD_TIME_ENABLED
    }
}
#[doc = "Field `DISUP2` reader - Enable/disable updates of functional registers for channel 2 (see Section 24.8.2)."]
pub type DISUP2_R = crate::BitReader<DISUP2_A>;
#[doc = "Enable/disable updates of functional registers for channel 2 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISUP2_A {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    NOUPDATE = 1,
}
impl From<DISUP2_A> for bool {
    #[inline(always)]
    fn from(variant: DISUP2_A) -> Self {
        variant as u8 != 0
    }
}
impl DISUP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DISUP2_A {
        match self.bits {
            false => DISUP2_A::UPDATE,
            true => DISUP2_A::NOUPDATE,
        }
    }
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == DISUP2_A::UPDATE
    }
    #[doc = "Functional registers remain the same as long as the timer is running."]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == DISUP2_A::NOUPDATE
    }
}
#[doc = "Field `INVBDC` reader - Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode."]
pub type INVBDC_R = crate::BitReader<INVBDC_A>;
#[doc = "Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVBDC_A {
    #[doc = "0: The MCOB outputs have opposite polarity from the MCOA outputs (aside from dead time)."]
    OPPOSITE = 0,
    #[doc = "1: The MCOB outputs have the same basic polarity as the MCOA outputs. (see Section 24.8.6)"]
    SAME = 1,
}
impl From<INVBDC_A> for bool {
    #[inline(always)]
    fn from(variant: INVBDC_A) -> Self {
        variant as u8 != 0
    }
}
impl INVBDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INVBDC_A {
        match self.bits {
            false => INVBDC_A::OPPOSITE,
            true => INVBDC_A::SAME,
        }
    }
    #[doc = "The MCOB outputs have opposite polarity from the MCOA outputs (aside from dead time)."]
    #[inline(always)]
    pub fn is_opposite(&self) -> bool {
        *self == INVBDC_A::OPPOSITE
    }
    #[doc = "The MCOB outputs have the same basic polarity as the MCOA outputs. (see Section 24.8.6)"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        *self == INVBDC_A::SAME
    }
}
#[doc = "Field `ACMODE` reader - 3-phase AC mode select (see Section 24.8.7)."]
pub type ACMODE_R = crate::BitReader<ACMODE_A>;
#[doc = "3-phase AC mode select (see Section 24.8.7).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMODE_A {
    #[doc = "0: 3-phase AC-mode off: Each PWM channel uses its own timer-counter and period register."]
    OFF = 0,
    #[doc = "1: 3-phase AC-mode on: All PWM channels use the timer-counter and period register of channel 0."]
    ON = 1,
}
impl From<ACMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ACMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACMODE_A {
        match self.bits {
            false => ACMODE_A::OFF,
            true => ACMODE_A::ON,
        }
    }
    #[doc = "3-phase AC-mode off: Each PWM channel uses its own timer-counter and period register."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ACMODE_A::OFF
    }
    #[doc = "3-phase AC-mode on: All PWM channels use the timer-counter and period register of channel 0."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == ACMODE_A::ON
    }
}
#[doc = "Field `DCMODE` reader - 3-phase DC mode select (see Section 24.8.6)."]
pub type DCMODE_R = crate::BitReader<DCMODE_A>;
#[doc = "3-phase DC mode select (see Section 24.8.6).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMODE_A {
    #[doc = "0: 3-phase DC mode off: PWM channels are independent (unless bit ACMODE = 1)"]
    OFF = 0,
    #[doc = "1: 3-phase DC mode on: The internal MCOA0 output is routed through the CP register (i.e. a mask) register to all six PWM outputs."]
    ON = 1,
}
impl From<DCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DCMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCMODE_A {
        match self.bits {
            false => DCMODE_A::OFF,
            true => DCMODE_A::ON,
        }
    }
    #[doc = "3-phase DC mode off: PWM channels are independent (unless bit ACMODE = 1)"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DCMODE_A::OFF
    }
    #[doc = "3-phase DC mode on: The internal MCOA0 output is routed through the CP register (i.e. a mask) register to all six PWM outputs."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == DCMODE_A::ON
    }
}
impl R {
    #[doc = "Bit 0 - Stops/starts timer channel 0."]
    #[inline(always)]
    pub fn run0(&self) -> RUN0_R {
        RUN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Edge/center aligned operation for channel 0."]
    #[inline(always)]
    pub fn center0(&self) -> CENTER0_R {
        CENTER0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects polarity of the MCOA0 and MCOB0 pins."]
    #[inline(always)]
    pub fn pola0(&self) -> POLA0_R {
        POLA0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls the dead-time feature for channel 0."]
    #[inline(always)]
    pub fn dte0(&self) -> DTE0_R {
        DTE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable/disable updates of functional registers for channel 0 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup0(&self) -> DISUP0_R {
        DISUP0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Stops/starts timer channel 1."]
    #[inline(always)]
    pub fn run1(&self) -> RUN1_R {
        RUN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Edge/center aligned operation for channel 1."]
    #[inline(always)]
    pub fn center1(&self) -> CENTER1_R {
        CENTER1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects polarity of the MCOA1 and MCOB1 pins."]
    #[inline(always)]
    pub fn pola1(&self) -> POLA1_R {
        POLA1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Controls the dead-time feature for channel 1."]
    #[inline(always)]
    pub fn dte1(&self) -> DTE1_R {
        DTE1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable/disable updates of functional registers for channel 1 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup1(&self) -> DISUP1_R {
        DISUP1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Stops/starts timer channel 2."]
    #[inline(always)]
    pub fn run2(&self) -> RUN2_R {
        RUN2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Edge/center aligned operation for channel 2."]
    #[inline(always)]
    pub fn center2(&self) -> CENTER2_R {
        CENTER2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Selects polarity of the MCOA2 and MCOB2 pins."]
    #[inline(always)]
    pub fn pola2(&self) -> POLA2_R {
        POLA2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Controls the dead-time feature for channel 1."]
    #[inline(always)]
    pub fn dte2(&self) -> DTE2_R {
        DTE2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable/disable updates of functional registers for channel 2 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup2(&self) -> DISUP2_R {
        DISUP2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode."]
    #[inline(always)]
    pub fn invbdc(&self) -> INVBDC_R {
        INVBDC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 3-phase AC mode select (see Section 24.8.7)."]
    #[inline(always)]
    pub fn acmode(&self) -> ACMODE_R {
        ACMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 3-phase DC mode select (see Section 24.8.6)."]
    #[inline(always)]
    pub fn dcmode(&self) -> DCMODE_R {
        DCMODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CON")
            .field("run0", &format_args!("{}", self.run0().bit()))
            .field("center0", &format_args!("{}", self.center0().bit()))
            .field("pola0", &format_args!("{}", self.pola0().bit()))
            .field("dte0", &format_args!("{}", self.dte0().bit()))
            .field("disup0", &format_args!("{}", self.disup0().bit()))
            .field("run1", &format_args!("{}", self.run1().bit()))
            .field("center1", &format_args!("{}", self.center1().bit()))
            .field("pola1", &format_args!("{}", self.pola1().bit()))
            .field("dte1", &format_args!("{}", self.dte1().bit()))
            .field("disup1", &format_args!("{}", self.disup1().bit()))
            .field("run2", &format_args!("{}", self.run2().bit()))
            .field("center2", &format_args!("{}", self.center2().bit()))
            .field("pola2", &format_args!("{}", self.pola2().bit()))
            .field("dte2", &format_args!("{}", self.dte2().bit()))
            .field("disup2", &format_args!("{}", self.disup2().bit()))
            .field("invbdc", &format_args!("{}", self.invbdc().bit()))
            .field("acmode", &format_args!("{}", self.acmode().bit()))
            .field("dcmode", &format_args!("{}", self.dcmode().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "PWM Control read address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`con::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CON_SPEC;
impl crate::RegisterSpec for CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`con::R`](R) reader structure"]
impl crate::Readable for CON_SPEC {}
#[doc = "`reset()` method sets CON to value 0"]
impl crate::Resettable for CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
