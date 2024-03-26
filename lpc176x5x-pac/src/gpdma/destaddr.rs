#[doc = "Register `DESTADDR%s` reader"]
pub type R = crate::R<DESTADDR_SPEC>;
#[doc = "Register `DESTADDR%s` writer"]
pub type W = crate::W<DESTADDR_SPEC>;
#[doc = "Field `DESTADDR` reader - DMA Destination address. Reading this register will return the current destination address."]
pub type DESTADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DESTADDR` writer - DMA Destination address. Reading this register will return the current destination address."]
pub type DESTADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Destination address. Reading this register will return the current destination address."]
    #[inline(always)]
    pub fn destaddr(&self) -> DESTADDR_R {
        DESTADDR_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DESTADDR")
            .field("destaddr", &format_args!("{}", self.destaddr().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DESTADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Destination address. Reading this register will return the current destination address."]
    #[inline(always)]
    #[must_use]
    pub fn destaddr(&mut self) -> DESTADDR_W<DESTADDR_SPEC, 0> {
        DESTADDR_W::new(self)
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
#[doc = "DMA Channel 0 Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESTADDR_SPEC;
impl crate::RegisterSpec for DESTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`destaddr::R`](R) reader structure"]
impl crate::Readable for DESTADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`destaddr::W`](W) writer structure"]
impl crate::Writable for DESTADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTADDR%s to value 0"]
impl crate::Resettable for DESTADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
