#[doc = "Register `RXDESCRIPTOR` reader"]
pub type R = crate::R<RXDESCRIPTOR_SPEC>;
#[doc = "Register `RXDESCRIPTOR` writer"]
pub type W = crate::W<RXDESCRIPTOR_SPEC>;
#[doc = "Field `RXDESCRIPTOR` reader - MSBs of receive descriptor base address."]
pub type RXDESCRIPTOR_R = crate::FieldReader<u32>;
#[doc = "Field `RXDESCRIPTOR` writer - MSBs of receive descriptor base address."]
pub type RXDESCRIPTOR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - MSBs of receive descriptor base address."]
    #[inline(always)]
    pub fn rxdescriptor(&self) -> RXDESCRIPTOR_R {
        RXDESCRIPTOR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDESCRIPTOR")
            .field(
                "rxdescriptor",
                &format_args!("{}", self.rxdescriptor().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXDESCRIPTOR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 2:31 - MSBs of receive descriptor base address."]
    #[inline(always)]
    #[must_use]
    pub fn rxdescriptor(&mut self) -> RXDESCRIPTOR_W<RXDESCRIPTOR_SPEC, 2> {
        RXDESCRIPTOR_W::new(self)
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
#[doc = "Receive descriptor base address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdescriptor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdescriptor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDESCRIPTOR_SPEC;
impl crate::RegisterSpec for RXDESCRIPTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdescriptor::R`](R) reader structure"]
impl crate::Readable for RXDESCRIPTOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdescriptor::W`](W) writer structure"]
impl crate::Writable for RXDESCRIPTOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXDESCRIPTOR to value 0"]
impl crate::Resettable for RXDESCRIPTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
