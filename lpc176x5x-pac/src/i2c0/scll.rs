#[doc = "Register `SCLL` reader"]
pub type R = crate::R<SCLL_SPEC>;
#[doc = "Register `SCLL` writer"]
pub type W = crate::W<SCLL_SPEC>;
#[doc = "Field `SCLL` reader - Count for SCL low time period selection."]
pub type SCLL_R = crate::FieldReader<u16>;
#[doc = "Field `SCLL` writer - Count for SCL low time period selection."]
pub type SCLL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Count for SCL low time period selection."]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCLL")
            .field("scll", &format_args!("{}", self.scll().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SCLL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count for SCL low time period selection."]
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> SCLL_W<SCLL_SPEC, 0> {
        SCLL_W::new(self)
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
#[doc = "SCL Duty Cycle Register Low Half Word. Determines the low time of the I2C clock. SCLL and SCLH together determine the clock frequency generated by an I2C master and certain times used in slave mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCLL_SPEC;
impl crate::RegisterSpec for SCLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scll::R`](R) reader structure"]
impl crate::Readable for SCLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scll::W`](W) writer structure"]
impl crate::Writable for SCLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCLL to value 0x04"]
impl crate::Resettable for SCLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}