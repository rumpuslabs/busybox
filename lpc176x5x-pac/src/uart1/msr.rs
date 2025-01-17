#[doc = "Register `MSR` reader"]
pub type R = crate::R<MSR_SPEC>;
#[doc = "Field `DCTS` reader - Delta CTS. Set upon state change of input CTS. Cleared on an MSR read."]
pub type DCTS_R = crate::BitReader<DCTS_A>;
#[doc = "Delta CTS. Set upon state change of input CTS. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCTS_A {
    #[doc = "0: No change detected on modem input, CTS."]
    NO_CHANGE_DETECTED = 0,
    #[doc = "1: State change detected on modem input, CTS."]
    CHANGE_DETECTED = 1,
}
impl From<DCTS_A> for bool {
    #[inline(always)]
    fn from(variant: DCTS_A) -> Self {
        variant as u8 != 0
    }
}
impl DCTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCTS_A {
        match self.bits {
            false => DCTS_A::NO_CHANGE_DETECTED,
            true => DCTS_A::CHANGE_DETECTED,
        }
    }
    #[doc = "No change detected on modem input, CTS."]
    #[inline(always)]
    pub fn is_no_change_detected(&self) -> bool {
        *self == DCTS_A::NO_CHANGE_DETECTED
    }
    #[doc = "State change detected on modem input, CTS."]
    #[inline(always)]
    pub fn is_change_detected(&self) -> bool {
        *self == DCTS_A::CHANGE_DETECTED
    }
}
#[doc = "Field `DDSR` reader - Delta DSR. Set upon state change of input DSR. Cleared on an MSR read."]
pub type DDSR_R = crate::BitReader<DDSR_A>;
#[doc = "Delta DSR. Set upon state change of input DSR. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDSR_A {
    #[doc = "0: No change detected on modem input, DSR."]
    NO_CHANGE_DETECTED = 0,
    #[doc = "1: State change detected on modem input, DSR."]
    CHANGE_DETECTED = 1,
}
impl From<DDSR_A> for bool {
    #[inline(always)]
    fn from(variant: DDSR_A) -> Self {
        variant as u8 != 0
    }
}
impl DDSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDSR_A {
        match self.bits {
            false => DDSR_A::NO_CHANGE_DETECTED,
            true => DDSR_A::CHANGE_DETECTED,
        }
    }
    #[doc = "No change detected on modem input, DSR."]
    #[inline(always)]
    pub fn is_no_change_detected(&self) -> bool {
        *self == DDSR_A::NO_CHANGE_DETECTED
    }
    #[doc = "State change detected on modem input, DSR."]
    #[inline(always)]
    pub fn is_change_detected(&self) -> bool {
        *self == DDSR_A::CHANGE_DETECTED
    }
}
#[doc = "Field `TERI` reader - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read."]
pub type TERI_R = crate::BitReader<TERI_A>;
#[doc = "Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERI_A {
    #[doc = "0: No change detected on modem input, RI."]
    NO_CHANGE_DETECTED = 0,
    #[doc = "1: Low-to-high transition detected on RI."]
    CHANGE_DETECTED = 1,
}
impl From<TERI_A> for bool {
    #[inline(always)]
    fn from(variant: TERI_A) -> Self {
        variant as u8 != 0
    }
}
impl TERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TERI_A {
        match self.bits {
            false => TERI_A::NO_CHANGE_DETECTED,
            true => TERI_A::CHANGE_DETECTED,
        }
    }
    #[doc = "No change detected on modem input, RI."]
    #[inline(always)]
    pub fn is_no_change_detected(&self) -> bool {
        *self == TERI_A::NO_CHANGE_DETECTED
    }
    #[doc = "Low-to-high transition detected on RI."]
    #[inline(always)]
    pub fn is_change_detected(&self) -> bool {
        *self == TERI_A::CHANGE_DETECTED
    }
}
#[doc = "Field `DDCD` reader - Delta DCD. Set upon state change of input DCD. Cleared on an MSR read."]
pub type DDCD_R = crate::BitReader<DDCD_A>;
#[doc = "Delta DCD. Set upon state change of input DCD. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDCD_A {
    #[doc = "0: No change detected on modem input, DCD."]
    NO_CHANGE_DETECTED = 0,
    #[doc = "1: State change detected on modem input, DCD."]
    CHANGE_DETECTED = 1,
}
impl From<DDCD_A> for bool {
    #[inline(always)]
    fn from(variant: DDCD_A) -> Self {
        variant as u8 != 0
    }
}
impl DDCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDCD_A {
        match self.bits {
            false => DDCD_A::NO_CHANGE_DETECTED,
            true => DDCD_A::CHANGE_DETECTED,
        }
    }
    #[doc = "No change detected on modem input, DCD."]
    #[inline(always)]
    pub fn is_no_change_detected(&self) -> bool {
        *self == DDCD_A::NO_CHANGE_DETECTED
    }
    #[doc = "State change detected on modem input, DCD."]
    #[inline(always)]
    pub fn is_change_detected(&self) -> bool {
        *self == DDCD_A::CHANGE_DETECTED
    }
}
#[doc = "Field `CTS` reader - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR\\[1\\]
in modem loopback mode."]
pub type CTS_R = crate::BitReader;
#[doc = "Field `DSR` reader - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR\\[0\\]
in modem loopback mode."]
pub type DSR_R = crate::BitReader;
#[doc = "Field `RI` reader - Ring Indicator State. Complement of input RI. This bit is connected to MCR\\[2\\]
in modem loopback mode."]
pub type RI_R = crate::BitReader;
#[doc = "Field `DCD` reader - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR\\[3\\]
in modem loopback mode."]
pub type DCD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Delta CTS. Set upon state change of input CTS. Cleared on an MSR read."]
    #[inline(always)]
    pub fn dcts(&self) -> DCTS_R {
        DCTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Delta DSR. Set upon state change of input DSR. Cleared on an MSR read."]
    #[inline(always)]
    pub fn ddsr(&self) -> DDSR_R {
        DDSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read."]
    #[inline(always)]
    pub fn teri(&self) -> TERI_R {
        TERI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Delta DCD. Set upon state change of input DCD. Cleared on an MSR read."]
    #[inline(always)]
    pub fn ddcd(&self) -> DDCD_R {
        DDCD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR\\[1\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR\\[0\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ring Indicator State. Complement of input RI. This bit is connected to MCR\\[2\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR\\[3\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Modem Status Register. Contains handshake signal status flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`](R). WARN: The register is **modified** in some way after a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MSR_SPEC {}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
