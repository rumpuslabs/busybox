#[doc = "Register `DMA1` reader"]
pub type R = crate::R<DMA1_SPEC>;
#[doc = "Register `DMA1` writer"]
pub type W = crate::W<DMA1_SPEC>;
#[doc = "Field `RX_DMA1_ENABLE` reader - When 1, enables DMA1 for I2S receive."]
pub type RX_DMA1_ENABLE_R = crate::BitReader;
#[doc = "Field `RX_DMA1_ENABLE` writer - When 1, enables DMA1 for I2S receive."]
pub type RX_DMA1_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_DMA1_ENABLE` reader - When 1, enables DMA1 for I2S transmit."]
pub type TX_DMA1_ENABLE_R = crate::BitReader;
#[doc = "Field `TX_DMA1_ENABLE` writer - When 1, enables DMA1 for I2S transmit."]
pub type TX_DMA1_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_DEPTH_DMA1` reader - Set the FIFO level that triggers a receive DMA request on DMA1."]
pub type RX_DEPTH_DMA1_R = crate::FieldReader;
#[doc = "Field `RX_DEPTH_DMA1` writer - Set the FIFO level that triggers a receive DMA request on DMA1."]
pub type RX_DEPTH_DMA1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TX_DEPTH_DMA1` reader - Set the FIFO level that triggers a transmit DMA request on DMA1."]
pub type TX_DEPTH_DMA1_R = crate::FieldReader;
#[doc = "Field `TX_DEPTH_DMA1` writer - Set the FIFO level that triggers a transmit DMA request on DMA1."]
pub type TX_DEPTH_DMA1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma1_enable(&self) -> RX_DMA1_ENABLE_R {
        RX_DMA1_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma1_enable(&self) -> TX_DMA1_ENABLE_R {
        TX_DMA1_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1."]
    #[inline(always)]
    pub fn rx_depth_dma1(&self) -> RX_DEPTH_DMA1_R {
        RX_DEPTH_DMA1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1."]
    #[inline(always)]
    pub fn tx_depth_dma1(&self) -> TX_DEPTH_DMA1_R {
        TX_DEPTH_DMA1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1")
            .field(
                "rx_dma1_enable",
                &format_args!("{}", self.rx_dma1_enable().bit()),
            )
            .field(
                "tx_dma1_enable",
                &format_args!("{}", self.tx_dma1_enable().bit()),
            )
            .field(
                "rx_depth_dma1",
                &format_args!("{}", self.rx_depth_dma1().bits()),
            )
            .field(
                "tx_depth_dma1",
                &format_args!("{}", self.tx_depth_dma1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    #[must_use]
    pub fn rx_dma1_enable(&mut self) -> RX_DMA1_ENABLE_W<DMA1_SPEC, 0> {
        RX_DMA1_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    #[must_use]
    pub fn tx_dma1_enable(&mut self) -> TX_DMA1_ENABLE_W<DMA1_SPEC, 1> {
        TX_DMA1_ENABLE_W::new(self)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1."]
    #[inline(always)]
    #[must_use]
    pub fn rx_depth_dma1(&mut self) -> RX_DEPTH_DMA1_W<DMA1_SPEC, 8> {
        RX_DEPTH_DMA1_W::new(self)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1."]
    #[inline(always)]
    #[must_use]
    pub fn tx_depth_dma1(&mut self) -> TX_DEPTH_DMA1_W<DMA1_SPEC, 16> {
        TX_DEPTH_DMA1_W::new(self)
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
#[doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA1_SPEC;
impl crate::RegisterSpec for DMA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma1::R`](R) reader structure"]
impl crate::Readable for DMA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma1::W`](W) writer structure"]
impl crate::Writable for DMA1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA1 to value 0"]
impl crate::Resettable for DMA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
