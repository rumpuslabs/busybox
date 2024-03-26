#[doc = "Register `DEVINTSET` writer"]
pub type W = crate::W<DEVINTSET_SPEC>;
#[doc = "Field `FRAMESET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type FRAMESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_FASTSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type EP_FASTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_SLOWSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type EP_SLOWSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEV_STATSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type DEV_STATSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCEMPTYSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type CCEMPTYSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDFULLSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type CDFULLSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RxENDPKTSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type RX_ENDPKTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TxENDPKTSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type TX_ENDPKTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_RLZEDSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type EP_RLZEDSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERR_INTSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type ERR_INTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DEVINTSET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    #[must_use]
    pub fn frameset(&mut self) -> FRAMESET_W<DEVINTSET_SPEC, 0> {
        FRAMESET_W::new(self)
    }
    #[doc = "Bit 1 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    #[must_use]
    pub fn ep_fastset(&mut self) -> EP_FASTSET_W<DEVINTSET_SPEC, 1> {
        EP_FASTSET_W::new(self)
    }
    #[doc = "Bit 2 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    #[must_use]
    pub fn ep_slowset(&mut self) -> EP_SLOWSET_W<DEVINTSET_SPEC, 2> {
        EP_SLOWSET_W::new(self)
    }
    #[doc = "Bit 3 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    #[must_use]
    pub fn dev_statset(&mut self) -> DEV_STATSET_W<DEVINTSET_SPEC, 3> {
        DEV_STATSET_W::new(self)
    }
    #[doc = "Bit 4 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    #[must_use]
    pub fn ccemptyset(&mut self) -> CCEMPTYSET_W<DEVINTSET_SPEC, 4> {
        CCEMPTYSET_W::new(self)
    }
    #[doc = "Bit 5 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    #[must_use]
    pub fn cdfullset(&mut self) -> CDFULLSET_W<DEVINTSET_SPEC, 5> {
        CDFULLSET_W::new(self)
    }
    #[doc = "Bit 6 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    #[must_use]
    pub fn rx_endpktset(&mut self) -> RX_ENDPKTSET_W<DEVINTSET_SPEC, 6> {
        RX_ENDPKTSET_W::new(self)
    }
    #[doc = "Bit 7 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    #[must_use]
    pub fn tx_endpktset(&mut self) -> TX_ENDPKTSET_W<DEVINTSET_SPEC, 7> {
        TX_ENDPKTSET_W::new(self)
    }
    #[doc = "Bit 8 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    #[must_use]
    pub fn ep_rlzedset(&mut self) -> EP_RLZEDSET_W<DEVINTSET_SPEC, 8> {
        EP_RLZEDSET_W::new(self)
    }
    #[doc = "Bit 9 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    #[must_use]
    pub fn err_intset(&mut self) -> ERR_INTSET_W<DEVINTSET_SPEC, 9> {
        ERR_INTSET_W::new(self)
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
#[doc = "USB Device Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devintset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVINTSET_SPEC;
impl crate::RegisterSpec for DEVINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devintset::W`](W) writer structure"]
impl crate::Writable for DEVINTSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVINTSET to value 0"]
impl crate::Resettable for DEVINTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
