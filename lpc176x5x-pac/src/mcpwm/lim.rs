#[doc = "Register `LIM[%s]` reader"]
pub type R = crate::R<LIM_SPEC>;
#[doc = "Register `LIM[%s]` writer"]
pub type W = crate::W<LIM_SPEC>;
#[doc = "Field `MCLIM` reader - Limit value."]
pub type MCLIM_R = crate::FieldReader<u32>;
#[doc = "Field `MCLIM` writer - Limit value."]
pub type MCLIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Limit value."]
    #[inline(always)]
    pub fn mclim(&self) -> MCLIM_R {
        MCLIM_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LIM")
            .field("mclim", &format_args!("{}", self.mclim().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<LIM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Limit value."]
    #[inline(always)]
    #[must_use]
    pub fn mclim(&mut self) -> MCLIM_W<LIM_SPEC, 0> {
        MCLIM_W::new(self)
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
#[doc = "Limit register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIM_SPEC;
impl crate::RegisterSpec for LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lim::R`](R) reader structure"]
impl crate::Readable for LIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lim::W`](W) writer structure"]
impl crate::Writable for LIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LIM[%s]
to value 0"]
impl crate::Resettable for LIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
