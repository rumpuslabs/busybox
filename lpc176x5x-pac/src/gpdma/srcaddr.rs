#[doc = "Register `SRCADDR%s` reader"]
pub type R = crate::R<SRCADDR_SPEC>;
#[doc = "Register `SRCADDR%s` writer"]
pub type W = crate::W<SRCADDR_SPEC>;
#[doc = "Field `SRCADDR` reader - DMA source address. Reading this register will return the current source address."]
pub type SRCADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SRCADDR` writer - DMA source address. Reading this register will return the current source address."]
pub type SRCADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA source address. Reading this register will return the current source address."]
    #[inline(always)]
    pub fn srcaddr(&self) -> SRCADDR_R {
        SRCADDR_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRCADDR")
            .field("srcaddr", &format_args!("{}", self.srcaddr().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SRCADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA source address. Reading this register will return the current source address."]
    #[inline(always)]
    #[must_use]
    pub fn srcaddr(&mut self) -> SRCADDR_W<SRCADDR_SPEC, 0> {
        SRCADDR_W::new(self)
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
#[doc = "DMA Channel 0 Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCADDR_SPEC;
impl crate::RegisterSpec for SRCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcaddr::R`](R) reader structure"]
impl crate::Readable for SRCADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcaddr::W`](W) writer structure"]
impl crate::Writable for SRCADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCADDR%s to value 0"]
impl crate::Resettable for SRCADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
