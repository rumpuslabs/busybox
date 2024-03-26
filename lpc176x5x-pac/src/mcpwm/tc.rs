#[doc = "Register `TC[%s]` reader"]
pub type R = crate::R<TC_SPEC>;
#[doc = "Register `TC[%s]` writer"]
pub type W = crate::W<TC_SPEC>;
#[doc = "Field `MCTC` reader - Timer/Counter value."]
pub type MCTC_R = crate::FieldReader<u32>;
#[doc = "Field `MCTC` writer - Timer/Counter value."]
pub type MCTC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer/Counter value."]
    #[inline(always)]
    pub fn mctc(&self) -> MCTC_R {
        MCTC_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC")
            .field("mctc", &format_args!("{}", self.mctc().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer/Counter value."]
    #[inline(always)]
    #[must_use]
    pub fn mctc(&mut self) -> MCTC_W<TC_SPEC, 0> {
        MCTC_W::new(self)
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
#[doc = "Timer Counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TC_SPEC;
impl crate::RegisterSpec for TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tc::R`](R) reader structure"]
impl crate::Readable for TC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tc::W`](W) writer structure"]
impl crate::Writable for TC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TC[%s]
to value 0"]
impl crate::Resettable for TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}