#[doc = "Register `IIR` reader"]
pub type R = crate::R<IIR_SPEC>;
#[doc = "Field `INTSTATUS` reader - Interrupt status. Note that IIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\]."]
pub type INTSTATUS_R = crate::BitReader<INTSTATUS_A>;
#[doc = "Interrupt status. Note that IIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\].\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTSTATUS_A {
    #[doc = "0: At least one interrupt is pending."]
    PENDING = 0,
    #[doc = "1: No interrupt is pending."]
    NOT_PENDING = 1,
}
impl From<INTSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: INTSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl INTSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTSTATUS_A {
        match self.bits {
            false => INTSTATUS_A::PENDING,
            true => INTSTATUS_A::NOT_PENDING,
        }
    }
    #[doc = "At least one interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == INTSTATUS_A::PENDING
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == INTSTATUS_A::NOT_PENDING
    }
}
#[doc = "Field `INTID` reader - Interrupt identification. IER\\[3:1\\]
identifies an interrupt corresponding to the UART1 Rx or TX FIFO. All other combinations of IER\\[3:1\\]
not listed below are reserved (100,101,111)."]
pub type INTID_R = crate::FieldReader<INTID_A>;
#[doc = "Interrupt identification. IER\\[3:1\\]
identifies an interrupt corresponding to the UART1 Rx or TX FIFO. All other combinations of IER\\[3:1\\]
not listed below are reserved (100,101,111).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTID_A {
    #[doc = "3: 1 - Receive Line Status (RLS)."]
    RLS = 3,
    #[doc = "2: 2a - Receive Data Available (RDA)."]
    RDA = 2,
    #[doc = "6: 2b - Character Time-out Indicator (CTI)."]
    CTI = 6,
    #[doc = "1: 3 - THRE Interrupt."]
    THRE = 1,
    #[doc = "0: 4 - Modem Interrupt."]
    MODEM = 0,
}
impl From<INTID_A> for u8 {
    #[inline(always)]
    fn from(variant: INTID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTID_A {
    type Ux = u8;
}
impl INTID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INTID_A> {
        match self.bits {
            3 => Some(INTID_A::RLS),
            2 => Some(INTID_A::RDA),
            6 => Some(INTID_A::CTI),
            1 => Some(INTID_A::THRE),
            0 => Some(INTID_A::MODEM),
            _ => None,
        }
    }
    #[doc = "1 - Receive Line Status (RLS)."]
    #[inline(always)]
    pub fn is_rls(&self) -> bool {
        *self == INTID_A::RLS
    }
    #[doc = "2a - Receive Data Available (RDA)."]
    #[inline(always)]
    pub fn is_rda(&self) -> bool {
        *self == INTID_A::RDA
    }
    #[doc = "2b - Character Time-out Indicator (CTI)."]
    #[inline(always)]
    pub fn is_cti(&self) -> bool {
        *self == INTID_A::CTI
    }
    #[doc = "3 - THRE Interrupt."]
    #[inline(always)]
    pub fn is_thre(&self) -> bool {
        *self == INTID_A::THRE
    }
    #[doc = "4 - Modem Interrupt."]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == INTID_A::MODEM
    }
}
#[doc = "Field `FIFOENABLE` reader - Copies of FCR\\[0\\]."]
pub type FIFOENABLE_R = crate::FieldReader;
#[doc = "Field `ABEOINT` reader - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
pub type ABEOINT_R = crate::BitReader;
#[doc = "Field `ABTOINT` reader - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
pub type ABTOINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt status. Note that IIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\]."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification. IER\\[3:1\\]
identifies an interrupt corresponding to the UART1 Rx or TX FIFO. All other combinations of IER\\[3:1\\]
not listed below are reserved (100,101,111)."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Copies of FCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoenable(&self) -> FIFOENABLE_R {
        FIFOENABLE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
    #[inline(always)]
    pub fn abeoint(&self) -> ABEOINT_R {
        ABEOINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
    #[inline(always)]
    pub fn abtoint(&self) -> ABTOINT_R {
        ABTOINT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IIR")
            .field("intstatus", &format_args!("{}", self.intstatus().bit()))
            .field("intid", &format_args!("{}", self.intid().bits()))
            .field("fifoenable", &format_args!("{}", self.fifoenable().bits()))
            .field("abeoint", &format_args!("{}", self.abeoint().bit()))
            .field("abtoint", &format_args!("{}", self.abtoint().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IIR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IIR_SPEC;
impl crate::RegisterSpec for IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IIR_SPEC {}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IIR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
