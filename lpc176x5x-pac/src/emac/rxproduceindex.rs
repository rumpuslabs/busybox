#[doc = "Register `RXPRODUCEINDEX` reader"]
pub type R = crate::R<RXPRODUCEINDEX_SPEC>;
#[doc = "Field `RXPRODUCEIX` reader - Index of the descriptor that is going to be filled next by the receive datapath."]
pub type RXPRODUCEIX_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be filled next by the receive datapath."]
    #[inline(always)]
    pub fn rxproduceix(&self) -> RXPRODUCEIX_R {
        RXPRODUCEIX_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXPRODUCEINDEX")
            .field(
                "rxproduceix",
                &format_args!("{}", self.rxproduceix().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXPRODUCEINDEX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Receive produce index register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxproduceindex::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXPRODUCEINDEX_SPEC;
impl crate::RegisterSpec for RXPRODUCEINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxproduceindex::R`](R) reader structure"]
impl crate::Readable for RXPRODUCEINDEX_SPEC {}
#[doc = "`reset()` method sets RXPRODUCEINDEX to value 0"]
impl crate::Resettable for RXPRODUCEINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
