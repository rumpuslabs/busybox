#[doc = "Register `CNTCON` reader"]
pub type R = crate::R<CNTCON_SPEC>;
#[doc = "Field `TC0MCI0_RE` reader - Counter 0 rising edge mode, channel 0."]
pub type TC0MCI0_RE_R = crate::BitReader<TC0MCI0_RE_A>;
#[doc = "Counter 0 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC0MCI0_RE_A {
    #[doc = "0: A rising edge on MCI0 does not affect counter 0."]
    DISABLED = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI0."]
    RISING = 1,
}
impl From<TC0MCI0_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI0_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC0MCI0_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC0MCI0_RE_A {
        match self.bits {
            false => TC0MCI0_RE_A::DISABLED,
            true => TC0MCI0_RE_A::RISING,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC0MCI0_RE_A::DISABLED
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI0."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI0_RE_A::RISING
    }
}
#[doc = "Field `TC0MCI0_FE` reader - Counter 0 falling edge mode, channel 0."]
pub type TC0MCI0_FE_R = crate::BitReader<TC0MCI0_FE_A>;
#[doc = "Counter 0 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC0MCI0_FE_A {
    #[doc = "0: A falling edge on MCI0 does not affect counter 0."]
    DISABLED = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI0."]
    FALLING = 1,
}
impl From<TC0MCI0_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI0_FE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC0MCI0_FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC0MCI0_FE_A {
        match self.bits {
            false => TC0MCI0_FE_A::DISABLED,
            true => TC0MCI0_FE_A::FALLING,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC0MCI0_FE_A::DISABLED
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI0."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC0MCI0_FE_A::FALLING
    }
}
#[doc = "Field `TC0MCI1_RE` reader - Counter 0 rising edge mode, channel 1."]
pub type TC0MCI1_RE_R = crate::BitReader<TC0MCI1_RE_A>;
#[doc = "Counter 0 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC0MCI1_RE_A {
    #[doc = "0: A rising edge on MCI1 does not affect counter 0."]
    DISABLED = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI1."]
    RISING = 1,
}
impl From<TC0MCI1_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI1_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC0MCI1_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC0MCI1_RE_A {
        match self.bits {
            false => TC0MCI1_RE_A::DISABLED,
            true => TC0MCI1_RE_A::RISING,
        }
    }
    #[doc = "A rising edge on MCI1 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC0MCI1_RE_A::DISABLED
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI1."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI1_RE_A::RISING
    }
}
#[doc = "Field `TC0MCI1_FE` reader - Counter 0 falling edge mode, channel 1."]
pub type TC0MCI1_FE_R = crate::BitReader<TC0MCI1_FE_A>;
#[doc = "Counter 0 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC0MCI1_FE_A {
    #[doc = "0: A falling edge on MCI1 does not affect counter 0."]
    DISABLED = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI1."]
    FALLING = 1,
}
impl From<TC0MCI1_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI1_FE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC0MCI1_FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC0MCI1_FE_A {
        match self.bits {
            false => TC0MCI1_FE_A::DISABLED,
            true => TC0MCI1_FE_A::FALLING,
        }
    }
    #[doc = "A falling edge on MCI1 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC0MCI1_FE_A::DISABLED
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI1."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC0MCI1_FE_A::FALLING
    }
}
#[doc = "Field `TC0MCI2_RE` reader - Counter 0 rising edge mode, channel 2."]
pub type TC0MCI2_RE_R = crate::BitReader<TC0MCI2_RE_A>;
#[doc = "Counter 0 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC0MCI2_RE_A {
    #[doc = "0: A rising edge on MCI0 does not affect counter 0."]
    DISABLED = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI2."]
    RISING = 1,
}
impl From<TC0MCI2_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI2_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC0MCI2_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC0MCI2_RE_A {
        match self.bits {
            false => TC0MCI2_RE_A::DISABLED,
            true => TC0MCI2_RE_A::RISING,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC0MCI2_RE_A::DISABLED
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI2."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI2_RE_A::RISING
    }
}
#[doc = "Field `TC0MCI2_FE` reader - Counter 0 falling edge mode, channel 2."]
pub type TC0MCI2_FE_R = crate::BitReader<TC0MCI2_FE_A>;
#[doc = "Counter 0 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC0MCI2_FE_A {
    #[doc = "0: A falling edge on MCI0 does not affect counter 0."]
    DISABLED = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI2."]
    FALLLING = 1,
}
impl From<TC0MCI2_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI2_FE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC0MCI2_FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC0MCI2_FE_A {
        match self.bits {
            false => TC0MCI2_FE_A::DISABLED,
            true => TC0MCI2_FE_A::FALLLING,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC0MCI2_FE_A::DISABLED
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI2."]
    #[inline(always)]
    pub fn is_fallling(&self) -> bool {
        *self == TC0MCI2_FE_A::FALLLING
    }
}
#[doc = "Field `TC1MCI0_RE` reader - Counter 1 rising edge mode, channel 0."]
pub type TC1MCI0_RE_R = crate::BitReader<TC1MCI0_RE_A>;
#[doc = "Counter 1 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC1MCI0_RE_A {
    #[doc = "0: A rising edge on MCI0 does not affect counter 1."]
    DISABLED = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI0."]
    RISING = 1,
}
impl From<TC1MCI0_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI0_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC1MCI0_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC1MCI0_RE_A {
        match self.bits {
            false => TC1MCI0_RE_A::DISABLED,
            true => TC1MCI0_RE_A::RISING,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC1MCI0_RE_A::DISABLED
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI0."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI0_RE_A::RISING
    }
}
#[doc = "Field `TC1MCI0_FE` reader - Counter 1 falling edge mode, channel 0."]
pub type TC1MCI0_FE_R = crate::BitReader<TC1MCI0_FE_A>;
#[doc = "Counter 1 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC1MCI0_FE_A {
    #[doc = "0: A falling edge on MCI0 does not affect counter 1."]
    DISABLED = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI0."]
    FALLING = 1,
}
impl From<TC1MCI0_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI0_FE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC1MCI0_FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC1MCI0_FE_A {
        match self.bits {
            false => TC1MCI0_FE_A::DISABLED,
            true => TC1MCI0_FE_A::FALLING,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC1MCI0_FE_A::DISABLED
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI0."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI0_FE_A::FALLING
    }
}
#[doc = "Field `TC1MCI1_RE` reader - Counter 1 rising edge mode, channel 1."]
pub type TC1MCI1_RE_R = crate::BitReader<TC1MCI1_RE_A>;
#[doc = "Counter 1 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC1MCI1_RE_A {
    #[doc = "0: A rising edge on MCI1 does not affect counter 1."]
    DISABLED = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI1."]
    RISING = 1,
}
impl From<TC1MCI1_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI1_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC1MCI1_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC1MCI1_RE_A {
        match self.bits {
            false => TC1MCI1_RE_A::DISABLED,
            true => TC1MCI1_RE_A::RISING,
        }
    }
    #[doc = "A rising edge on MCI1 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC1MCI1_RE_A::DISABLED
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI1."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI1_RE_A::RISING
    }
}
#[doc = "Field `TC1MCI1_FE` reader - Counter 1 falling edge mode, channel 1."]
pub type TC1MCI1_FE_R = crate::BitReader<TC1MCI1_FE_A>;
#[doc = "Counter 1 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC1MCI1_FE_A {
    #[doc = "0: A falling edge on MCI0 does not affect counter 1."]
    DISABLED = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI1."]
    FALLING = 1,
}
impl From<TC1MCI1_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI1_FE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC1MCI1_FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC1MCI1_FE_A {
        match self.bits {
            false => TC1MCI1_FE_A::DISABLED,
            true => TC1MCI1_FE_A::FALLING,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC1MCI1_FE_A::DISABLED
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI1."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI1_FE_A::FALLING
    }
}
#[doc = "Field `TC1MCI2_RE` reader - Counter 1 rising edge mode, channel 2."]
pub type TC1MCI2_RE_R = crate::BitReader<TC1MCI2_RE_A>;
#[doc = "Counter 1 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC1MCI2_RE_A {
    #[doc = "0: A rising edge on MCI2 does not affect counter 1."]
    DISABLED = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI2."]
    RISING = 1,
}
impl From<TC1MCI2_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI2_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC1MCI2_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC1MCI2_RE_A {
        match self.bits {
            false => TC1MCI2_RE_A::DISABLED,
            true => TC1MCI2_RE_A::RISING,
        }
    }
    #[doc = "A rising edge on MCI2 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC1MCI2_RE_A::DISABLED
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI2."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI2_RE_A::RISING
    }
}
#[doc = "Field `TC1MCI2_FE` reader - Counter 1 falling edge mode, channel 2."]
pub type TC1MCI2_FE_R = crate::BitReader<TC1MCI2_FE_A>;
#[doc = "Counter 1 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC1MCI2_FE_A {
    #[doc = "0: A falling edge on MCI2 does not affect counter 1."]
    DISABLED = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI2."]
    FALLING = 1,
}
impl From<TC1MCI2_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI2_FE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC1MCI2_FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC1MCI2_FE_A {
        match self.bits {
            false => TC1MCI2_FE_A::DISABLED,
            true => TC1MCI2_FE_A::FALLING,
        }
    }
    #[doc = "A falling edge on MCI2 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC1MCI2_FE_A::DISABLED
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI2."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI2_FE_A::FALLING
    }
}
#[doc = "Field `TC2MCI0_RE` reader - Counter 2 rising edge mode, channel 0."]
pub type TC2MCI0_RE_R = crate::BitReader<TC2MCI0_RE_A>;
#[doc = "Counter 2 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC2MCI0_RE_A {
    #[doc = "0: A rising edge on MCI0 does not affect counter 2."]
    DISABLED = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI0."]
    RISING = 1,
}
impl From<TC2MCI0_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI0_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC2MCI0_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC2MCI0_RE_A {
        match self.bits {
            false => TC2MCI0_RE_A::DISABLED,
            true => TC2MCI0_RE_A::RISING,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC2MCI0_RE_A::DISABLED
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI0."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC2MCI0_RE_A::RISING
    }
}
#[doc = "Field `TC2MCI0_FE` reader - Counter 2 falling edge mode, channel 0."]
pub type TC2MCI0_FE_R = crate::BitReader<TC2MCI0_FE_A>;
#[doc = "Counter 2 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC2MCI0_FE_A {
    #[doc = "0: A falling edge on MCI0 does not affect counter 2."]
    DISABLED = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI0."]
    FALLING = 1,
}
impl From<TC2MCI0_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI0_FE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC2MCI0_FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC2MCI0_FE_A {
        match self.bits {
            false => TC2MCI0_FE_A::DISABLED,
            true => TC2MCI0_FE_A::FALLING,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC2MCI0_FE_A::DISABLED
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI0."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI0_FE_A::FALLING
    }
}
#[doc = "Field `TC2MCI1_RE` reader - Counter 2 rising edge mode, channel 1."]
pub type TC2MCI1_RE_R = crate::BitReader<TC2MCI1_RE_A>;
#[doc = "Counter 2 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC2MCI1_RE_A {
    #[doc = "0: A rising edge on MCI1 does not affect counter 2."]
    DISABLED = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI1."]
    RISING = 1,
}
impl From<TC2MCI1_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI1_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC2MCI1_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC2MCI1_RE_A {
        match self.bits {
            false => TC2MCI1_RE_A::DISABLED,
            true => TC2MCI1_RE_A::RISING,
        }
    }
    #[doc = "A rising edge on MCI1 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC2MCI1_RE_A::DISABLED
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI1."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC2MCI1_RE_A::RISING
    }
}
#[doc = "Field `TC2MCI1_FE` reader - Counter 2 falling edge mode, channel 1."]
pub type TC2MCI1_FE_R = crate::BitReader<TC2MCI1_FE_A>;
#[doc = "Counter 2 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC2MCI1_FE_A {
    #[doc = "0: A falling edge on MCI1 does not affect counter 2."]
    DISABLED = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI1."]
    FALLING = 1,
}
impl From<TC2MCI1_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI1_FE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC2MCI1_FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC2MCI1_FE_A {
        match self.bits {
            false => TC2MCI1_FE_A::DISABLED,
            true => TC2MCI1_FE_A::FALLING,
        }
    }
    #[doc = "A falling edge on MCI1 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC2MCI1_FE_A::DISABLED
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI1."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI1_FE_A::FALLING
    }
}
#[doc = "Field `TC2MCI2_RE` reader - Counter 2 rising edge mode, channel 2."]
pub type TC2MCI2_RE_R = crate::BitReader<TC2MCI2_RE_A>;
#[doc = "Counter 2 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC2MCI2_RE_A {
    #[doc = "0: A rising edge on MCI2 does not affect counter 2."]
    DISABLED = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI2."]
    RISIING = 1,
}
impl From<TC2MCI2_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI2_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC2MCI2_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC2MCI2_RE_A {
        match self.bits {
            false => TC2MCI2_RE_A::DISABLED,
            true => TC2MCI2_RE_A::RISIING,
        }
    }
    #[doc = "A rising edge on MCI2 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC2MCI2_RE_A::DISABLED
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI2."]
    #[inline(always)]
    pub fn is_risiing(&self) -> bool {
        *self == TC2MCI2_RE_A::RISIING
    }
}
#[doc = "Field `TC2MCI2_FE` reader - Counter 2 falling edge mode, channel 2."]
pub type TC2MCI2_FE_R = crate::BitReader<TC2MCI2_FE_A>;
#[doc = "Counter 2 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC2MCI2_FE_A {
    #[doc = "0: A falling edge on MCI2 does not affect counter 2."]
    DISABLED = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI2."]
    FALLING = 1,
}
impl From<TC2MCI2_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI2_FE_A) -> Self {
        variant as u8 != 0
    }
}
impl TC2MCI2_FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC2MCI2_FE_A {
        match self.bits {
            false => TC2MCI2_FE_A::DISABLED,
            true => TC2MCI2_FE_A::FALLING,
        }
    }
    #[doc = "A falling edge on MCI2 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TC2MCI2_FE_A::DISABLED
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI2."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI2_FE_A::FALLING
    }
}
#[doc = "Field `CNTR0` reader - Channel 0 counter/timer mode."]
pub type CNTR0_R = crate::BitReader<CNTR0_A>;
#[doc = "Channel 0 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTR0_A {
    #[doc = "0: Channel 0 is in timer mode."]
    TIMER_MODE = 0,
    #[doc = "1: Channel 0 is in counter mode."]
    COUNTER_MODE = 1,
}
impl From<CNTR0_A> for bool {
    #[inline(always)]
    fn from(variant: CNTR0_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNTR0_A {
        match self.bits {
            false => CNTR0_A::TIMER_MODE,
            true => CNTR0_A::COUNTER_MODE,
        }
    }
    #[doc = "Channel 0 is in timer mode."]
    #[inline(always)]
    pub fn is_timer_mode(&self) -> bool {
        *self == CNTR0_A::TIMER_MODE
    }
    #[doc = "Channel 0 is in counter mode."]
    #[inline(always)]
    pub fn is_counter_mode(&self) -> bool {
        *self == CNTR0_A::COUNTER_MODE
    }
}
#[doc = "Field `CNTR1` reader - Channel 1 counter/timer mode."]
pub type CNTR1_R = crate::BitReader<CNTR1_A>;
#[doc = "Channel 1 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTR1_A {
    #[doc = "0: Channel 1 is in timer mode."]
    TIMER_MODE = 0,
    #[doc = "1: Channel 1 is in counter mode."]
    COUNTER_MODE = 1,
}
impl From<CNTR1_A> for bool {
    #[inline(always)]
    fn from(variant: CNTR1_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNTR1_A {
        match self.bits {
            false => CNTR1_A::TIMER_MODE,
            true => CNTR1_A::COUNTER_MODE,
        }
    }
    #[doc = "Channel 1 is in timer mode."]
    #[inline(always)]
    pub fn is_timer_mode(&self) -> bool {
        *self == CNTR1_A::TIMER_MODE
    }
    #[doc = "Channel 1 is in counter mode."]
    #[inline(always)]
    pub fn is_counter_mode(&self) -> bool {
        *self == CNTR1_A::COUNTER_MODE
    }
}
#[doc = "Field `CNTR2` reader - Channel 2 counter/timer mode."]
pub type CNTR2_R = crate::BitReader<CNTR2_A>;
#[doc = "Channel 2 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTR2_A {
    #[doc = "0: Channel 2 is in timer mode."]
    TIMER_MODE = 0,
    #[doc = "1: Channel 2 is in counter mode."]
    COUNTER_MODE = 1,
}
impl From<CNTR2_A> for bool {
    #[inline(always)]
    fn from(variant: CNTR2_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNTR2_A {
        match self.bits {
            false => CNTR2_A::TIMER_MODE,
            true => CNTR2_A::COUNTER_MODE,
        }
    }
    #[doc = "Channel 2 is in timer mode."]
    #[inline(always)]
    pub fn is_timer_mode(&self) -> bool {
        *self == CNTR2_A::TIMER_MODE
    }
    #[doc = "Channel 2 is in counter mode."]
    #[inline(always)]
    pub fn is_counter_mode(&self) -> bool {
        *self == CNTR2_A::COUNTER_MODE
    }
}
impl R {
    #[doc = "Bit 0 - Counter 0 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc0mci0_re(&self) -> TC0MCI0_RE_R {
        TC0MCI0_RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter 0 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc0mci0_fe(&self) -> TC0MCI0_FE_R {
        TC0MCI0_FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter 0 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc0mci1_re(&self) -> TC0MCI1_RE_R {
        TC0MCI1_RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter 0 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc0mci1_fe(&self) -> TC0MCI1_FE_R {
        TC0MCI1_FE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counter 0 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc0mci2_re(&self) -> TC0MCI2_RE_R {
        TC0MCI2_RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter 0 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc0mci2_fe(&self) -> TC0MCI2_FE_R {
        TC0MCI2_FE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter 1 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc1mci0_re(&self) -> TC1MCI0_RE_R {
        TC1MCI0_RE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter 1 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc1mci0_fe(&self) -> TC1MCI0_FE_R {
        TC1MCI0_FE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Counter 1 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc1mci1_re(&self) -> TC1MCI1_RE_R {
        TC1MCI1_RE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Counter 1 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc1mci1_fe(&self) -> TC1MCI1_FE_R {
        TC1MCI1_FE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Counter 1 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc1mci2_re(&self) -> TC1MCI2_RE_R {
        TC1MCI2_RE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Counter 1 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc1mci2_fe(&self) -> TC1MCI2_FE_R {
        TC1MCI2_FE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Counter 2 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc2mci0_re(&self) -> TC2MCI0_RE_R {
        TC2MCI0_RE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Counter 2 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc2mci0_fe(&self) -> TC2MCI0_FE_R {
        TC2MCI0_FE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Counter 2 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc2mci1_re(&self) -> TC2MCI1_RE_R {
        TC2MCI1_RE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Counter 2 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc2mci1_fe(&self) -> TC2MCI1_FE_R {
        TC2MCI1_FE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Counter 2 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc2mci2_re(&self) -> TC2MCI2_RE_R {
        TC2MCI2_RE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Counter 2 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc2mci2_fe(&self) -> TC2MCI2_FE_R {
        TC2MCI2_FE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel 0 counter/timer mode."]
    #[inline(always)]
    pub fn cntr0(&self) -> CNTR0_R {
        CNTR0_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel 1 counter/timer mode."]
    #[inline(always)]
    pub fn cntr1(&self) -> CNTR1_R {
        CNTR1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel 2 counter/timer mode."]
    #[inline(always)]
    pub fn cntr2(&self) -> CNTR2_R {
        CNTR2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTCON")
            .field("tc0mci0_re", &format_args!("{}", self.tc0mci0_re().bit()))
            .field("tc0mci0_fe", &format_args!("{}", self.tc0mci0_fe().bit()))
            .field("tc0mci1_re", &format_args!("{}", self.tc0mci1_re().bit()))
            .field("tc0mci1_fe", &format_args!("{}", self.tc0mci1_fe().bit()))
            .field("tc0mci2_re", &format_args!("{}", self.tc0mci2_re().bit()))
            .field("tc0mci2_fe", &format_args!("{}", self.tc0mci2_fe().bit()))
            .field("tc1mci0_re", &format_args!("{}", self.tc1mci0_re().bit()))
            .field("tc1mci0_fe", &format_args!("{}", self.tc1mci0_fe().bit()))
            .field("tc1mci1_re", &format_args!("{}", self.tc1mci1_re().bit()))
            .field("tc1mci1_fe", &format_args!("{}", self.tc1mci1_fe().bit()))
            .field("tc1mci2_re", &format_args!("{}", self.tc1mci2_re().bit()))
            .field("tc1mci2_fe", &format_args!("{}", self.tc1mci2_fe().bit()))
            .field("tc2mci0_re", &format_args!("{}", self.tc2mci0_re().bit()))
            .field("tc2mci0_fe", &format_args!("{}", self.tc2mci0_fe().bit()))
            .field("tc2mci1_re", &format_args!("{}", self.tc2mci1_re().bit()))
            .field("tc2mci1_fe", &format_args!("{}", self.tc2mci1_fe().bit()))
            .field("tc2mci2_re", &format_args!("{}", self.tc2mci2_re().bit()))
            .field("tc2mci2_fe", &format_args!("{}", self.tc2mci2_fe().bit()))
            .field("cntr0", &format_args!("{}", self.cntr0().bit()))
            .field("cntr1", &format_args!("{}", self.cntr1().bit()))
            .field("cntr2", &format_args!("{}", self.cntr2().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CNTCON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count Control read address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntcon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTCON_SPEC;
impl crate::RegisterSpec for CNTCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntcon::R`](R) reader structure"]
impl crate::Readable for CNTCON_SPEC {}
#[doc = "`reset()` method sets CNTCON to value 0"]
impl crate::Resettable for CNTCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
