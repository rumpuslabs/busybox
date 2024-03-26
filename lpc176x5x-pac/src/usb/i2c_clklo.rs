#[doc = "Register `I2C_CLKLO` writer"]
pub type W = crate::W<I2C_CLKLO_SPEC>;
#[doc = "Field `CDLO` writer - Clock divisor low. This value is the number of 48 MHz clocks the serial clock (SCL) will be low."]
pub type CDLO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_CLKLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divisor low. This value is the number of 48 MHz clocks the serial clock (SCL) will be low."]
    #[inline(always)]
    #[must_use]
    pub fn cdlo(&mut self) -> CDLO_W<I2C_CLKLO_SPEC, 0> {
        CDLO_W::new(self)
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
#[doc = "I2C Clock Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_clklo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_CLKLO_SPEC;
impl crate::RegisterSpec for I2C_CLKLO_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2c_clklo::W`](W) writer structure"]
impl crate::Writable for I2C_CLKLO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_CLKLO to value 0xb9"]
impl crate::Resettable for I2C_CLKLO_SPEC {
    const RESET_VALUE: Self::Ux = 0xb9;
}
