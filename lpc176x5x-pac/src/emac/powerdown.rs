#[doc = "Register `POWERDOWN` reader"]
pub type R = crate::R<POWERDOWN_SPEC>;
#[doc = "Register `POWERDOWN` writer"]
pub type W = crate::W<POWERDOWN_SPEC>;
#[doc = "Field `PD` reader - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
pub type PD_R = crate::BitReader;
#[doc = "Field `PD` writer - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
pub type PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 31 - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWERDOWN")
            .field("pd", &format_args!("{}", self.pd().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<POWERDOWN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<POWERDOWN_SPEC, 31> {
        PD_W::new(self)
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
#[doc = "Power-down register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`powerdown::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerdown::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWERDOWN_SPEC;
impl crate::RegisterSpec for POWERDOWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`powerdown::R`](R) reader structure"]
impl crate::Readable for POWERDOWN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`powerdown::W`](W) writer structure"]
impl crate::Writable for POWERDOWN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWERDOWN to value 0"]
impl crate::Resettable for POWERDOWN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
