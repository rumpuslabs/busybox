#[doc = "Register `EPDMAEN` writer"]
pub type W = crate::W<EPDMAEN_SPEC>;
#[doc = "Field `EP_DMA_EN0` writer - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_ENABLE bit value must be 0)."]
pub type EP_DMA_EN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_DMA_EN1` writer - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_ENABLE bit must be 0)."]
pub type EP_DMA_EN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_DMA_EN` writer - Endpoint xx(2 &lt;= xx &lt;= 31) DMA enable control bit. 0 = No effect. 1 = Enable the DMA operation for endpoint EPxx."]
pub type EP_DMA_EN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<EPDMAEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_ENABLE bit value must be 0)."]
    #[inline(always)]
    #[must_use]
    pub fn ep_dma_en0(&mut self) -> EP_DMA_EN0_W<EPDMAEN_SPEC, 0> {
        EP_DMA_EN0_W::new(self)
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_ENABLE bit must be 0)."]
    #[inline(always)]
    #[must_use]
    pub fn ep_dma_en1(&mut self) -> EP_DMA_EN1_W<EPDMAEN_SPEC, 1> {
        EP_DMA_EN1_W::new(self)
    }
    #[doc = "Bits 2:31 - Endpoint xx(2 &lt;= xx &lt;= 31) DMA enable control bit. 0 = No effect. 1 = Enable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    #[must_use]
    pub fn ep_dma_en(&mut self) -> EP_DMA_EN_W<EPDMAEN_SPEC, 2> {
        EP_DMA_EN_W::new(self)
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
#[doc = "USB Endpoint DMA Enable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epdmaen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPDMAEN_SPEC;
impl crate::RegisterSpec for EPDMAEN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`epdmaen::W`](W) writer structure"]
impl crate::Writable for EPDMAEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPDMAEN to value 0"]
impl crate::Resettable for EPDMAEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
