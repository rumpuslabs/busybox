#[doc = "Register `HRS` reader"]
pub type R = crate::R<HRS_SPEC>;
#[doc = "Register `HRS` writer"]
pub type W = crate::W<HRS_SPEC>;
#[doc = "Field `HOURS` reader - Hours value in the range of 0 to 23"]
pub type HOURS_R = crate::FieldReader;
#[doc = "Field `HOURS` writer - Hours value in the range of 0 to 23"]
pub type HOURS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRS")
            .field("hours", &format_args!("{}", self.hours().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<HRS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    #[must_use]
    pub fn hours(&mut self) -> HOURS_W<HRS_SPEC, 0> {
        HOURS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hours Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRS_SPEC;
impl crate::RegisterSpec for HRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrs::R`](R) reader structure"]
impl crate::Readable for HRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hrs::W`](W) writer structure"]
impl crate::Writable for HRS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRS to value 0"]
impl crate::Resettable for HRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
