#[doc = "Register `CTIME1` reader"]
pub type R = crate::R<CTIME1_SPEC>;
#[doc = "Field `DOM` reader - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
pub type DOM_R = crate::FieldReader;
#[doc = "Field `MONTH` reader - Month value in the range of 1 to 12."]
pub type MONTH_R = crate::FieldReader;
#[doc = "Field `YEAR` reader - Year value in the range of 0 to 4095."]
pub type YEAR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&self) -> DOM_R {
        DOM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month value in the range of 1 to 12."]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIME1")
            .field("dom", &format_args!("{}", self.dom().bits()))
            .field("month", &format_args!("{}", self.month().bits()))
            .field("year", &format_args!("{}", self.year().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CTIME1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Consolidated Time Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctime1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTIME1_SPEC;
impl crate::RegisterSpec for CTIME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctime1::R`](R) reader structure"]
impl crate::Readable for CTIME1_SPEC {}
#[doc = "`reset()` method sets CTIME1 to value 0"]
impl crate::Resettable for CTIME1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
