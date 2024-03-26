#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `TMR_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
pub type TMR_EN_R = crate::BitReader;
#[doc = "Field `TMR_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
pub type TMR_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REMOVE_PU_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
pub type REMOVE_PU_EN_R = crate::BitReader;
#[doc = "Field `REMOVE_PU_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
pub type REMOVE_PU_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNP_FAILURE_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
pub type HNP_FAILURE_EN_R = crate::BitReader;
#[doc = "Field `HNP_FAILURE_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
pub type HNP_FAILURE_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNP_SUCCES_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
pub type HNP_SUCCES_EN_R = crate::BitReader;
#[doc = "Field `HNP_SUCCES_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
pub type HNP_SUCCES_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_en(&self) -> TMR_EN_R {
        TMR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_en(&self) -> REMOVE_PU_EN_R {
        REMOVE_PU_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_en(&self) -> HNP_FAILURE_EN_R {
        HNP_FAILURE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_en(&self) -> HNP_SUCCES_EN_R {
        HNP_SUCCES_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("tmr_en", &format_args!("{}", self.tmr_en().bit()))
            .field(
                "remove_pu_en",
                &format_args!("{}", self.remove_pu_en().bit()),
            )
            .field(
                "hnp_failure_en",
                &format_args!("{}", self.hnp_failure_en().bit()),
            )
            .field(
                "hnp_succes_en",
                &format_args!("{}", self.hnp_succes_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn tmr_en(&mut self) -> TMR_EN_W<INTEN_SPEC, 0> {
        TMR_EN_W::new(self)
    }
    #[doc = "Bit 1 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn remove_pu_en(&mut self) -> REMOVE_PU_EN_W<INTEN_SPEC, 1> {
        REMOVE_PU_EN_W::new(self)
    }
    #[doc = "Bit 2 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_failure_en(&mut self) -> HNP_FAILURE_EN_W<INTEN_SPEC, 2> {
        HNP_FAILURE_EN_W::new(self)
    }
    #[doc = "Bit 3 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_succes_en(&mut self) -> HNP_SUCCES_EN_W<INTEN_SPEC, 3> {
        HNP_SUCCES_EN_W::new(self)
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
#[doc = "OTG Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
