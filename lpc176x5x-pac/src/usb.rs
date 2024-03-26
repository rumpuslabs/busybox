#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - OTG Interrupt Status"]
    pub intst: INTST,
    #[doc = "0x104 - OTG Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x108 - OTG Interrupt Set"]
    pub intset: INTSET,
    #[doc = "0x10c - OTG Interrupt Clear"]
    pub intclr: INTCLR,
    #[doc = "0x110 - OTG Status and Control and USB port select"]
    pub stctrl: STCTRL,
    #[doc = "0x114 - OTG Timer"]
    pub tmr: TMR,
    _reserved6: [u8; 0xe8],
    #[doc = "0x200 - USB Device Interrupt Status"]
    pub devintst: DEVINTST,
    #[doc = "0x204 - USB Device Interrupt Enable"]
    pub devinten: DEVINTEN,
    #[doc = "0x208 - USB Device Interrupt Clear"]
    pub devintclr: DEVINTCLR,
    #[doc = "0x20c - USB Device Interrupt Set"]
    pub devintset: DEVINTSET,
    #[doc = "0x210 - USB Command Code"]
    pub cmdcode: CMDCODE,
    #[doc = "0x214 - USB Command Data"]
    pub cmddata: CMDDATA,
    #[doc = "0x218 - USB Receive Data"]
    pub rxdata: RXDATA,
    #[doc = "0x21c - USB Transmit Data"]
    pub txdata: TXDATA,
    #[doc = "0x220 - USB Receive Packet Length"]
    pub rxplen: RXPLEN,
    #[doc = "0x224 - USB Transmit Packet Length"]
    pub txplen: TXPLEN,
    #[doc = "0x228 - USB Control"]
    pub ctrl: CTRL,
    #[doc = "0x22c - USB Device Interrupt Priority"]
    pub devintpri: DEVINTPRI,
    #[doc = "0x230 - USB Endpoint Interrupt Status"]
    pub epintst: EPINTST,
    #[doc = "0x234 - USB Endpoint Interrupt Enable"]
    pub epinten: EPINTEN,
    #[doc = "0x238 - USB Endpoint Interrupt Clear"]
    pub epintclr: EPINTCLR,
    #[doc = "0x23c - USB Endpoint Interrupt Set"]
    pub epintset: EPINTSET,
    #[doc = "0x240 - USB Endpoint Priority"]
    pub epintpri: EPINTPRI,
    #[doc = "0x244 - USB Realize Endpoint"]
    pub reep: REEP,
    #[doc = "0x248 - USB Endpoint Index"]
    pub epind: EPIND,
    #[doc = "0x24c - USB MaxPacketSize"]
    pub maxpsize: MAXPSIZE,
    #[doc = "0x250 - USB DMA Request Status"]
    pub dmarst: DMARST,
    #[doc = "0x254 - USB DMA Request Clear"]
    pub dmarclr: DMARCLR,
    #[doc = "0x258 - USB DMA Request Set"]
    pub dmarset: DMARSET,
    _reserved29: [u8; 0x24],
    #[doc = "0x280 - USB UDCA Head"]
    pub udcah: UDCAH,
    #[doc = "0x284 - USB Endpoint DMA Status"]
    pub epdmast: EPDMAST,
    #[doc = "0x288 - USB Endpoint DMA Enable"]
    pub epdmaen: EPDMAEN,
    #[doc = "0x28c - USB Endpoint DMA Disable"]
    pub epdmadis: EPDMADIS,
    #[doc = "0x290 - USB DMA Interrupt Status"]
    pub dmaintst: DMAINTST,
    #[doc = "0x294 - USB DMA Interrupt Enable"]
    pub dmainten: DMAINTEN,
    _reserved35: [u8; 0x08],
    #[doc = "0x2a0 - USB End of Transfer Interrupt Status"]
    pub eotintst: EOTINTST,
    #[doc = "0x2a4 - USB End of Transfer Interrupt Clear"]
    pub eotintclr: EOTINTCLR,
    #[doc = "0x2a8 - USB End of Transfer Interrupt Set"]
    pub eotintset: EOTINTSET,
    #[doc = "0x2ac - USB New DD Request Interrupt Status"]
    pub nddrintst: NDDRINTST,
    #[doc = "0x2b0 - USB New DD Request Interrupt Clear"]
    pub nddrintclr: NDDRINTCLR,
    #[doc = "0x2b4 - USB New DD Request Interrupt Set"]
    pub nddrintset: NDDRINTSET,
    #[doc = "0x2b8 - USB System Error Interrupt Status"]
    pub syserrintst: SYSERRINTST,
    #[doc = "0x2bc - USB System Error Interrupt Clear"]
    pub syserrintclr: SYSERRINTCLR,
    #[doc = "0x2c0 - USB System Error Interrupt Set"]
    pub syserrintset: SYSERRINTSET,
    _reserved44: [u8; 0x3c],
    _reserved_44_i2c: [u8; 0x04],
    #[doc = "0x304 - I2C Status"]
    pub i2c_sts: I2C_STS,
    #[doc = "0x308 - I2C Control"]
    pub i2c_ctl: I2C_CTL,
    #[doc = "0x30c - I2C Clock High"]
    pub i2c_clkhi: I2C_CLKHI,
    #[doc = "0x310 - I2C Clock Low"]
    pub i2c_clklo: I2C_CLKLO,
    _reserved49: [u8; 0x0ce0],
    _reserved_49_otgclkctrl: [u8; 0x04],
    _reserved_50_otgclkst: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x300 - I2C Transmit"]
    #[inline(always)]
    pub const fn i2c_wo(&self) -> &I2C_WO {
        unsafe { &*(self as *const Self).cast::<u8>().add(768usize).cast() }
    }
    #[doc = "0x300 - I2C Receive"]
    #[inline(always)]
    pub const fn i2c_rx(&self) -> &I2C_RX {
        unsafe { &*(self as *const Self).cast::<u8>().add(768usize).cast() }
    }
    #[doc = "0xff4 - OTG clock controller"]
    #[inline(always)]
    pub const fn otgclkctrl(&self) -> &OTGCLKCTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(4084usize).cast() }
    }
    #[doc = "0xff4 - USB Clock Control"]
    #[inline(always)]
    pub const fn usbclkctrl(&self) -> &USBCLKCTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(4084usize).cast() }
    }
    #[doc = "0xff8 - OTG clock status"]
    #[inline(always)]
    pub const fn otgclkst(&self) -> &OTGCLKST {
        unsafe { &*(self as *const Self).cast::<u8>().add(4088usize).cast() }
    }
    #[doc = "0xff8 - USB Clock Status"]
    #[inline(always)]
    pub const fn usbclkst(&self) -> &USBCLKST {
        unsafe { &*(self as *const Self).cast::<u8>().add(4088usize).cast() }
    }
}
#[doc = "INTST (r) register accessor: OTG Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intst`]
module"]
pub type INTST = crate::Reg<intst::INTST_SPEC>;
#[doc = "OTG Interrupt Status"]
pub mod intst;
#[doc = "INTEN (rw) register accessor: OTG Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "OTG Interrupt Enable"]
pub mod inten;
#[doc = "INTSET (w) register accessor: OTG Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intset`]
module"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "OTG Interrupt Set"]
pub mod intset;
#[doc = "INTCLR (w) register accessor: OTG Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`]
module"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "OTG Interrupt Clear"]
pub mod intclr;
#[doc = "STCTRL (rw) register accessor: OTG Status and Control and USB port select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stctrl`]
module"]
pub type STCTRL = crate::Reg<stctrl::STCTRL_SPEC>;
#[doc = "OTG Status and Control and USB port select"]
pub mod stctrl;
#[doc = "TMR (rw) register accessor: OTG Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr`]
module"]
pub type TMR = crate::Reg<tmr::TMR_SPEC>;
#[doc = "OTG Timer"]
pub mod tmr;
#[doc = "DEVINTST (r) register accessor: USB Device Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devintst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devintst`]
module"]
pub type DEVINTST = crate::Reg<devintst::DEVINTST_SPEC>;
#[doc = "USB Device Interrupt Status"]
pub mod devintst;
#[doc = "DEVINTEN (rw) register accessor: USB Device Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devinten`]
module"]
pub type DEVINTEN = crate::Reg<devinten::DEVINTEN_SPEC>;
#[doc = "USB Device Interrupt Enable"]
pub mod devinten;
#[doc = "DEVINTCLR (w) register accessor: USB Device Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devintclr`]
module"]
pub type DEVINTCLR = crate::Reg<devintclr::DEVINTCLR_SPEC>;
#[doc = "USB Device Interrupt Clear"]
pub mod devintclr;
#[doc = "DEVINTSET (w) register accessor: USB Device Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devintset`]
module"]
pub type DEVINTSET = crate::Reg<devintset::DEVINTSET_SPEC>;
#[doc = "USB Device Interrupt Set"]
pub mod devintset;
#[doc = "CMDCODE (w) register accessor: USB Command Code\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdcode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdcode`]
module"]
pub type CMDCODE = crate::Reg<cmdcode::CMDCODE_SPEC>;
#[doc = "USB Command Code"]
pub mod cmdcode;
#[doc = "CMDDATA (r) register accessor: USB Command Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata`]
module"]
pub type CMDDATA = crate::Reg<cmddata::CMDDATA_SPEC>;
#[doc = "USB Command Data"]
pub mod cmddata;
#[doc = "RXDATA (r) register accessor: USB Receive Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "USB Receive Data"]
pub mod rxdata;
#[doc = "TXDATA (w) register accessor: USB Transmit Data\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "USB Transmit Data"]
pub mod txdata;
#[doc = "RXPLEN (r) register accessor: USB Receive Packet Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxplen::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxplen`]
module"]
pub type RXPLEN = crate::Reg<rxplen::RXPLEN_SPEC>;
#[doc = "USB Receive Packet Length"]
pub mod rxplen;
#[doc = "TXPLEN (w) register accessor: USB Transmit Packet Length\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txplen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txplen`]
module"]
pub type TXPLEN = crate::Reg<txplen::TXPLEN_SPEC>;
#[doc = "USB Transmit Packet Length"]
pub mod txplen;
#[doc = "CTRL (rw) register accessor: USB Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "USB Control"]
pub mod ctrl;
#[doc = "DEVINTPRI (w) register accessor: USB Device Interrupt Priority\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devintpri::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devintpri`]
module"]
pub type DEVINTPRI = crate::Reg<devintpri::DEVINTPRI_SPEC>;
#[doc = "USB Device Interrupt Priority"]
pub mod devintpri;
#[doc = "EPINTST (r) register accessor: USB Endpoint Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epintst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintst`]
module"]
pub type EPINTST = crate::Reg<epintst::EPINTST_SPEC>;
#[doc = "USB Endpoint Interrupt Status"]
pub mod epintst;
#[doc = "EPINTEN (rw) register accessor: USB Endpoint Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epinten`]
module"]
pub type EPINTEN = crate::Reg<epinten::EPINTEN_SPEC>;
#[doc = "USB Endpoint Interrupt Enable"]
pub mod epinten;
#[doc = "EPINTCLR (w) register accessor: USB Endpoint Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintclr`]
module"]
pub type EPINTCLR = crate::Reg<epintclr::EPINTCLR_SPEC>;
#[doc = "USB Endpoint Interrupt Clear"]
pub mod epintclr;
#[doc = "EPINTSET (w) register accessor: USB Endpoint Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintset`]
module"]
pub type EPINTSET = crate::Reg<epintset::EPINTSET_SPEC>;
#[doc = "USB Endpoint Interrupt Set"]
pub mod epintset;
#[doc = "EPINTPRI (w) register accessor: USB Endpoint Priority\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epintpri::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintpri`]
module"]
pub type EPINTPRI = crate::Reg<epintpri::EPINTPRI_SPEC>;
#[doc = "USB Endpoint Priority"]
pub mod epintpri;
#[doc = "REEP (rw) register accessor: USB Realize Endpoint\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reep`]
module"]
pub type REEP = crate::Reg<reep::REEP_SPEC>;
#[doc = "USB Realize Endpoint"]
pub mod reep;
#[doc = "EPIND (w) register accessor: USB Endpoint Index\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epind::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epind`]
module"]
pub type EPIND = crate::Reg<epind::EPIND_SPEC>;
#[doc = "USB Endpoint Index"]
pub mod epind;
#[doc = "MAXPSIZE (rw) register accessor: USB MaxPacketSize\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxpsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxpsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxpsize`]
module"]
pub type MAXPSIZE = crate::Reg<maxpsize::MAXPSIZE_SPEC>;
#[doc = "USB MaxPacketSize"]
pub mod maxpsize;
#[doc = "DMARST (r) register accessor: USB DMA Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarst`]
module"]
pub type DMARST = crate::Reg<dmarst::DMARST_SPEC>;
#[doc = "USB DMA Request Status"]
pub mod dmarst;
#[doc = "DMARCLR (w) register accessor: USB DMA Request Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarclr`]
module"]
pub type DMARCLR = crate::Reg<dmarclr::DMARCLR_SPEC>;
#[doc = "USB DMA Request Clear"]
pub mod dmarclr;
#[doc = "DMARSET (w) register accessor: USB DMA Request Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarset`]
module"]
pub type DMARSET = crate::Reg<dmarset::DMARSET_SPEC>;
#[doc = "USB DMA Request Set"]
pub mod dmarset;
#[doc = "UDCAH (rw) register accessor: USB UDCA Head\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udcah::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udcah::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udcah`]
module"]
pub type UDCAH = crate::Reg<udcah::UDCAH_SPEC>;
#[doc = "USB UDCA Head"]
pub mod udcah;
#[doc = "EPDMAST (r) register accessor: USB Endpoint DMA Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epdmast::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epdmast`]
module"]
pub type EPDMAST = crate::Reg<epdmast::EPDMAST_SPEC>;
#[doc = "USB Endpoint DMA Status"]
pub mod epdmast;
#[doc = "EPDMAEN (w) register accessor: USB Endpoint DMA Enable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epdmaen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epdmaen`]
module"]
pub type EPDMAEN = crate::Reg<epdmaen::EPDMAEN_SPEC>;
#[doc = "USB Endpoint DMA Enable"]
pub mod epdmaen;
#[doc = "EPDMADIS (w) register accessor: USB Endpoint DMA Disable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epdmadis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epdmadis`]
module"]
pub type EPDMADIS = crate::Reg<epdmadis::EPDMADIS_SPEC>;
#[doc = "USB Endpoint DMA Disable"]
pub mod epdmadis;
#[doc = "DMAINTST (r) register accessor: USB DMA Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaintst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaintst`]
module"]
pub type DMAINTST = crate::Reg<dmaintst::DMAINTST_SPEC>;
#[doc = "USB DMA Interrupt Status"]
pub mod dmaintst;
#[doc = "DMAINTEN (rw) register accessor: USB DMA Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmainten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmainten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmainten`]
module"]
pub type DMAINTEN = crate::Reg<dmainten::DMAINTEN_SPEC>;
#[doc = "USB DMA Interrupt Enable"]
pub mod dmainten;
#[doc = "EOTINTST (r) register accessor: USB End of Transfer Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eotintst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eotintst`]
module"]
pub type EOTINTST = crate::Reg<eotintst::EOTINTST_SPEC>;
#[doc = "USB End of Transfer Interrupt Status"]
pub mod eotintst;
#[doc = "EOTINTCLR (w) register accessor: USB End of Transfer Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eotintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eotintclr`]
module"]
pub type EOTINTCLR = crate::Reg<eotintclr::EOTINTCLR_SPEC>;
#[doc = "USB End of Transfer Interrupt Clear"]
pub mod eotintclr;
#[doc = "EOTINTSET (w) register accessor: USB End of Transfer Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eotintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eotintset`]
module"]
pub type EOTINTSET = crate::Reg<eotintset::EOTINTSET_SPEC>;
#[doc = "USB End of Transfer Interrupt Set"]
pub mod eotintset;
#[doc = "NDDRINTST (r) register accessor: USB New DD Request Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nddrintst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nddrintst`]
module"]
pub type NDDRINTST = crate::Reg<nddrintst::NDDRINTST_SPEC>;
#[doc = "USB New DD Request Interrupt Status"]
pub mod nddrintst;
#[doc = "NDDRINTCLR (w) register accessor: USB New DD Request Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nddrintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nddrintclr`]
module"]
pub type NDDRINTCLR = crate::Reg<nddrintclr::NDDRINTCLR_SPEC>;
#[doc = "USB New DD Request Interrupt Clear"]
pub mod nddrintclr;
#[doc = "NDDRINTSET (w) register accessor: USB New DD Request Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nddrintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nddrintset`]
module"]
pub type NDDRINTSET = crate::Reg<nddrintset::NDDRINTSET_SPEC>;
#[doc = "USB New DD Request Interrupt Set"]
pub mod nddrintset;
#[doc = "SYSERRINTST (r) register accessor: USB System Error Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syserrintst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syserrintst`]
module"]
pub type SYSERRINTST = crate::Reg<syserrintst::SYSERRINTST_SPEC>;
#[doc = "USB System Error Interrupt Status"]
pub mod syserrintst;
#[doc = "SYSERRINTCLR (w) register accessor: USB System Error Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syserrintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syserrintclr`]
module"]
pub type SYSERRINTCLR = crate::Reg<syserrintclr::SYSERRINTCLR_SPEC>;
#[doc = "USB System Error Interrupt Clear"]
pub mod syserrintclr;
#[doc = "SYSERRINTSET (w) register accessor: USB System Error Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syserrintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syserrintset`]
module"]
pub type SYSERRINTSET = crate::Reg<syserrintset::SYSERRINTSET_SPEC>;
#[doc = "USB System Error Interrupt Set"]
pub mod syserrintset;
#[doc = "I2C_RX (r) register accessor: I2C Receive\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_rx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_rx`]
module"]
pub type I2C_RX = crate::Reg<i2c_rx::I2C_RX_SPEC>;
#[doc = "I2C Receive"]
pub mod i2c_rx;
#[doc = "I2C_WO (w) register accessor: I2C Transmit\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_wo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_wo`]
module"]
pub type I2C_WO = crate::Reg<i2c_wo::I2C_WO_SPEC>;
#[doc = "I2C Transmit"]
pub mod i2c_wo;
#[doc = "I2C_STS (r) register accessor: I2C Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_sts`]
module"]
pub type I2C_STS = crate::Reg<i2c_sts::I2C_STS_SPEC>;
#[doc = "I2C Status"]
pub mod i2c_sts;
#[doc = "I2C_CTL (rw) register accessor: I2C Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_ctl`]
module"]
pub type I2C_CTL = crate::Reg<i2c_ctl::I2C_CTL_SPEC>;
#[doc = "I2C Control"]
pub mod i2c_ctl;
#[doc = "I2C_CLKHI (rw) register accessor: I2C Clock High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_clkhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_clkhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_clkhi`]
module"]
pub type I2C_CLKHI = crate::Reg<i2c_clkhi::I2C_CLKHI_SPEC>;
#[doc = "I2C Clock High"]
pub mod i2c_clkhi;
#[doc = "I2C_CLKLO (w) register accessor: I2C Clock Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_clklo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_clklo`]
module"]
pub type I2C_CLKLO = crate::Reg<i2c_clklo::I2C_CLKLO_SPEC>;
#[doc = "I2C Clock Low"]
pub mod i2c_clklo;
#[doc = "USBCLKCTRL (rw) register accessor: USB Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbclkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbclkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkctrl`]
module"]
pub type USBCLKCTRL = crate::Reg<usbclkctrl::USBCLKCTRL_SPEC>;
#[doc = "USB Clock Control"]
pub mod usbclkctrl;
#[doc = "OTGCLKCTRL (rw) register accessor: OTG clock controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otgclkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otgclkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otgclkctrl`]
module"]
pub type OTGCLKCTRL = crate::Reg<otgclkctrl::OTGCLKCTRL_SPEC>;
#[doc = "OTG clock controller"]
pub mod otgclkctrl;
#[doc = "USBCLKST (r) register accessor: USB Clock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbclkst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkst`]
module"]
pub type USBCLKST = crate::Reg<usbclkst::USBCLKST_SPEC>;
#[doc = "USB Clock Status"]
pub mod usbclkst;
#[doc = "OTGCLKST (r) register accessor: OTG clock status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otgclkst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otgclkst`]
module"]
pub type OTGCLKST = crate::Reg<otgclkst::OTGCLKST_SPEC>;
#[doc = "OTG clock status"]
pub mod otgclkst;
