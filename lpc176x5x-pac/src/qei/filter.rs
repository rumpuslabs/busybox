#[doc = "Register `FILTER` reader"]
pub type R = crate::R<FILTER_SPEC>;
#[doc = "Register `FILTER` writer"]
pub type W = crate::W<FILTER_SPEC>;
#[doc = "Field `FILTA` reader - Digital filter sampling delay."]
pub type FILTA_R = crate::FieldReader<u32>;
#[doc = "Field `FILTA` writer - Digital filter sampling delay."]
pub type FILTA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Digital filter sampling delay."]
    #[inline(always)]
    pub fn filta(&self) -> FILTA_R {
        FILTA_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER")
            .field("filta", &format_args!("{}", self.filta().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FILTER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital filter sampling delay."]
    #[inline(always)]
    #[must_use]
    pub fn filta(&mut self) -> FILTA_W<FILTER_SPEC, 0> {
        FILTA_W::new(self)
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
#[doc = "Digital filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_SPEC;
impl crate::RegisterSpec for FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter::R`](R) reader structure"]
impl crate::Readable for FILTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter::W`](W) writer structure"]
impl crate::Writable for FILTER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FILTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
