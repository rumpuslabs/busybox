#[doc = "Register `DOW` reader"]
pub type R = crate::R<DOW_SPEC>;
#[doc = "Register `DOW` writer"]
pub type W = crate::W<DOW_SPEC>;
#[doc = "Field `DOW` reader - Day of week value in the range of 0 to 6."]
pub type DOW_R = crate::FieldReader;
#[doc = "Field `DOW` writer - Day of week value in the range of 0 to 6."]
pub type DOW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Day of week value in the range of 0 to 6."]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOW")
            .field("dow", &format_args!("{}", self.dow().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of week value in the range of 0 to 6."]
    #[inline(always)]
    #[must_use]
    pub fn dow(&mut self) -> DOW_W<DOW_SPEC, 0> {
        DOW_W::new(self)
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
#[doc = "Day of Week Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dow::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dow::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOW_SPEC;
impl crate::RegisterSpec for DOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dow::R`](R) reader structure"]
impl crate::Readable for DOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dow::W`](W) writer structure"]
impl crate::Writable for DOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOW to value 0"]
impl crate::Resettable for DOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
