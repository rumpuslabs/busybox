#[doc = "Register `CONCLR` writer"]
pub type W = crate::W<CONCLR_SPEC>;
#[doc = "Field `AAC` writer - Assert acknowledge Clear bit."]
pub type AAC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SIC` writer - I2C interrupt Clear bit."]
pub type SIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STAC` writer - START flag Clear bit."]
pub type STAC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2ENC` writer - I2C interface Disable bit."]
pub type I2ENC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CONCLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 2 - Assert acknowledge Clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn aac(&mut self) -> AAC_W<CONCLR_SPEC, 2> {
        AAC_W::new(self)
    }
    #[doc = "Bit 3 - I2C interrupt Clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn sic(&mut self) -> SIC_W<CONCLR_SPEC, 3> {
        SIC_W::new(self)
    }
    #[doc = "Bit 5 - START flag Clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn stac(&mut self) -> STAC_W<CONCLR_SPEC, 5> {
        STAC_W::new(self)
    }
    #[doc = "Bit 6 - I2C interface Disable bit."]
    #[inline(always)]
    #[must_use]
    pub fn i2enc(&mut self) -> I2ENC_W<CONCLR_SPEC, 6> {
        I2ENC_W::new(self)
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
#[doc = "I2C Control Clear Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is cleared. Writing a zero has no effect on the corresponding bit in the I2C control register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONCLR_SPEC;
impl crate::RegisterSpec for CONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`conclr::W`](W) writer structure"]
impl crate::Writable for CONCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONCLR to value 0"]
impl crate::Resettable for CONCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
