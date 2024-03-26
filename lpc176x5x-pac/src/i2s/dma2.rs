#[doc = "Register `DMA2` reader"]
pub type R = crate::R<DMA2_SPEC>;
#[doc = "Register `DMA2` writer"]
pub type W = crate::W<DMA2_SPEC>;
#[doc = "Field `RX_DMA2_ENABLE` reader - When 1, enables DMA1 for I2S receive."]
pub type RX_DMA2_ENABLE_R = crate::BitReader;
#[doc = "Field `RX_DMA2_ENABLE` writer - When 1, enables DMA1 for I2S receive."]
pub type RX_DMA2_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_DMA2_ENABLE` reader - When 1, enables DMA1 for I2S transmit."]
pub type TX_DMA2_ENABLE_R = crate::BitReader;
#[doc = "Field `TX_DMA2_ENABLE` writer - When 1, enables DMA1 for I2S transmit."]
pub type TX_DMA2_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_DEPTH_DMA2` reader - Set the FIFO level that triggers a receive DMA request on DMA2."]
pub type RX_DEPTH_DMA2_R = crate::FieldReader;
#[doc = "Field `RX_DEPTH_DMA2` writer - Set the FIFO level that triggers a receive DMA request on DMA2."]
pub type RX_DEPTH_DMA2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TX_DEPTH_DMA2` reader - Set the FIFO level that triggers a transmit DMA request on DMA2."]
pub type TX_DEPTH_DMA2_R = crate::FieldReader;
#[doc = "Field `TX_DEPTH_DMA2` writer - Set the FIFO level that triggers a transmit DMA request on DMA2."]
pub type TX_DEPTH_DMA2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma2_enable(&self) -> RX_DMA2_ENABLE_R {
        RX_DMA2_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma2_enable(&self) -> TX_DMA2_ENABLE_R {
        TX_DMA2_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA2."]
    #[inline(always)]
    pub fn rx_depth_dma2(&self) -> RX_DEPTH_DMA2_R {
        RX_DEPTH_DMA2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA2."]
    #[inline(always)]
    pub fn tx_depth_dma2(&self) -> TX_DEPTH_DMA2_R {
        TX_DEPTH_DMA2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA2")
            .field(
                "rx_dma2_enable",
                &format_args!("{}", self.rx_dma2_enable().bit()),
            )
            .field(
                "tx_dma2_enable",
                &format_args!("{}", self.tx_dma2_enable().bit()),
            )
            .field(
                "rx_depth_dma2",
                &format_args!("{}", self.rx_depth_dma2().bits()),
            )
            .field(
                "tx_depth_dma2",
                &format_args!("{}", self.tx_depth_dma2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    #[must_use]
    pub fn rx_dma2_enable(&mut self) -> RX_DMA2_ENABLE_W<DMA2_SPEC, 0> {
        RX_DMA2_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    #[must_use]
    pub fn tx_dma2_enable(&mut self) -> TX_DMA2_ENABLE_W<DMA2_SPEC, 1> {
        TX_DMA2_ENABLE_W::new(self)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA2."]
    #[inline(always)]
    #[must_use]
    pub fn rx_depth_dma2(&mut self) -> RX_DEPTH_DMA2_W<DMA2_SPEC, 8> {
        RX_DEPTH_DMA2_W::new(self)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA2."]
    #[inline(always)]
    #[must_use]
    pub fn tx_depth_dma2(&mut self) -> TX_DEPTH_DMA2_W<DMA2_SPEC, 16> {
        TX_DEPTH_DMA2_W::new(self)
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
#[doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA2_SPEC;
impl crate::RegisterSpec for DMA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma2::R`](R) reader structure"]
impl crate::Readable for DMA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma2::W`](W) writer structure"]
impl crate::Writable for DMA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA2 to value 0"]
impl crate::Resettable for DMA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
