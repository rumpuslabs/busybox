#[doc = "Register `CTIME0` reader"]
pub type R = crate::R<CTIME0_SPEC>;
#[doc = "Field `SECONDS` reader - Seconds value in the range of 0 to 59"]
pub type SECONDS_R = crate::FieldReader;
#[doc = "Field `MINUTES` reader - Minutes value in the range of 0 to 59"]
pub type MINUTES_R = crate::FieldReader;
#[doc = "Field `HOURS` reader - Hours value in the range of 0 to 23"]
pub type HOURS_R = crate::FieldReader;
#[doc = "Field `DOW` reader - Day of week value in the range of 0 to 6"]
pub type DOW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59"]
    #[inline(always)]
    pub fn seconds(&self) -> SECONDS_R {
        SECONDS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    pub fn minutes(&self) -> MINUTES_R {
        MINUTES_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Day of week value in the range of 0 to 6"]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIME0")
            .field("seconds", &format_args!("{}", self.seconds().bits()))
            .field("minutes", &format_args!("{}", self.minutes().bits()))
            .field("hours", &format_args!("{}", self.hours().bits()))
            .field("dow", &format_args!("{}", self.dow().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CTIME0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Consolidated Time Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctime0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTIME0_SPEC;
impl crate::RegisterSpec for CTIME0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctime0::R`](R) reader structure"]
impl crate::Readable for CTIME0_SPEC {}
#[doc = "`reset()` method sets CTIME0 to value 0"]
impl crate::Resettable for CTIME0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
