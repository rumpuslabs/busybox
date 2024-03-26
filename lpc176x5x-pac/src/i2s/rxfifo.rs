#[doc = "Register `RXFIFO` reader"]
pub type R = crate::R<RXFIFO_SPEC>;
#[doc = "Field `I2SRXFIFO` reader - 8 x 32-bit transmit FIFO."]
pub type I2SRXFIFO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 8 x 32-bit transmit FIFO."]
    #[inline(always)]
    pub fn i2srxfifo(&self) -> I2SRXFIFO_R {
        I2SRXFIFO_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXFIFO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifo::R`](R). WARN: The register is **modified** in some way after a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFIFO_SPEC;
impl crate::RegisterSpec for RXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifo::R`](R) reader structure"]
impl crate::Readable for RXFIFO_SPEC {}
#[doc = "`reset()` method sets RXFIFO to value 0"]
impl crate::Resettable for RXFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
