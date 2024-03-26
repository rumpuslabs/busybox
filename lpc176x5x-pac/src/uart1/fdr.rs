#[doc = "Register `FDR` reader"]
pub type R = crate::R<FDR_SPEC>;
#[doc = "Register `FDR` writer"]
pub type W = crate::W<FDR_SPEC>;
#[doc = "Field `DIVADDVAL` reader - Baud rate generation pre-scaler divisor value. If this field is 0, fractional baud rate generator will not impact the UART1 baud rate."]
pub type DIVADDVAL_R = crate::FieldReader;
#[doc = "Field `DIVADDVAL` writer - Baud rate generation pre-scaler divisor value. If this field is 0, fractional baud rate generator will not impact the UART1 baud rate."]
pub type DIVADDVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MULVAL` reader - Baud rate pre-scaler multiplier value. This field must be greater or equal 1 for UART1 to operate properly, regardless of whether the fractional baud rate generator is used or not."]
pub type MULVAL_R = crate::FieldReader;
#[doc = "Field `MULVAL` writer - Baud rate pre-scaler multiplier value. This field must be greater or equal 1 for UART1 to operate properly, regardless of whether the fractional baud rate generator is used or not."]
pub type MULVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Baud rate generation pre-scaler divisor value. If this field is 0, fractional baud rate generator will not impact the UART1 baud rate."]
    #[inline(always)]
    pub fn divaddval(&self) -> DIVADDVAL_R {
        DIVADDVAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Baud rate pre-scaler multiplier value. This field must be greater or equal 1 for UART1 to operate properly, regardless of whether the fractional baud rate generator is used or not."]
    #[inline(always)]
    pub fn mulval(&self) -> MULVAL_R {
        MULVAL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDR")
            .field("divaddval", &format_args!("{}", self.divaddval().bits()))
            .field("mulval", &format_args!("{}", self.mulval().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud rate generation pre-scaler divisor value. If this field is 0, fractional baud rate generator will not impact the UART1 baud rate."]
    #[inline(always)]
    #[must_use]
    pub fn divaddval(&mut self) -> DIVADDVAL_W<FDR_SPEC, 0> {
        DIVADDVAL_W::new(self)
    }
    #[doc = "Bits 4:7 - Baud rate pre-scaler multiplier value. This field must be greater or equal 1 for UART1 to operate properly, regardless of whether the fractional baud rate generator is used or not."]
    #[inline(always)]
    #[must_use]
    pub fn mulval(&mut self) -> MULVAL_W<FDR_SPEC, 4> {
        MULVAL_W::new(self)
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
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr::R`](R) reader structure"]
impl crate::Readable for FDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdr::W`](W) writer structure"]
impl crate::Writable for FDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDR to value 0x10"]
impl crate::Resettable for FDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
