#[doc = "Register `RXDESCRIPTORNUMBER` reader"]
pub type R = crate::R<RXDESCRIPTORNUMBER_SPEC>;
#[doc = "Register `RXDESCRIPTORNUMBER` writer"]
pub type W = crate::W<RXDESCRIPTORNUMBER_SPEC>;
#[doc = "Field `RXDESCRIPTORNUMBER` reader - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
pub type RXDESCRIPTORNUMBER_R = crate::FieldReader<u16>;
#[doc = "Field `RXDESCRIPTORNUMBER` writer - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
pub type RXDESCRIPTORNUMBER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
    #[inline(always)]
    pub fn rxdescriptornumber(&self) -> RXDESCRIPTORNUMBER_R {
        RXDESCRIPTORNUMBER_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDESCRIPTORNUMBER")
            .field(
                "rxdescriptornumber",
                &format_args!("{}", self.rxdescriptornumber().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXDESCRIPTORNUMBER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
    #[inline(always)]
    #[must_use]
    pub fn rxdescriptornumber(&mut self) -> RXDESCRIPTORNUMBER_W<RXDESCRIPTORNUMBER_SPEC, 0> {
        RXDESCRIPTORNUMBER_W::new(self)
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
#[doc = "Receive number of descriptors register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdescriptornumber::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdescriptornumber::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDESCRIPTORNUMBER_SPEC;
impl crate::RegisterSpec for RXDESCRIPTORNUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdescriptornumber::R`](R) reader structure"]
impl crate::Readable for RXDESCRIPTORNUMBER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdescriptornumber::W`](W) writer structure"]
impl crate::Writable for RXDESCRIPTORNUMBER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXDESCRIPTORNUMBER to value 0"]
impl crate::Resettable for RXDESCRIPTORNUMBER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
