#[doc = "Register `VELCOMP` reader"]
pub type R = crate::R<VELCOMP_SPEC>;
#[doc = "Register `VELCOMP` writer"]
pub type W = crate::W<VELCOMP_SPEC>;
#[doc = "Field `VELPC` reader - Compare velocity pulse count."]
pub type VELPC_R = crate::FieldReader<u32>;
#[doc = "Field `VELPC` writer - Compare velocity pulse count."]
pub type VELPC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare velocity pulse count."]
    #[inline(always)]
    pub fn velpc(&self) -> VELPC_R {
        VELPC_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VELCOMP")
            .field("velpc", &format_args!("{}", self.velpc().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<VELCOMP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare velocity pulse count."]
    #[inline(always)]
    #[must_use]
    pub fn velpc(&mut self) -> VELPC_W<VELCOMP_SPEC, 0> {
        VELPC_W::new(self)
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
#[doc = "Velocity compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`velcomp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`velcomp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VELCOMP_SPEC;
impl crate::RegisterSpec for VELCOMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`velcomp::R`](R) reader structure"]
impl crate::Readable for VELCOMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`velcomp::W`](W) writer structure"]
impl crate::Writable for VELCOMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VELCOMP to value 0"]
impl crate::Resettable for VELCOMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
