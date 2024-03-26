#[doc = "Register `TXDESCRIPTORNUMBER` reader"]
pub type R = crate::R<TXDESCRIPTORNUMBER_SPEC>;
#[doc = "Register `TXDESCRIPTORNUMBER` writer"]
pub type W = crate::W<TXDESCRIPTORNUMBER_SPEC>;
#[doc = "Field `TXDN` reader - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
pub type TXDN_R = crate::FieldReader<u16>;
#[doc = "Field `TXDN` writer - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
pub type TXDN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
    #[inline(always)]
    pub fn txdn(&self) -> TXDN_R {
        TXDN_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXDESCRIPTORNUMBER")
            .field("txdn", &format_args!("{}", self.txdn().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXDESCRIPTORNUMBER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
    #[inline(always)]
    #[must_use]
    pub fn txdn(&mut self) -> TXDN_W<TXDESCRIPTORNUMBER_SPEC, 0> {
        TXDN_W::new(self)
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
#[doc = "Transmit number of descriptors register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdescriptornumber::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdescriptornumber::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDESCRIPTORNUMBER_SPEC;
impl crate::RegisterSpec for TXDESCRIPTORNUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdescriptornumber::R`](R) reader structure"]
impl crate::Readable for TXDESCRIPTORNUMBER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdescriptornumber::W`](W) writer structure"]
impl crate::Writable for TXDESCRIPTORNUMBER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDESCRIPTORNUMBER to value 0"]
impl crate::Resettable for TXDESCRIPTORNUMBER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
