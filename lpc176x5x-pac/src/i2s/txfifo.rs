#[doc = "Register `TXFIFO` writer"]
pub type W = crate::W<TXFIFO_SPEC>;
#[doc = "Field `I2STXFIFO` writer - 8 x 32-bit transmit FIFO."]
pub type I2STXFIFO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXFIFO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - 8 x 32-bit transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn i2stxfifo(&mut self) -> I2STXFIFO_W<TXFIFO_SPEC, 0> {
        I2STXFIFO_W::new(self)
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
#[doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFIFO_SPEC;
impl crate::RegisterSpec for TXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txfifo::W`](W) writer structure"]
impl crate::Writable for TXFIFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXFIFO to value 0"]
impl crate::Resettable for TXFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
