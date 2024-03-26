#[doc = "Register `RXRATE` reader"]
pub type R = crate::R<RXRATE_SPEC>;
#[doc = "Register `RXRATE` writer"]
pub type W = crate::W<RXRATE_SPEC>;
#[doc = "Field `Y_DIVIDER` reader - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
pub type Y_DIVIDER_R = crate::FieldReader;
#[doc = "Field `Y_DIVIDER` writer - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
pub type Y_DIVIDER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `X_DIVIDER` reader - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
pub type X_DIVIDER_R = crate::FieldReader;
#[doc = "Field `X_DIVIDER` writer - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
pub type X_DIVIDER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
    #[inline(always)]
    pub fn y_divider(&self) -> Y_DIVIDER_R {
        Y_DIVIDER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
    #[inline(always)]
    pub fn x_divider(&self) -> X_DIVIDER_R {
        X_DIVIDER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXRATE")
            .field("y_divider", &format_args!("{}", self.y_divider().bits()))
            .field("x_divider", &format_args!("{}", self.x_divider().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXRATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
    #[inline(always)]
    #[must_use]
    pub fn y_divider(&mut self) -> Y_DIVIDER_W<RXRATE_SPEC, 0> {
        Y_DIVIDER_W::new(self)
    }
    #[doc = "Bits 8:15 - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
    #[inline(always)]
    #[must_use]
    pub fn x_divider(&mut self) -> X_DIVIDER_W<RXRATE_SPEC, 8> {
        X_DIVIDER_W::new(self)
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
#[doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxrate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxrate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXRATE_SPEC;
impl crate::RegisterSpec for RXRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxrate::R`](R) reader structure"]
impl crate::Readable for RXRATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxrate::W`](W) writer structure"]
impl crate::Writable for RXRATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXRATE to value 0"]
impl crate::Resettable for RXRATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
