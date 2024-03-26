#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Field `ILIM0_F` reader - Limit interrupt flag for channel 0."]
pub type ILIM0_F_R = crate::BitReader<ILIM0_F_A>;
#[doc = "Limit interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILIM0_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    NOT_PENDING = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    PENDING = 1,
}
impl From<ILIM0_F_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM0_F_A) -> Self {
        variant as u8 != 0
    }
}
impl ILIM0_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ILIM0_F_A {
        match self.bits {
            false => ILIM0_F_A::NOT_PENDING,
            true => ILIM0_F_A::PENDING,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ILIM0_F_A::NOT_PENDING
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ILIM0_F_A::PENDING
    }
}
#[doc = "Field `IMAT0_F` reader - Match interrupt flag for channel 0."]
pub type IMAT0_F_R = crate::BitReader<IMAT0_F_A>;
#[doc = "Match interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMAT0_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    NOT_PENDING = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    PENDING = 1,
}
impl From<IMAT0_F_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT0_F_A) -> Self {
        variant as u8 != 0
    }
}
impl IMAT0_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IMAT0_F_A {
        match self.bits {
            false => IMAT0_F_A::NOT_PENDING,
            true => IMAT0_F_A::PENDING,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == IMAT0_F_A::NOT_PENDING
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IMAT0_F_A::PENDING
    }
}
#[doc = "Field `ICAP0_F` reader - Capture interrupt flag for channel 0."]
pub type ICAP0_F_R = crate::BitReader<ICAP0_F_A>;
#[doc = "Capture interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAP0_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    NOT_PENDING = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    PENDING = 1,
}
impl From<ICAP0_F_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP0_F_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAP0_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICAP0_F_A {
        match self.bits {
            false => ICAP0_F_A::NOT_PENDING,
            true => ICAP0_F_A::PENDING,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ICAP0_F_A::NOT_PENDING
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ICAP0_F_A::PENDING
    }
}
#[doc = "Field `ILIM1_F` reader - Limit interrupt flag for channel 1."]
pub type ILIM1_F_R = crate::BitReader<ILIM1_F_A>;
#[doc = "Limit interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILIM1_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    NOT_PENDING = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    PENDING = 1,
}
impl From<ILIM1_F_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM1_F_A) -> Self {
        variant as u8 != 0
    }
}
impl ILIM1_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ILIM1_F_A {
        match self.bits {
            false => ILIM1_F_A::NOT_PENDING,
            true => ILIM1_F_A::PENDING,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ILIM1_F_A::NOT_PENDING
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ILIM1_F_A::PENDING
    }
}
#[doc = "Field `IMAT1_F` reader - Match interrupt flag for channel 1."]
pub type IMAT1_F_R = crate::BitReader<IMAT1_F_A>;
#[doc = "Match interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMAT1_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    NOT_PENDING = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    PENDING = 1,
}
impl From<IMAT1_F_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT1_F_A) -> Self {
        variant as u8 != 0
    }
}
impl IMAT1_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IMAT1_F_A {
        match self.bits {
            false => IMAT1_F_A::NOT_PENDING,
            true => IMAT1_F_A::PENDING,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == IMAT1_F_A::NOT_PENDING
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IMAT1_F_A::PENDING
    }
}
#[doc = "Field `ICAP1_F` reader - Capture interrupt flag for channel 1."]
pub type ICAP1_F_R = crate::BitReader<ICAP1_F_A>;
#[doc = "Capture interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAP1_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    NOT_PENDING = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    PENDING = 1,
}
impl From<ICAP1_F_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP1_F_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAP1_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICAP1_F_A {
        match self.bits {
            false => ICAP1_F_A::NOT_PENDING,
            true => ICAP1_F_A::PENDING,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ICAP1_F_A::NOT_PENDING
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ICAP1_F_A::PENDING
    }
}
#[doc = "Field `ILIM2_F` reader - Limit interrupt flag for channel 2."]
pub type ILIM2_F_R = crate::BitReader<ILIM2_F_A>;
#[doc = "Limit interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILIM2_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    NOT_PENDING = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    PENDING = 1,
}
impl From<ILIM2_F_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM2_F_A) -> Self {
        variant as u8 != 0
    }
}
impl ILIM2_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ILIM2_F_A {
        match self.bits {
            false => ILIM2_F_A::NOT_PENDING,
            true => ILIM2_F_A::PENDING,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ILIM2_F_A::NOT_PENDING
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ILIM2_F_A::PENDING
    }
}
#[doc = "Field `IMAT2_F` reader - Match interrupt flag for channel 2."]
pub type IMAT2_F_R = crate::BitReader<IMAT2_F_A>;
#[doc = "Match interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMAT2_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    NOT_PENDING = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    PENDING = 1,
}
impl From<IMAT2_F_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT2_F_A) -> Self {
        variant as u8 != 0
    }
}
impl IMAT2_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IMAT2_F_A {
        match self.bits {
            false => IMAT2_F_A::NOT_PENDING,
            true => IMAT2_F_A::PENDING,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == IMAT2_F_A::NOT_PENDING
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IMAT2_F_A::PENDING
    }
}
#[doc = "Field `ICAP2_F` reader - Capture interrupt flag for channel 2."]
pub type ICAP2_F_R = crate::BitReader<ICAP2_F_A>;
#[doc = "Capture interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICAP2_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    NOT_PENDING = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    PENDING = 1,
}
impl From<ICAP2_F_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP2_F_A) -> Self {
        variant as u8 != 0
    }
}
impl ICAP2_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICAP2_F_A {
        match self.bits {
            false => ICAP2_F_A::NOT_PENDING,
            true => ICAP2_F_A::PENDING,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ICAP2_F_A::NOT_PENDING
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ICAP2_F_A::PENDING
    }
}
#[doc = "Field `ABORT_F` reader - Fast abort interrupt flag."]
pub type ABORT_F_R = crate::BitReader<ABORT_F_A>;
#[doc = "Fast abort interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    NOT_PENDING = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    PENDING = 1,
}
impl From<ABORT_F_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_F_A) -> Self {
        variant as u8 != 0
    }
}
impl ABORT_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABORT_F_A {
        match self.bits {
            false => ABORT_F_A::NOT_PENDING,
            true => ABORT_F_A::PENDING,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ABORT_F_A::NOT_PENDING
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ABORT_F_A::PENDING
    }
}
impl R {
    #[doc = "Bit 0 - Limit interrupt flag for channel 0."]
    #[inline(always)]
    pub fn ilim0_f(&self) -> ILIM0_F_R {
        ILIM0_F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Match interrupt flag for channel 0."]
    #[inline(always)]
    pub fn imat0_f(&self) -> IMAT0_F_R {
        IMAT0_F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture interrupt flag for channel 0."]
    #[inline(always)]
    pub fn icap0_f(&self) -> ICAP0_F_R {
        ICAP0_F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Limit interrupt flag for channel 1."]
    #[inline(always)]
    pub fn ilim1_f(&self) -> ILIM1_F_R {
        ILIM1_F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Match interrupt flag for channel 1."]
    #[inline(always)]
    pub fn imat1_f(&self) -> IMAT1_F_R {
        IMAT1_F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture interrupt flag for channel 1."]
    #[inline(always)]
    pub fn icap1_f(&self) -> ICAP1_F_R {
        ICAP1_F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Limit interrupt flag for channel 2."]
    #[inline(always)]
    pub fn ilim2_f(&self) -> ILIM2_F_R {
        ILIM2_F_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Match interrupt flag for channel 2."]
    #[inline(always)]
    pub fn imat2_f(&self) -> IMAT2_F_R {
        IMAT2_F_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture interrupt flag for channel 2."]
    #[inline(always)]
    pub fn icap2_f(&self) -> ICAP2_F_R {
        ICAP2_F_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast abort interrupt flag."]
    #[inline(always)]
    pub fn abort_f(&self) -> ABORT_F_R {
        ABORT_F_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTF")
            .field("ilim0_f", &format_args!("{}", self.ilim0_f().bit()))
            .field("imat0_f", &format_args!("{}", self.imat0_f().bit()))
            .field("icap0_f", &format_args!("{}", self.icap0_f().bit()))
            .field("ilim1_f", &format_args!("{}", self.ilim1_f().bit()))
            .field("imat1_f", &format_args!("{}", self.imat1_f().bit()))
            .field("icap1_f", &format_args!("{}", self.icap1_f().bit()))
            .field("ilim2_f", &format_args!("{}", self.ilim2_f().bit()))
            .field("imat2_f", &format_args!("{}", self.imat2_f().bit()))
            .field("icap2_f", &format_args!("{}", self.icap2_f().bit()))
            .field("abort_f", &format_args!("{}", self.abort_f().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt flags read address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for INTF_SPEC {}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
