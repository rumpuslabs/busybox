#[doc = "Register `DEVINTEN` reader"]
pub type R = crate::R<DEVINTEN_SPEC>;
#[doc = "Register `DEVINTEN` writer"]
pub type W = crate::W<DEVINTEN_SPEC>;
#[doc = "Field `FRAMEEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type FRAMEEN_R = crate::BitReader;
#[doc = "Field `FRAMEEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type FRAMEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_FASTEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EP_FASTEN_R = crate::BitReader;
#[doc = "Field `EP_FASTEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EP_FASTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_SLOWEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EP_SLOWEN_R = crate::BitReader;
#[doc = "Field `EP_SLOWEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EP_SLOWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEV_STATEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type DEV_STATEN_R = crate::BitReader;
#[doc = "Field `DEV_STATEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type DEV_STATEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCEMPTYEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type CCEMPTYEN_R = crate::BitReader;
#[doc = "Field `CCEMPTYEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type CCEMPTYEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDFULLEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type CDFULLEN_R = crate::BitReader;
#[doc = "Field `CDFULLEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type CDFULLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RxENDPKTEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type RX_ENDPKTEN_R = crate::BitReader;
#[doc = "Field `RxENDPKTEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type RX_ENDPKTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TxENDPKTEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type TX_ENDPKTEN_R = crate::BitReader;
#[doc = "Field `TxENDPKTEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type TX_ENDPKTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP_RLZEDEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EP_RLZEDEN_R = crate::BitReader;
#[doc = "Field `EP_RLZEDEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EP_RLZEDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERR_INTEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type ERR_INTEN_R = crate::BitReader;
#[doc = "Field `ERR_INTEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type ERR_INTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn frameen(&self) -> FRAMEEN_R {
        FRAMEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_fasten(&self) -> EP_FASTEN_R {
        EP_FASTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_slowen(&self) -> EP_SLOWEN_R {
        EP_SLOWEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn dev_staten(&self) -> DEV_STATEN_R {
        DEV_STATEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ccemptyen(&self) -> CCEMPTYEN_R {
        CCEMPTYEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn cdfullen(&self) -> CDFULLEN_R {
        CDFULLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn rx_endpkten(&self) -> RX_ENDPKTEN_R {
        RX_ENDPKTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn tx_endpkten(&self) -> TX_ENDPKTEN_R {
        TX_ENDPKTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_rlzeden(&self) -> EP_RLZEDEN_R {
        EP_RLZEDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn err_inten(&self) -> ERR_INTEN_R {
        ERR_INTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVINTEN")
            .field("frameen", &format_args!("{}", self.frameen().bit()))
            .field("ep_fasten", &format_args!("{}", self.ep_fasten().bit()))
            .field("ep_slowen", &format_args!("{}", self.ep_slowen().bit()))
            .field("dev_staten", &format_args!("{}", self.dev_staten().bit()))
            .field("ccemptyen", &format_args!("{}", self.ccemptyen().bit()))
            .field("cdfullen", &format_args!("{}", self.cdfullen().bit()))
            .field("rx_endpkten", &format_args!("{}", self.rx_endpkten().bit()))
            .field("tx_endpkten", &format_args!("{}", self.tx_endpkten().bit()))
            .field("ep_rlzeden", &format_args!("{}", self.ep_rlzeden().bit()))
            .field("err_inten", &format_args!("{}", self.err_inten().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DEVINTEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn frameen(&mut self) -> FRAMEEN_W<DEVINTEN_SPEC, 0> {
        FRAMEEN_W::new(self)
    }
    #[doc = "Bit 1 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn ep_fasten(&mut self) -> EP_FASTEN_W<DEVINTEN_SPEC, 1> {
        EP_FASTEN_W::new(self)
    }
    #[doc = "Bit 2 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn ep_slowen(&mut self) -> EP_SLOWEN_W<DEVINTEN_SPEC, 2> {
        EP_SLOWEN_W::new(self)
    }
    #[doc = "Bit 3 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn dev_staten(&mut self) -> DEV_STATEN_W<DEVINTEN_SPEC, 3> {
        DEV_STATEN_W::new(self)
    }
    #[doc = "Bit 4 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn ccemptyen(&mut self) -> CCEMPTYEN_W<DEVINTEN_SPEC, 4> {
        CCEMPTYEN_W::new(self)
    }
    #[doc = "Bit 5 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn cdfullen(&mut self) -> CDFULLEN_W<DEVINTEN_SPEC, 5> {
        CDFULLEN_W::new(self)
    }
    #[doc = "Bit 6 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn rx_endpkten(&mut self) -> RX_ENDPKTEN_W<DEVINTEN_SPEC, 6> {
        RX_ENDPKTEN_W::new(self)
    }
    #[doc = "Bit 7 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn tx_endpkten(&mut self) -> TX_ENDPKTEN_W<DEVINTEN_SPEC, 7> {
        TX_ENDPKTEN_W::new(self)
    }
    #[doc = "Bit 8 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn ep_rlzeden(&mut self) -> EP_RLZEDEN_W<DEVINTEN_SPEC, 8> {
        EP_RLZEDEN_W::new(self)
    }
    #[doc = "Bit 9 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn err_inten(&mut self) -> ERR_INTEN_W<DEVINTEN_SPEC, 9> {
        ERR_INTEN_W::new(self)
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
#[doc = "USB Device Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devinten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devinten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVINTEN_SPEC;
impl crate::RegisterSpec for DEVINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinten::R`](R) reader structure"]
impl crate::Readable for DEVINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devinten::W`](W) writer structure"]
impl crate::Writable for DEVINTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVINTEN to value 0"]
impl crate::Resettable for DEVINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
