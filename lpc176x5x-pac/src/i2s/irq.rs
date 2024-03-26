#[doc = "Register `IRQ` reader"]
pub type R = crate::R<IRQ_SPEC>;
#[doc = "Register `IRQ` writer"]
pub type W = crate::W<IRQ_SPEC>;
#[doc = "Field `RX_IRQ_ENABLE` reader - When 1, enables I2S receive interrupt."]
pub type RX_IRQ_ENABLE_R = crate::BitReader;
#[doc = "Field `RX_IRQ_ENABLE` writer - When 1, enables I2S receive interrupt."]
pub type RX_IRQ_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_IRQ_ENABLE` reader - When 1, enables I2S transmit interrupt."]
pub type TX_IRQ_ENABLE_R = crate::BitReader;
#[doc = "Field `TX_IRQ_ENABLE` writer - When 1, enables I2S transmit interrupt."]
pub type TX_IRQ_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_DEPTH_IRQ` reader - Set the FIFO level on which to create an irq request."]
pub type RX_DEPTH_IRQ_R = crate::FieldReader;
#[doc = "Field `RX_DEPTH_IRQ` writer - Set the FIFO level on which to create an irq request."]
pub type RX_DEPTH_IRQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TX_DEPTH_IRQ` reader - Set the FIFO level on which to create an irq request."]
pub type TX_DEPTH_IRQ_R = crate::FieldReader;
#[doc = "Field `TX_DEPTH_IRQ` writer - Set the FIFO level on which to create an irq request."]
pub type TX_DEPTH_IRQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - When 1, enables I2S receive interrupt."]
    #[inline(always)]
    pub fn rx_irq_enable(&self) -> RX_IRQ_ENABLE_R {
        RX_IRQ_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, enables I2S transmit interrupt."]
    #[inline(always)]
    pub fn tx_irq_enable(&self) -> TX_IRQ_ENABLE_R {
        TX_IRQ_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    pub fn rx_depth_irq(&self) -> RX_DEPTH_IRQ_R {
        RX_DEPTH_IRQ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    pub fn tx_depth_irq(&self) -> TX_DEPTH_IRQ_R {
        TX_DEPTH_IRQ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ")
            .field(
                "rx_irq_enable",
                &format_args!("{}", self.rx_irq_enable().bit()),
            )
            .field(
                "tx_irq_enable",
                &format_args!("{}", self.tx_irq_enable().bit()),
            )
            .field(
                "rx_depth_irq",
                &format_args!("{}", self.rx_depth_irq().bits()),
            )
            .field(
                "tx_depth_irq",
                &format_args!("{}", self.tx_depth_irq().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IRQ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables I2S receive interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_irq_enable(&mut self) -> RX_IRQ_ENABLE_W<IRQ_SPEC, 0> {
        RX_IRQ_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - When 1, enables I2S transmit interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_irq_enable(&mut self) -> TX_IRQ_ENABLE_W<IRQ_SPEC, 1> {
        TX_IRQ_ENABLE_W::new(self)
    }
    #[doc = "Bits 8:11 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    #[must_use]
    pub fn rx_depth_irq(&mut self) -> RX_DEPTH_IRQ_W<IRQ_SPEC, 8> {
        RX_DEPTH_IRQ_W::new(self)
    }
    #[doc = "Bits 16:19 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    #[must_use]
    pub fn tx_depth_irq(&mut self) -> TX_DEPTH_IRQ_W<IRQ_SPEC, 16> {
        TX_DEPTH_IRQ_W::new(self)
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
#[doc = "I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_SPEC;
impl crate::RegisterSpec for IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq::R`](R) reader structure"]
impl crate::Readable for IRQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq::W`](W) writer structure"]
impl crate::Writable for IRQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ to value 0"]
impl crate::Resettable for IRQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
