#[doc = "Register `INXCMP0` reader"]
pub type R = crate::R<INXCMP0_SPEC>;
#[doc = "Register `INXCMP0` writer"]
pub type W = crate::W<INXCMP0_SPEC>;
#[doc = "Field `ICMP0` reader - Index compare value 0."]
pub type ICMP0_R = crate::FieldReader<u32>;
#[doc = "Field `ICMP0` writer - Index compare value 0."]
pub type ICMP0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Index compare value 0."]
    #[inline(always)]
    pub fn icmp0(&self) -> ICMP0_R {
        ICMP0_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INXCMP0")
            .field("icmp0", &format_args!("{}", self.icmp0().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INXCMP0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Index compare value 0."]
    #[inline(always)]
    #[must_use]
    pub fn icmp0(&mut self) -> ICMP0_W<INXCMP0_SPEC, 0> {
        ICMP0_W::new(self)
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
#[doc = "Index compare register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inxcmp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inxcmp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INXCMP0_SPEC;
impl crate::RegisterSpec for INXCMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inxcmp0::R`](R) reader structure"]
impl crate::Readable for INXCMP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inxcmp0::W`](W) writer structure"]
impl crate::Writable for INXCMP0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INXCMP0 to value 0xffff_ffff"]
impl crate::Resettable for INXCMP0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
