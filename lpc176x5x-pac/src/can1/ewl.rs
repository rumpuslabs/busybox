#[doc = "Register `EWL` reader"]
pub type R = crate::R<EWL_SPEC>;
#[doc = "Register `EWL` writer"]
pub type W = crate::W<EWL_SPEC>;
#[doc = "Field `EWL` reader - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
pub type EWL_R = crate::FieldReader;
#[doc = "Field `EWL` writer - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
pub type EWL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EWL")
            .field("ewl", &format_args!("{}", self.ewl().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<EWL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
    #[inline(always)]
    #[must_use]
    pub fn ewl(&mut self) -> EWL_W<EWL_SPEC, 0> {
        EWL_W::new(self)
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
#[doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ewl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ewl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EWL_SPEC;
impl crate::RegisterSpec for EWL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ewl::R`](R) reader structure"]
impl crate::Readable for EWL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ewl::W`](W) writer structure"]
impl crate::Writable for EWL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EWL to value 0x60"]
impl crate::Resettable for EWL_SPEC {
    const RESET_VALUE: Self::Ux = 0x60;
}
