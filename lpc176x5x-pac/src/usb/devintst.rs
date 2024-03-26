#[doc = "Register `DEVINTST` reader"]
pub type R = crate::R<DEVINTST_SPEC>;
#[doc = "Field `FRAME` reader - The frame interrupt occurs every 1 ms. This is used in isochronous packet transfers."]
pub type FRAME_R = crate::BitReader;
#[doc = "Field `EP_FAST` reader - Fast endpoint interrupt. If an Endpoint Interrupt Priority register (USBEpIntPri) bit is set, the corresponding endpoint interrupt will be routed to this bit."]
pub type EP_FAST_R = crate::BitReader;
#[doc = "Field `EP_SLOW` reader - Slow endpoints interrupt. If an Endpoint Interrupt Priority Register (USBEpIntPri) bit is not set, the corresponding endpoint interrupt will be routed to this bit."]
pub type EP_SLOW_R = crate::BitReader;
#[doc = "Field `DEV_STAT` reader - Set when USB Bus reset, USB suspend change or Connect change event occurs. Refer to Section 13.12.6 Set Device Status (Command: 0xFE, Data: write 1 byte) on page 366."]
pub type DEV_STAT_R = crate::BitReader;
#[doc = "Field `CCEMPTY` reader - The command code register (USBCmdCode) is empty (New command can be written)."]
pub type CCEMPTY_R = crate::BitReader;
#[doc = "Field `CDFULL` reader - Command data register (USBCmdData) is full (Data can be read now)."]
pub type CDFULL_R = crate::BitReader;
#[doc = "Field `RxENDPKT` reader - The current packet in the endpoint buffer is transferred to the CPU."]
pub type RX_ENDPKT_R = crate::BitReader;
#[doc = "Field `TxENDPKT` reader - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen)."]
pub type TX_ENDPKT_R = crate::BitReader;
#[doc = "Field `EP_RLZED` reader - Endpoints realized. Set when Realize Endpoint register (USBReEp) or MaxPacketSize register (USBMaxPSize) is updated and the corresponding operation is completed."]
pub type EP_RLZED_R = crate::BitReader;
#[doc = "Field `ERR_INT` reader - Error Interrupt. Any bus error interrupt from the USB device. Refer to Section 13.12.9 Read Error Status (Command: 0xFB, Data: read 1 byte) on page 368"]
pub type ERR_INT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The frame interrupt occurs every 1 ms. This is used in isochronous packet transfers."]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast endpoint interrupt. If an Endpoint Interrupt Priority register (USBEpIntPri) bit is set, the corresponding endpoint interrupt will be routed to this bit."]
    #[inline(always)]
    pub fn ep_fast(&self) -> EP_FAST_R {
        EP_FAST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slow endpoints interrupt. If an Endpoint Interrupt Priority Register (USBEpIntPri) bit is not set, the corresponding endpoint interrupt will be routed to this bit."]
    #[inline(always)]
    pub fn ep_slow(&self) -> EP_SLOW_R {
        EP_SLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set when USB Bus reset, USB suspend change or Connect change event occurs. Refer to Section 13.12.6 Set Device Status (Command: 0xFE, Data: write 1 byte) on page 366."]
    #[inline(always)]
    pub fn dev_stat(&self) -> DEV_STAT_R {
        DEV_STAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The command code register (USBCmdCode) is empty (New command can be written)."]
    #[inline(always)]
    pub fn ccempty(&self) -> CCEMPTY_R {
        CCEMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Command data register (USBCmdData) is full (Data can be read now)."]
    #[inline(always)]
    pub fn cdfull(&self) -> CDFULL_R {
        CDFULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The current packet in the endpoint buffer is transferred to the CPU."]
    #[inline(always)]
    pub fn rx_endpkt(&self) -> RX_ENDPKT_R {
        RX_ENDPKT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen)."]
    #[inline(always)]
    pub fn tx_endpkt(&self) -> TX_ENDPKT_R {
        TX_ENDPKT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoints realized. Set when Realize Endpoint register (USBReEp) or MaxPacketSize register (USBMaxPSize) is updated and the corresponding operation is completed."]
    #[inline(always)]
    pub fn ep_rlzed(&self) -> EP_RLZED_R {
        EP_RLZED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error Interrupt. Any bus error interrupt from the USB device. Refer to Section 13.12.9 Read Error Status (Command: 0xFB, Data: read 1 byte) on page 368"]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVINTST")
            .field("frame", &format_args!("{}", self.frame().bit()))
            .field("ep_fast", &format_args!("{}", self.ep_fast().bit()))
            .field("ep_slow", &format_args!("{}", self.ep_slow().bit()))
            .field("dev_stat", &format_args!("{}", self.dev_stat().bit()))
            .field("ccempty", &format_args!("{}", self.ccempty().bit()))
            .field("cdfull", &format_args!("{}", self.cdfull().bit()))
            .field("rx_endpkt", &format_args!("{}", self.rx_endpkt().bit()))
            .field("tx_endpkt", &format_args!("{}", self.tx_endpkt().bit()))
            .field("ep_rlzed", &format_args!("{}", self.ep_rlzed().bit()))
            .field("err_int", &format_args!("{}", self.err_int().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DEVINTST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "USB Device Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devintst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVINTST_SPEC;
impl crate::RegisterSpec for DEVINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devintst::R`](R) reader structure"]
impl crate::Readable for DEVINTST_SPEC {}
#[doc = "`reset()` method sets DEVINTST to value 0x10"]
impl crate::Resettable for DEVINTST_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
