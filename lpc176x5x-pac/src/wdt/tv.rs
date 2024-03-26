#[doc = "Register `TV` reader"]
pub type R = crate::R<TV_SPEC>;
#[doc = "Field `Count` reader - Counter timer value."]
pub type COUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Counter timer value."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TV")
            .field("count", &format_args!("{}", self.count().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_SPEC;
impl crate::RegisterSpec for TV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv::R`](R) reader structure"]
impl crate::Readable for TV_SPEC {}
#[doc = "`reset()` method sets TV to value 0xff"]
impl crate::Resettable for TV_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
