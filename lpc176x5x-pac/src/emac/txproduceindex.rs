#[doc = "Register `TXPRODUCEINDEX` reader"]
pub type R = crate::R<TXPRODUCEINDEX_SPEC>;
#[doc = "Register `TXPRODUCEINDEX` writer"]
pub type W = crate::W<TXPRODUCEINDEX_SPEC>;
#[doc = "Field `TXPI` reader - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
pub type TXPI_R = crate::FieldReader<u16>;
#[doc = "Field `TXPI` writer - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
pub type TXPI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
    #[inline(always)]
    pub fn txpi(&self) -> TXPI_R {
        TXPI_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXPRODUCEINDEX")
            .field("txpi", &format_args!("{}", self.txpi().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXPRODUCEINDEX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
    #[inline(always)]
    #[must_use]
    pub fn txpi(&mut self) -> TXPI_W<TXPRODUCEINDEX_SPEC, 0> {
        TXPI_W::new(self)
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
#[doc = "Transmit produce index register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txproduceindex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txproduceindex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPRODUCEINDEX_SPEC;
impl crate::RegisterSpec for TXPRODUCEINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txproduceindex::R`](R) reader structure"]
impl crate::Readable for TXPRODUCEINDEX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txproduceindex::W`](W) writer structure"]
impl crate::Writable for TXPRODUCEINDEX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPRODUCEINDEX to value 0"]
impl crate::Resettable for TXPRODUCEINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
