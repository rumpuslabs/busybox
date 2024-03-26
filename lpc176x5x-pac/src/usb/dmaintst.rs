#[doc = "Register `DMAINTST` reader"]
pub type R = crate::R<DMAINTST_SPEC>;
#[doc = "Field `EOT` reader - End of Transfer Interrupt bit."]
pub type EOT_R = crate::BitReader<EOT_A>;
#[doc = "End of Transfer Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOT_A {
    #[doc = "0: All bits in the USBEoTIntSt register are 0."]
    NOT_PENDING = 0,
    #[doc = "1: At least one bit in the USBEoTIntSt is set."]
    PENDING = 1,
}
impl From<EOT_A> for bool {
    #[inline(always)]
    fn from(variant: EOT_A) -> Self {
        variant as u8 != 0
    }
}
impl EOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOT_A {
        match self.bits {
            false => EOT_A::NOT_PENDING,
            true => EOT_A::PENDING,
        }
    }
    #[doc = "All bits in the USBEoTIntSt register are 0."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == EOT_A::NOT_PENDING
    }
    #[doc = "At least one bit in the USBEoTIntSt is set."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EOT_A::PENDING
    }
}
#[doc = "Field `NDDR` reader - New DD Request Interrupt bit."]
pub type NDDR_R = crate::BitReader<NDDR_A>;
#[doc = "New DD Request Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NDDR_A {
    #[doc = "0: All bits in the USBNDDRIntSt register are 0."]
    NOT_PENDING = 0,
    #[doc = "1: At least one bit in the USBNDDRIntSt is set."]
    PENDING = 1,
}
impl From<NDDR_A> for bool {
    #[inline(always)]
    fn from(variant: NDDR_A) -> Self {
        variant as u8 != 0
    }
}
impl NDDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NDDR_A {
        match self.bits {
            false => NDDR_A::NOT_PENDING,
            true => NDDR_A::PENDING,
        }
    }
    #[doc = "All bits in the USBNDDRIntSt register are 0."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == NDDR_A::NOT_PENDING
    }
    #[doc = "At least one bit in the USBNDDRIntSt is set."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == NDDR_A::PENDING
    }
}
#[doc = "Field `ERR` reader - System Error Interrupt bit."]
pub type ERR_R = crate::BitReader<ERR_A>;
#[doc = "System Error Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_A {
    #[doc = "0: All bits in the USBSysErrIntSt register are 0."]
    NOT_PENDING = 0,
    #[doc = "1: At least one bit in the USBSysErrIntSt is set."]
    PENDING = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERR_A {
        match self.bits {
            false => ERR_A::NOT_PENDING,
            true => ERR_A::PENDING,
        }
    }
    #[doc = "All bits in the USBSysErrIntSt register are 0."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ERR_A::NOT_PENDING
    }
    #[doc = "At least one bit in the USBSysErrIntSt is set."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR_A::PENDING
    }
}
impl R {
    #[doc = "Bit 0 - End of Transfer Interrupt bit."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New DD Request Interrupt bit."]
    #[inline(always)]
    pub fn nddr(&self) -> NDDR_R {
        NDDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System Error Interrupt bit."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAINTST")
            .field("eot", &format_args!("{}", self.eot().bit()))
            .field("nddr", &format_args!("{}", self.nddr().bit()))
            .field("err", &format_args!("{}", self.err().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DMAINTST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "USB DMA Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaintst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAINTST_SPEC;
impl crate::RegisterSpec for DMAINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaintst::R`](R) reader structure"]
impl crate::Readable for DMAINTST_SPEC {}
#[doc = "`reset()` method sets DMAINTST to value 0"]
impl crate::Resettable for DMAINTST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
