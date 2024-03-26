#[doc = "Register `STATE` reader"]
pub type R = crate::R<STATE_SPEC>;
#[doc = "Field `IRQ` reader - This bit reflects the presence of Receive Interrupt or Transmit Interrupt. This is determined by comparing the current FIFO levels to the rx_depth_irq and tx_depth_irq fields in the IRQ register."]
pub type IRQ_R = crate::BitReader;
#[doc = "Field `DMAREQ1` reader - This bit reflects the presence of Receive or Transmit DMA Request 1. This is determined by comparing the current FIFO levels to the rx_depth_dma1 and tx_depth_dma1 fields in the DMA1 register."]
pub type DMAREQ1_R = crate::BitReader;
#[doc = "Field `DMAREQ2` reader - This bit reflects the presence of Receive or Transmit DMA Request 2. This is determined by comparing the current FIFO levels to the rx_depth_dma2 and tx_depth_dma2 fields in the DMA2 register."]
pub type DMAREQ2_R = crate::BitReader;
#[doc = "Field `RX_LEVEL` reader - Reflects the current level of the Receive FIFO."]
pub type RX_LEVEL_R = crate::FieldReader;
#[doc = "Field `TX_LEVEL` reader - Reflects the current level of the Transmit FIFO."]
pub type TX_LEVEL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This bit reflects the presence of Receive Interrupt or Transmit Interrupt. This is determined by comparing the current FIFO levels to the rx_depth_irq and tx_depth_irq fields in the IRQ register."]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit reflects the presence of Receive or Transmit DMA Request 1. This is determined by comparing the current FIFO levels to the rx_depth_dma1 and tx_depth_dma1 fields in the DMA1 register."]
    #[inline(always)]
    pub fn dmareq1(&self) -> DMAREQ1_R {
        DMAREQ1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit reflects the presence of Receive or Transmit DMA Request 2. This is determined by comparing the current FIFO levels to the rx_depth_dma2 and tx_depth_dma2 fields in the DMA2 register."]
    #[inline(always)]
    pub fn dmareq2(&self) -> DMAREQ2_R {
        DMAREQ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Reflects the current level of the Receive FIFO."]
    #[inline(always)]
    pub fn rx_level(&self) -> RX_LEVEL_R {
        RX_LEVEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Reflects the current level of the Transmit FIFO."]
    #[inline(always)]
    pub fn tx_level(&self) -> TX_LEVEL_R {
        TX_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("irq", &format_args!("{}", self.irq().bit()))
            .field("dmareq1", &format_args!("{}", self.dmareq1().bit()))
            .field("dmareq2", &format_args!("{}", self.dmareq2().bit()))
            .field("rx_level", &format_args!("{}", self.rx_level().bits()))
            .field("tx_level", &format_args!("{}", self.tx_level().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "I2S Status Feedback Register. Contains status information about the I2S interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATE_SPEC {}
#[doc = "`reset()` method sets STATE to value 0x07"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
