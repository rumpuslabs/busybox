#[doc = "Register `FCANIC0` reader"]
pub type R = crate::R<FCANIC0_SPEC>;
#[doc = "Register `FCANIC0` writer"]
pub type W = crate::W<FCANIC0_SPEC>;
#[doc = "Field `INTPND` reader - FullCan Interrupt Pending 0 = FullCan Interrupt Pending bit 0. 1 = FullCan Interrupt Pending bit 1. ... 31 = FullCan Interrupt Pending bit 31."]
pub type INTPND_R = crate::FieldReader<u32>;
#[doc = "Field `INTPND` writer - FullCan Interrupt Pending 0 = FullCan Interrupt Pending bit 0. 1 = FullCan Interrupt Pending bit 1. ... 31 = FullCan Interrupt Pending bit 31."]
pub type INTPND_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - FullCan Interrupt Pending 0 = FullCan Interrupt Pending bit 0. 1 = FullCan Interrupt Pending bit 1. ... 31 = FullCan Interrupt Pending bit 31."]
    #[inline(always)]
    pub fn intpnd(&self) -> INTPND_R {
        INTPND_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCANIC0")
            .field("intpnd", &format_args!("{}", self.intpnd().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FCANIC0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - FullCan Interrupt Pending 0 = FullCan Interrupt Pending bit 0. 1 = FullCan Interrupt Pending bit 1. ... 31 = FullCan Interrupt Pending bit 31."]
    #[inline(always)]
    #[must_use]
    pub fn intpnd(&mut self) -> INTPND_W<FCANIC0_SPEC, 0> {
        INTPND_W::new(self)
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
#[doc = "FullCAN interrupt and capture register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcanic0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcanic0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCANIC0_SPEC;
impl crate::RegisterSpec for FCANIC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcanic0::R`](R) reader structure"]
impl crate::Readable for FCANIC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcanic0::W`](W) writer structure"]
impl crate::Writable for FCANIC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCANIC0 to value 0"]
impl crate::Resettable for FCANIC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
