#[doc = "Register `TMR` reader"]
pub type R = crate::R<TMR_SPEC>;
#[doc = "Register `TMR` writer"]
pub type W = crate::W<TMR_SPEC>;
#[doc = "Field `TIMEOUT_CNT` reader - The TMR interrupt is set when TMR_CNT reaches this value."]
pub type TIMEOUT_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUT_CNT` writer - The TMR interrupt is set when TMR_CNT reaches this value."]
pub type TIMEOUT_CNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - The TMR interrupt is set when TMR_CNT reaches this value."]
    #[inline(always)]
    pub fn timeout_cnt(&self) -> TIMEOUT_CNT_R {
        TIMEOUT_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMR")
            .field(
                "timeout_cnt",
                &format_args!("{}", self.timeout_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TMR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The TMR interrupt is set when TMR_CNT reaches this value."]
    #[inline(always)]
    #[must_use]
    pub fn timeout_cnt(&mut self) -> TIMEOUT_CNT_W<TMR_SPEC, 0> {
        TIMEOUT_CNT_W::new(self)
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
#[doc = "OTG Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMR_SPEC;
impl crate::RegisterSpec for TMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr::R`](R) reader structure"]
impl crate::Readable for TMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmr::W`](W) writer structure"]
impl crate::Writable for TMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMR to value 0xffff"]
impl crate::Resettable for TMR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
