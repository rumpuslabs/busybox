#[doc = "Register `I2C_RX` reader"]
pub type R = crate::R<I2C_RX_SPEC>;
#[doc = "Field `RXDATA` reader - Receive data."]
pub type RXDATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive data."]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_RX")
            .field("rxdata", &format_args!("{}", self.rxdata().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_RX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "I2C Receive\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_rx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_RX_SPEC;
impl crate::RegisterSpec for I2C_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_rx::R`](R) reader structure"]
impl crate::Readable for I2C_RX_SPEC {}
#[doc = "`reset()` method sets I2C_RX to value 0"]
impl crate::Resettable for I2C_RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
