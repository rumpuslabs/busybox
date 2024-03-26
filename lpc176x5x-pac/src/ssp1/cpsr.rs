#[doc = "Register `CPSR` reader"]
pub type R = crate::R<CPSR_SPEC>;
#[doc = "Register `CPSR` writer"]
pub type W = crate::W<CPSR_SPEC>;
#[doc = "Field `CPSDVSR` reader - This even value between 2 and 254, by which PCLK is divided to yield the prescaler output clock. Bit 0 always reads as 0."]
pub type CPSDVSR_R = crate::FieldReader;
#[doc = "Field `CPSDVSR` writer - This even value between 2 and 254, by which PCLK is divided to yield the prescaler output clock. Bit 0 always reads as 0."]
pub type CPSDVSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This even value between 2 and 254, by which PCLK is divided to yield the prescaler output clock. Bit 0 always reads as 0."]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CPSDVSR_R {
        CPSDVSR_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPSR")
            .field("cpsdvsr", &format_args!("{}", self.cpsdvsr().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CPSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This even value between 2 and 254, by which PCLK is divided to yield the prescaler output clock. Bit 0 always reads as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cpsdvsr(&mut self) -> CPSDVSR_W<CPSR_SPEC, 0> {
        CPSDVSR_W::new(self)
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
#[doc = "Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPSR_SPEC;
impl crate::RegisterSpec for CPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsr::R`](R) reader structure"]
impl crate::Readable for CPSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpsr::W`](W) writer structure"]
impl crate::Writable for CPSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPSR to value 0"]
impl crate::Resettable for CPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
