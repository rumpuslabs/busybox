#[doc = "Register `RXCONSUMEINDEX` reader"]
pub type R = crate::R<RXCONSUMEINDEX_SPEC>;
#[doc = "Register `RXCONSUMEINDEX` writer"]
pub type W = crate::W<RXCONSUMEINDEX_SPEC>;
#[doc = "Field `RXCONSUMEIX` reader - Index of the descriptor that is going to be processed next by the receive"]
pub type RXCONSUMEIX_R = crate::FieldReader<u16>;
#[doc = "Field `RXCONSUMEIX` writer - Index of the descriptor that is going to be processed next by the receive"]
pub type RXCONSUMEIX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be processed next by the receive"]
    #[inline(always)]
    pub fn rxconsumeix(&self) -> RXCONSUMEIX_R {
        RXCONSUMEIX_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXCONSUMEINDEX")
            .field(
                "rxconsumeix",
                &format_args!("{}", self.rxconsumeix().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXCONSUMEINDEX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be processed next by the receive"]
    #[inline(always)]
    #[must_use]
    pub fn rxconsumeix(&mut self) -> RXCONSUMEIX_W<RXCONSUMEINDEX_SPEC, 0> {
        RXCONSUMEIX_W::new(self)
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
#[doc = "Receive consume index register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxconsumeindex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxconsumeindex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCONSUMEINDEX_SPEC;
impl crate::RegisterSpec for RXCONSUMEINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxconsumeindex::R`](R) reader structure"]
impl crate::Readable for RXCONSUMEINDEX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxconsumeindex::W`](W) writer structure"]
impl crate::Writable for RXCONSUMEINDEX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCONSUMEINDEX to value 0"]
impl crate::Resettable for RXCONSUMEINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
