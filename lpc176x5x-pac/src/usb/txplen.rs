#[doc = "Register `TXPLEN` writer"]
pub type W = crate::W<TXPLEN_SPEC>;
#[doc = "Field `PKT_LNGTH` writer - The remaining number of bytes to be written to the selected endpoint buffer. This field is decremented by 4 by hardware after each write to USBTxData. When this field decrements to 0, the TxENDPKT bit will be set in USBDevIntSt."]
pub type PKT_LNGTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXPLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:9 - The remaining number of bytes to be written to the selected endpoint buffer. This field is decremented by 4 by hardware after each write to USBTxData. When this field decrements to 0, the TxENDPKT bit will be set in USBDevIntSt."]
    #[inline(always)]
    #[must_use]
    pub fn pkt_lngth(&mut self) -> PKT_LNGTH_W<TXPLEN_SPEC, 0> {
        PKT_LNGTH_W::new(self)
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
#[doc = "USB Transmit Packet Length\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txplen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPLEN_SPEC;
impl crate::RegisterSpec for TXPLEN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txplen::W`](W) writer structure"]
impl crate::Writable for TXPLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPLEN to value 0"]
impl crate::Resettable for TXPLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
