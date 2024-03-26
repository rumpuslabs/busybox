#[doc = "Register `TXRATE` reader"]
pub type R = crate::R<TXRATE_SPEC>;
#[doc = "Register `TXRATE` writer"]
pub type W = crate::W<TXRATE_SPEC>;
#[doc = "Field `Y_DIVIDER` reader - I2S transmit MCLK rate denominator. This value is used to divide PCLK to produce the transmit MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
pub type Y_DIVIDER_R = crate::FieldReader;
#[doc = "Field `Y_DIVIDER` writer - I2S transmit MCLK rate denominator. This value is used to divide PCLK to produce the transmit MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
pub type Y_DIVIDER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `X_DIVIDER` reader - I2S transmit MCLK rate numerator. This value is used to multiply PCLK by to produce the transmit MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
pub type X_DIVIDER_R = crate::FieldReader;
#[doc = "Field `X_DIVIDER` writer - I2S transmit MCLK rate numerator. This value is used to multiply PCLK by to produce the transmit MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
pub type X_DIVIDER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - I2S transmit MCLK rate denominator. This value is used to divide PCLK to produce the transmit MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
    #[inline(always)]
    pub fn y_divider(&self) -> Y_DIVIDER_R {
        Y_DIVIDER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2S transmit MCLK rate numerator. This value is used to multiply PCLK by to produce the transmit MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
    #[inline(always)]
    pub fn x_divider(&self) -> X_DIVIDER_R {
        X_DIVIDER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXRATE")
            .field("y_divider", &format_args!("{}", self.y_divider().bits()))
            .field("x_divider", &format_args!("{}", self.x_divider().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXRATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S transmit MCLK rate denominator. This value is used to divide PCLK to produce the transmit MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
    #[inline(always)]
    #[must_use]
    pub fn y_divider(&mut self) -> Y_DIVIDER_W<TXRATE_SPEC, 0> {
        Y_DIVIDER_W::new(self)
    }
    #[doc = "Bits 8:15 - I2S transmit MCLK rate numerator. This value is used to multiply PCLK by to produce the transmit MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
    #[inline(always)]
    #[must_use]
    pub fn x_divider(&mut self) -> X_DIVIDER_W<TXRATE_SPEC, 8> {
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
#[doc = "I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txrate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txrate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXRATE_SPEC;
impl crate::RegisterSpec for TXRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txrate::R`](R) reader structure"]
impl crate::Readable for TXRATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txrate::W`](W) writer structure"]
impl crate::Writable for TXRATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXRATE to value 0"]
impl crate::Resettable for TXRATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
