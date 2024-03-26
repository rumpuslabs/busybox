#[doc = "Register `I2C_CLKHI` reader"]
pub type R = crate::R<I2C_CLKHI_SPEC>;
#[doc = "Register `I2C_CLKHI` writer"]
pub type W = crate::W<I2C_CLKHI_SPEC>;
#[doc = "Field `CDHI` reader - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
pub type CDHI_R = crate::FieldReader;
#[doc = "Field `CDHI` writer - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
pub type CDHI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
    #[inline(always)]
    pub fn cdhi(&self) -> CDHI_R {
        CDHI_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_CLKHI")
            .field("cdhi", &format_args!("{}", self.cdhi().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_CLKHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
    #[inline(always)]
    #[must_use]
    pub fn cdhi(&mut self) -> CDHI_W<I2C_CLKHI_SPEC, 0> {
        CDHI_W::new(self)
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
#[doc = "I2C Clock High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_clkhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_clkhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_CLKHI_SPEC;
impl crate::RegisterSpec for I2C_CLKHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_clkhi::R`](R) reader structure"]
impl crate::Readable for I2C_CLKHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_clkhi::W`](W) writer structure"]
impl crate::Writable for I2C_CLKHI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_CLKHI to value 0xb9"]
impl crate::Resettable for I2C_CLKHI_SPEC {
    const RESET_VALUE: Self::Ux = 0xb9;
}
