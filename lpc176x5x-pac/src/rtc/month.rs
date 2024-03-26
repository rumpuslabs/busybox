#[doc = "Register `MONTH` reader"]
pub type R = crate::R<MONTH_SPEC>;
#[doc = "Register `MONTH` writer"]
pub type W = crate::W<MONTH_SPEC>;
#[doc = "Field `MONTH` reader - Month value in the range of 1 to 12."]
pub type MONTH_R = crate::FieldReader;
#[doc = "Field `MONTH` writer - Month value in the range of 1 to 12."]
pub type MONTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Month value in the range of 1 to 12."]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MONTH")
            .field("month", &format_args!("{}", self.month().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MONTH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Month value in the range of 1 to 12."]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MONTH_W<MONTH_SPEC, 0> {
        MONTH_W::new(self)
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
#[doc = "Months Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`month::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`month::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MONTH_SPEC;
impl crate::RegisterSpec for MONTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`month::R`](R) reader structure"]
impl crate::Readable for MONTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`month::W`](W) writer structure"]
impl crate::Writable for MONTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MONTH to value 0"]
impl crate::Resettable for MONTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
