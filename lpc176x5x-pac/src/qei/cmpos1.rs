#[doc = "Register `CMPOS1` reader"]
pub type R = crate::R<CMPOS1_SPEC>;
#[doc = "Register `CMPOS1` writer"]
pub type W = crate::W<CMPOS1_SPEC>;
#[doc = "Field `PCMP1` reader - Position compare value 1."]
pub type PCMP1_R = crate::FieldReader<u32>;
#[doc = "Field `PCMP1` writer - Position compare value 1."]
pub type PCMP1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Position compare value 1."]
    #[inline(always)]
    pub fn pcmp1(&self) -> PCMP1_R {
        PCMP1_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPOS1")
            .field("pcmp1", &format_args!("{}", self.pcmp1().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CMPOS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Position compare value 1."]
    #[inline(always)]
    #[must_use]
    pub fn pcmp1(&mut self) -> PCMP1_W<CMPOS1_SPEC, 0> {
        PCMP1_W::new(self)
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
#[doc = "Position compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpos1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpos1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPOS1_SPEC;
impl crate::RegisterSpec for CMPOS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpos1::R`](R) reader structure"]
impl crate::Readable for CMPOS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpos1::W`](W) writer structure"]
impl crate::Writable for CMPOS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPOS1 to value 0xffff_ffff"]
impl crate::Resettable for CMPOS1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
