#[doc = "Register `TRM` reader"]
pub type R = crate::R<TRM_SPEC>;
#[doc = "Register `TRM` writer"]
pub type W = crate::W<TRM_SPEC>;
#[doc = "Field `ADCOFFS` reader - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
pub type ADCOFFS_R = crate::FieldReader;
#[doc = "Field `ADCOFFS` writer - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
pub type ADCOFFS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TRIM` reader - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `TRIM` writer - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
pub type TRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 4:7 - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
    #[inline(always)]
    pub fn adcoffs(&self) -> ADCOFFS_R {
        ADCOFFS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRM")
            .field("adcoffs", &format_args!("{}", self.adcoffs().bits()))
            .field("trim", &format_args!("{}", self.trim().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TRM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 4:7 - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
    #[inline(always)]
    #[must_use]
    pub fn adcoffs(&mut self) -> ADCOFFS_W<TRM_SPEC, 4> {
        ADCOFFS_W::new(self)
    }
    #[doc = "Bits 8:11 - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<TRM_SPEC, 8> {
        TRIM_W::new(self)
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
#[doc = "ADC trim register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRM_SPEC;
impl crate::RegisterSpec for TRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trm::R`](R) reader structure"]
impl crate::Readable for TRM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trm::W`](W) writer structure"]
impl crate::Writable for TRM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRM to value 0"]
impl crate::Resettable for TRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
