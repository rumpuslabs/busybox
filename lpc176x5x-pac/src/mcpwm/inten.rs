#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Field `ILIM0` reader - Limit interrupt for channel 0."]
pub type ILIM0_R = crate::BitReader<ILIM0_A>;
#[doc = "Limit interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILIM0_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ILIM0_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM0_A) -> Self {
        variant as u8 != 0
    }
}
impl ILIM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ILIM0_A {
        match self.bits {
            false => ILIM0_A::DISABLED,
            true => ILIM0_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ILIM0_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ILIM0_A::ENABLED
    }
}
#[doc = "Field `IMAT0` reader - Match interrupt for channel 0."]
pub type IMAT0_R = crate::BitReader<IMAT0_A>;
#[doc = "Match interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMAT0_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<IMAT0_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT0_A) -> Self {
        variant as u8 != 0
    }
}
impl IMAT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IMAT0_A {
        match self.bits {
            false => IMAT0_A::DISABLED,
            true => IMAT0_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IMAT0_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IMAT0_A::ENABLED
    }
}
#[doc = "Field `ICAP0` reader - Capture interrupt for channel 0."]
pub type ICAP0_R = crate::BitReader<ICAP0_A>;
#[doc = "Capture interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAP0_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ICAP0_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP0_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICAP0_A {
        match self.bits {
            false => ICAP0_A::DISABLED,
            true => ICAP0_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICAP0_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICAP0_A::ENABLED
    }
}
#[doc = "Field `ILIM1` reader - Limit interrupt for channel 1."]
pub type ILIM1_R = crate::BitReader<ILIM1_A>;
#[doc = "Limit interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILIM1_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ILIM1_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM1_A) -> Self {
        variant as u8 != 0
    }
}
impl ILIM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ILIM1_A {
        match self.bits {
            false => ILIM1_A::DISABLED,
            true => ILIM1_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ILIM1_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ILIM1_A::ENABLED
    }
}
#[doc = "Field `IMAT1` reader - Match interrupt for channel 1."]
pub type IMAT1_R = crate::BitReader<IMAT1_A>;
#[doc = "Match interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMAT1_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<IMAT1_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT1_A) -> Self {
        variant as u8 != 0
    }
}
impl IMAT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IMAT1_A {
        match self.bits {
            false => IMAT1_A::DISABLED,
            true => IMAT1_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IMAT1_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IMAT1_A::ENABLED
    }
}
#[doc = "Field `ICAP1` reader - Capture interrupt for channel 1."]
pub type ICAP1_R = crate::BitReader<ICAP1_A>;
#[doc = "Capture interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAP1_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ICAP1_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP1_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICAP1_A {
        match self.bits {
            false => ICAP1_A::DISABLED,
            true => ICAP1_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICAP1_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICAP1_A::ENABLED
    }
}
#[doc = "Field `ILIM2` reader - Limit interrupt for channel 2."]
pub type ILIM2_R = crate::BitReader<ILIM2_A>;
#[doc = "Limit interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILIM2_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ILIM2_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM2_A) -> Self {
        variant as u8 != 0
    }
}
impl ILIM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ILIM2_A {
        match self.bits {
            false => ILIM2_A::DISABLED,
            true => ILIM2_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ILIM2_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ILIM2_A::ENABLED
    }
}
#[doc = "Field `IMAT2` reader - Match interrupt for channel 2."]
pub type IMAT2_R = crate::BitReader<IMAT2_A>;
#[doc = "Match interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMAT2_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<IMAT2_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT2_A) -> Self {
        variant as u8 != 0
    }
}
impl IMAT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IMAT2_A {
        match self.bits {
            false => IMAT2_A::DISABLED,
            true => IMAT2_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IMAT2_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IMAT2_A::ENABLED
    }
}
#[doc = "Field `ICAP2` reader - Capture interrupt for channel 2."]
pub type ICAP2_R = crate::BitReader<ICAP2_A>;
#[doc = "Capture interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAP2_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ICAP2_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP2_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICAP2_A {
        match self.bits {
            false => ICAP2_A::DISABLED,
            true => ICAP2_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICAP2_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICAP2_A::ENABLED
    }
}
#[doc = "Field `ABORT` reader - Fast abort interrupt."]
pub type ABORT_R = crate::BitReader<ABORT_A>;
#[doc = "Fast abort interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_A) -> Self {
        variant as u8 != 0
    }
}
impl ABORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABORT_A {
        match self.bits {
            false => ABORT_A::DISABLED,
            true => ABORT_A::ENABLED,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABORT_A::DISABLED
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABORT_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 0 - Limit interrupt for channel 0."]
    #[inline(always)]
    pub fn ilim0(&self) -> ILIM0_R {
        ILIM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Match interrupt for channel 0."]
    #[inline(always)]
    pub fn imat0(&self) -> IMAT0_R {
        IMAT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture interrupt for channel 0."]
    #[inline(always)]
    pub fn icap0(&self) -> ICAP0_R {
        ICAP0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Limit interrupt for channel 1."]
    #[inline(always)]
    pub fn ilim1(&self) -> ILIM1_R {
        ILIM1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Match interrupt for channel 1."]
    #[inline(always)]
    pub fn imat1(&self) -> IMAT1_R {
        IMAT1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture interrupt for channel 1."]
    #[inline(always)]
    pub fn icap1(&self) -> ICAP1_R {
        ICAP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Limit interrupt for channel 2."]
    #[inline(always)]
    pub fn ilim2(&self) -> ILIM2_R {
        ILIM2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Match interrupt for channel 2."]
    #[inline(always)]
    pub fn imat2(&self) -> IMAT2_R {
        IMAT2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture interrupt for channel 2."]
    #[inline(always)]
    pub fn icap2(&self) -> ICAP2_R {
        ICAP2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast abort interrupt."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("ilim0", &format_args!("{}", self.ilim0().bit()))
            .field("imat0", &format_args!("{}", self.imat0().bit()))
            .field("icap0", &format_args!("{}", self.icap0().bit()))
            .field("ilim1", &format_args!("{}", self.ilim1().bit()))
            .field("imat1", &format_args!("{}", self.imat1().bit()))
            .field("icap1", &format_args!("{}", self.icap1().bit()))
            .field("ilim2", &format_args!("{}", self.ilim2().bit()))
            .field("imat2", &format_args!("{}", self.imat2().bit()))
            .field("icap2", &format_args!("{}", self.icap2().bit()))
            .field("abort", &format_args!("{}", self.abort().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt Enable read address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
