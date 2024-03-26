#[doc = "Register `DEVINTCLR` writer"]
pub type W = crate::W<DEVINTCLR_SPEC>;
#[doc = "Field `FRAMECLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type FRAMECLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_FASTCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type EP_FASTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_SLOWCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type EP_SLOWCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEV_STATCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type DEV_STATCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCEMPTYCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type CCEMPTYCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDFULLCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type CDFULLCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RxENDPKTCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type RX_ENDPKTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TxENDPKTCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type TX_ENDPKTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_RLZEDCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type EP_RLZEDCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERR_INTCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type ERR_INTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DEVINTCLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn frameclr(&mut self) -> FRAMECLR_W<DEVINTCLR_SPEC, 0> {
        FRAMECLR_W::new(self)
    }
    #[doc = "Bit 1 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ep_fastclr(&mut self) -> EP_FASTCLR_W<DEVINTCLR_SPEC, 1> {
        EP_FASTCLR_W::new(self)
    }
    #[doc = "Bit 2 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ep_slowclr(&mut self) -> EP_SLOWCLR_W<DEVINTCLR_SPEC, 2> {
        EP_SLOWCLR_W::new(self)
    }
    #[doc = "Bit 3 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn dev_statclr(&mut self) -> DEV_STATCLR_W<DEVINTCLR_SPEC, 3> {
        DEV_STATCLR_W::new(self)
    }
    #[doc = "Bit 4 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ccemptyclr(&mut self) -> CCEMPTYCLR_W<DEVINTCLR_SPEC, 4> {
        CCEMPTYCLR_W::new(self)
    }
    #[doc = "Bit 5 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn cdfullclr(&mut self) -> CDFULLCLR_W<DEVINTCLR_SPEC, 5> {
        CDFULLCLR_W::new(self)
    }
    #[doc = "Bit 6 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn rx_endpktclr(&mut self) -> RX_ENDPKTCLR_W<DEVINTCLR_SPEC, 6> {
        RX_ENDPKTCLR_W::new(self)
    }
    #[doc = "Bit 7 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn tx_endpktclr(&mut self) -> TX_ENDPKTCLR_W<DEVINTCLR_SPEC, 7> {
        TX_ENDPKTCLR_W::new(self)
    }
    #[doc = "Bit 8 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ep_rlzedclr(&mut self) -> EP_RLZEDCLR_W<DEVINTCLR_SPEC, 8> {
        EP_RLZEDCLR_W::new(self)
    }
    #[doc = "Bit 9 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn err_intclr(&mut self) -> ERR_INTCLR_W<DEVINTCLR_SPEC, 9> {
        ERR_INTCLR_W::new(self)
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
#[doc = "USB Device Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVINTCLR_SPEC;
impl crate::RegisterSpec for DEVINTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devintclr::W`](W) writer structure"]
impl crate::Writable for DEVINTCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVINTCLR to value 0"]
impl crate::Resettable for DEVINTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
