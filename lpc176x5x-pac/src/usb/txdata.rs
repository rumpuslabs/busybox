#[doc = "Register `TXDATA` writer"]
pub type W = crate::W<TXDATA_SPEC>;
#[doc = "Field `TX_DATA` writer - Transmit Data."]
pub type TX_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Data."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data(&mut self) -> TX_DATA_W<TXDATA_SPEC, 0> {
        TX_DATA_W::new(self)
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
#[doc = "USB Transmit Data\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDATA_SPEC;
impl crate::RegisterSpec for TXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdata::W`](W) writer structure"]
impl crate::Writable for TXDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TXDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
