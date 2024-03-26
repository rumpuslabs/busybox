#[doc = "Register `I2C_WO` writer"]
pub type W = crate::W<I2C_WO_SPEC>;
#[doc = "Field `TXDATA` writer - Transmit data."]
pub type TXDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `START` writer - When 1, issue a START condition before transmitting this byte."]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOP` writer - When 1, issue a STOP condition after transmitting this byte."]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_WO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data."]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<I2C_WO_SPEC, 0> {
        TXDATA_W::new(self)
    }
    #[doc = "Bit 8 - When 1, issue a START condition before transmitting this byte."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<I2C_WO_SPEC, 8> {
        START_W::new(self)
    }
    #[doc = "Bit 9 - When 1, issue a STOP condition after transmitting this byte."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<I2C_WO_SPEC, 9> {
        STOP_W::new(self)
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
#[doc = "I2C Transmit\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_wo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_WO_SPEC;
impl crate::RegisterSpec for I2C_WO_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2c_wo::W`](W) writer structure"]
impl crate::Writable for I2C_WO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_WO to value 0"]
impl crate::Resettable for I2C_WO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
