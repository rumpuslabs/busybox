#[doc = "Register `DMACREQSEL` reader"]
pub type R = crate::R<DMACREQSEL_SPEC>;
#[doc = "Register `DMACREQSEL` writer"]
pub type W = crate::W<DMACREQSEL_SPEC>;
#[doc = "Field `DMASEL08` reader - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
pub type DMASEL08_R = crate::BitReader;
#[doc = "Field `DMASEL08` writer - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
pub type DMASEL08_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMASEL09` reader - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
pub type DMASEL09_R = crate::BitReader;
#[doc = "Field `DMASEL09` writer - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
pub type DMASEL09_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMASEL10` reader - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
pub type DMASEL10_R = crate::BitReader;
#[doc = "Field `DMASEL10` writer - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
pub type DMASEL10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMASEL11` reader - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
pub type DMASEL11_R = crate::BitReader;
#[doc = "Field `DMASEL11` writer - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
pub type DMASEL11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMASEL12` reader - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
pub type DMASEL12_R = crate::BitReader;
#[doc = "Field `DMASEL12` writer - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
pub type DMASEL12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMASEL13` reader - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
pub type DMASEL13_R = crate::BitReader;
#[doc = "Field `DMASEL13` writer - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
pub type DMASEL13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMASEL14` reader - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
pub type DMASEL14_R = crate::BitReader;
#[doc = "Field `DMASEL14` writer - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
pub type DMASEL14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMASEL15` reader - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
pub type DMASEL15_R = crate::BitReader;
#[doc = "Field `DMASEL15` writer - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
pub type DMASEL15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel08(&self) -> DMASEL08_R {
        DMASEL08_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel09(&self) -> DMASEL09_R {
        DMASEL09_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel10(&self) -> DMASEL10_R {
        DMASEL10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel11(&self) -> DMASEL11_R {
        DMASEL11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel12(&self) -> DMASEL12_R {
        DMASEL12_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel13(&self) -> DMASEL13_R {
        DMASEL13_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    pub fn dmasel14(&self) -> DMASEL14_R {
        DMASEL14_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    pub fn dmasel15(&self) -> DMASEL15_R {
        DMASEL15_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACREQSEL")
            .field("dmasel08", &format_args!("{}", self.dmasel08().bit()))
            .field("dmasel09", &format_args!("{}", self.dmasel09().bit()))
            .field("dmasel10", &format_args!("{}", self.dmasel10().bit()))
            .field("dmasel11", &format_args!("{}", self.dmasel11().bit()))
            .field("dmasel12", &format_args!("{}", self.dmasel12().bit()))
            .field("dmasel13", &format_args!("{}", self.dmasel13().bit()))
            .field("dmasel14", &format_args!("{}", self.dmasel14().bit()))
            .field("dmasel15", &format_args!("{}", self.dmasel15().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DMACREQSEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel08(&mut self) -> DMASEL08_W<DMACREQSEL_SPEC, 0> {
        DMASEL08_W::new(self)
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel09(&mut self) -> DMASEL09_W<DMACREQSEL_SPEC, 1> {
        DMASEL09_W::new(self)
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel10(&mut self) -> DMASEL10_W<DMACREQSEL_SPEC, 2> {
        DMASEL10_W::new(self)
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel11(&mut self) -> DMASEL11_W<DMACREQSEL_SPEC, 3> {
        DMASEL11_W::new(self)
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel12(&mut self) -> DMASEL12_W<DMACREQSEL_SPEC, 4> {
        DMASEL12_W::new(self)
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel13(&mut self) -> DMASEL13_W<DMACREQSEL_SPEC, 5> {
        DMASEL13_W::new(self)
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel14(&mut self) -> DMASEL14_W<DMACREQSEL_SPEC, 6> {
        DMASEL14_W::new(self)
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel15(&mut self) -> DMASEL15_W<DMACREQSEL_SPEC, 7> {
        DMASEL15_W::new(self)
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
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacreqsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacreqsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACREQSEL_SPEC;
impl crate::RegisterSpec for DMACREQSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacreqsel::R`](R) reader structure"]
impl crate::Readable for DMACREQSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacreqsel::W`](W) writer structure"]
impl crate::Writable for DMACREQSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACREQSEL to value 0"]
impl crate::Resettable for DMACREQSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
