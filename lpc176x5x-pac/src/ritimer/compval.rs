#[doc = "Register `COMPVAL` reader"]
pub type R = crate::R<COMPVAL_SPEC>;
#[doc = "Register `COMPVAL` writer"]
pub type W = crate::W<COMPVAL_SPEC>;
#[doc = "Field `RICOMP` reader - Compare register. Holds the compare value which is compared to the counter."]
pub type RICOMP_R = crate::FieldReader<u32>;
#[doc = "Field `RICOMP` writer - Compare register. Holds the compare value which is compared to the counter."]
pub type RICOMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare register. Holds the compare value which is compared to the counter."]
    #[inline(always)]
    pub fn ricomp(&self) -> RICOMP_R {
        RICOMP_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPVAL")
            .field("ricomp", &format_args!("{}", self.ricomp().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<COMPVAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare register. Holds the compare value which is compared to the counter."]
    #[inline(always)]
    #[must_use]
    pub fn ricomp(&mut self) -> RICOMP_W<COMPVAL_SPEC, 0> {
        RICOMP_W::new(self)
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
#[doc = "Compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMPVAL_SPEC;
impl crate::RegisterSpec for COMPVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compval::R`](R) reader structure"]
impl crate::Readable for COMPVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`compval::W`](W) writer structure"]
impl crate::Writable for COMPVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPVAL to value 0xffff_ffff"]
impl crate::Resettable for COMPVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
