#[doc = "Register `CMPOS0` reader"]
pub type R = crate::R<CMPOS0_SPEC>;
#[doc = "Register `CMPOS0` writer"]
pub type W = crate::W<CMPOS0_SPEC>;
#[doc = "Field `PCMP0` reader - Position compare value 0."]
pub type PCMP0_R = crate::FieldReader<u32>;
#[doc = "Field `PCMP0` writer - Position compare value 0."]
pub type PCMP0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Position compare value 0."]
    #[inline(always)]
    pub fn pcmp0(&self) -> PCMP0_R {
        PCMP0_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPOS0")
            .field("pcmp0", &format_args!("{}", self.pcmp0().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CMPOS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Position compare value 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcmp0(&mut self) -> PCMP0_W<CMPOS0_SPEC, 0> {
        PCMP0_W::new(self)
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
#[doc = "Position compare register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpos0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpos0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPOS0_SPEC;
impl crate::RegisterSpec for CMPOS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpos0::R`](R) reader structure"]
impl crate::Readable for CMPOS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpos0::W`](W) writer structure"]
impl crate::Writable for CMPOS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPOS0 to value 0xffff_ffff"]
impl crate::Resettable for CMPOS0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
