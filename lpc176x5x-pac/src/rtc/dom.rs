#[doc = "Register `DOM` reader"]
pub type R = crate::R<DOM_SPEC>;
#[doc = "Register `DOM` writer"]
pub type W = crate::W<DOM_SPEC>;
#[doc = "Field `DOM` reader - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
pub type DOM_R = crate::FieldReader;
#[doc = "Field `DOM` writer - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
pub type DOM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&self) -> DOM_R {
        DOM_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOM")
            .field("dom", &format_args!("{}", self.dom().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DOM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    #[must_use]
    pub fn dom(&mut self) -> DOM_W<DOM_SPEC, 0> {
        DOM_W::new(self)
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
#[doc = "Day of Month Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOM_SPEC;
impl crate::RegisterSpec for DOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dom::R`](R) reader structure"]
impl crate::Readable for DOM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dom::W`](W) writer structure"]
impl crate::Writable for DOM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOM to value 0"]
impl crate::Resettable for DOM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
