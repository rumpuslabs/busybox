#[doc = "Register `CMPOS2` reader"]
pub type R = crate::R<CMPOS2_SPEC>;
#[doc = "Register `CMPOS2` writer"]
pub type W = crate::W<CMPOS2_SPEC>;
#[doc = "Field `PCMP2` reader - Position compare value 2."]
pub type PCMP2_R = crate::FieldReader<u32>;
#[doc = "Field `PCMP2` writer - Position compare value 2."]
pub type PCMP2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Position compare value 2."]
    #[inline(always)]
    pub fn pcmp2(&self) -> PCMP2_R {
        PCMP2_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPOS2")
            .field("pcmp2", &format_args!("{}", self.pcmp2().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CMPOS2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Position compare value 2."]
    #[inline(always)]
    #[must_use]
    pub fn pcmp2(&mut self) -> PCMP2_W<CMPOS2_SPEC, 0> {
        PCMP2_W::new(self)
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
#[doc = "Position compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpos2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpos2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPOS2_SPEC;
impl crate::RegisterSpec for CMPOS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpos2::R`](R) reader structure"]
impl crate::Readable for CMPOS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpos2::W`](W) writer structure"]
impl crate::Writable for CMPOS2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPOS2 to value 0xffff_ffff"]
impl crate::Resettable for CMPOS2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}