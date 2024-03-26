#[doc = "Register `CNTVAL` reader"]
pub type R = crate::R<CNTVAL_SPEC>;
#[doc = "Register `CNTVAL` writer"]
pub type W = crate::W<CNTVAL_SPEC>;
#[doc = "Field `VALUE` reader - 16-bit reload value for the DAC interrupt/DMA timer."]
pub type VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 16-bit reload value for the DAC interrupt/DMA timer."]
pub type VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit reload value for the DAC interrupt/DMA timer."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTVAL")
            .field("value", &format_args!("{}", self.value().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CNTVAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16-bit reload value for the DAC interrupt/DMA timer."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<CNTVAL_SPEC, 0> {
        VALUE_W::new(self)
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
#[doc = "DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTVAL_SPEC;
impl crate::RegisterSpec for CNTVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntval::R`](R) reader structure"]
impl crate::Readable for CNTVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntval::W`](W) writer structure"]
impl crate::Writable for CNTVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTVAL to value 0"]
impl crate::Resettable for CNTVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
