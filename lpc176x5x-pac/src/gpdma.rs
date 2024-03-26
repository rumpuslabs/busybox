#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Interrupt Status Register"]
    pub intstat: INTSTAT,
    #[doc = "0x04 - DMA Interrupt Terminal Count Request Status Register"]
    pub inttcstat: INTTCSTAT,
    #[doc = "0x08 - DMA Interrupt Terminal Count Request Clear Register"]
    pub inttcclear: INTTCCLEAR,
    #[doc = "0x0c - DMA Interrupt Error Status Register"]
    pub interrstat: INTERRSTAT,
    #[doc = "0x10 - DMA Interrupt Error Clear Register"]
    pub interrclr: INTERRCLR,
    #[doc = "0x14 - DMA Raw Interrupt Terminal Count Status Register"]
    pub rawinttcstat: RAWINTTCSTAT,
    #[doc = "0x18 - DMA Raw Error Interrupt Status Register"]
    pub rawinterrstat: RAWINTERRSTAT,
    #[doc = "0x1c - DMA Enabled Channel Register"]
    pub enbldchns: ENBLDCHNS,
    #[doc = "0x20 - DMA Software Burst Request Register"]
    pub softbreq: SOFTBREQ,
    #[doc = "0x24 - DMA Software Single Request Register"]
    pub softsreq: SOFTSREQ,
    #[doc = "0x28 - DMA Software Last Burst Request Register"]
    pub softlbreq: SOFTLBREQ,
    #[doc = "0x2c - DMA Software Last Single Request Register"]
    pub softlsreq: SOFTLSREQ,
    #[doc = "0x30 - DMA Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x34 - DMA Synchronization Register"]
    pub sync: SYNC,
    _reserved14: [u8; 0xc8],
    #[doc = "0x100 - DMA Channel 0 Source Address Register"]
    pub srcaddr: crate::ArrayProxy<SRCADDR, 8, 0x20>,
    _reserved15: [u8; 0x04],
    #[doc = "0x104 - DMA Channel 0 Destination Address Register"]
    pub destaddr: crate::ArrayProxy<DESTADDR, 8, 0x20>,
    _reserved16: [u8; 0x04],
    #[doc = "0x108 - DMA Channel 0 Linked List Item Register"]
    pub lli: crate::ArrayProxy<LLI, 8, 0x20>,
    _reserved17: [u8; 0x04],
    #[doc = "0x10c - DMA Channel 0 Control Register"]
    pub control: crate::ArrayProxy<CONTROL, 8, 0x20>,
    _reserved18: [u8; 0x04],
    #[doc = "0x110 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub cconfig: crate::ArrayProxy<CCONFIG, 8, 0x20>,
}
#[doc = "INTSTAT (r) register accessor: DMA Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "DMA Interrupt Status Register"]
pub mod intstat;
#[doc = "INTTCSTAT (r) register accessor: DMA Interrupt Terminal Count Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttcstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttcstat`]
module"]
pub type INTTCSTAT = crate::Reg<inttcstat::INTTCSTAT_SPEC>;
#[doc = "DMA Interrupt Terminal Count Request Status Register"]
pub mod inttcstat;
#[doc = "INTTCCLEAR (w) register accessor: DMA Interrupt Terminal Count Request Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttcclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttcclear`]
module"]
pub type INTTCCLEAR = crate::Reg<inttcclear::INTTCCLEAR_SPEC>;
#[doc = "DMA Interrupt Terminal Count Request Clear Register"]
pub mod inttcclear;
#[doc = "INTERRSTAT (r) register accessor: DMA Interrupt Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrstat`]
module"]
pub type INTERRSTAT = crate::Reg<interrstat::INTERRSTAT_SPEC>;
#[doc = "DMA Interrupt Error Status Register"]
pub mod interrstat;
#[doc = "INTERRCLR (w) register accessor: DMA Interrupt Error Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrclr`]
module"]
pub type INTERRCLR = crate::Reg<interrclr::INTERRCLR_SPEC>;
#[doc = "DMA Interrupt Error Clear Register"]
pub mod interrclr;
#[doc = "RAWINTTCSTAT (r) register accessor: DMA Raw Interrupt Terminal Count Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawinttcstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawinttcstat`]
module"]
pub type RAWINTTCSTAT = crate::Reg<rawinttcstat::RAWINTTCSTAT_SPEC>;
#[doc = "DMA Raw Interrupt Terminal Count Status Register"]
pub mod rawinttcstat;
#[doc = "RAWINTERRSTAT (r) register accessor: DMA Raw Error Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawinterrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawinterrstat`]
module"]
pub type RAWINTERRSTAT = crate::Reg<rawinterrstat::RAWINTERRSTAT_SPEC>;
#[doc = "DMA Raw Error Interrupt Status Register"]
pub mod rawinterrstat;
#[doc = "ENBLDCHNS (r) register accessor: DMA Enabled Channel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enbldchns::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enbldchns`]
module"]
pub type ENBLDCHNS = crate::Reg<enbldchns::ENBLDCHNS_SPEC>;
#[doc = "DMA Enabled Channel Register"]
pub mod enbldchns;
#[doc = "SOFTBREQ (rw) register accessor: DMA Software Burst Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softbreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softbreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softbreq`]
module"]
pub type SOFTBREQ = crate::Reg<softbreq::SOFTBREQ_SPEC>;
#[doc = "DMA Software Burst Request Register"]
pub mod softbreq;
#[doc = "SOFTSREQ (rw) register accessor: DMA Software Single Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softsreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softsreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softsreq`]
module"]
pub type SOFTSREQ = crate::Reg<softsreq::SOFTSREQ_SPEC>;
#[doc = "DMA Software Single Request Register"]
pub mod softsreq;
#[doc = "SOFTLBREQ (rw) register accessor: DMA Software Last Burst Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softlbreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softlbreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softlbreq`]
module"]
pub type SOFTLBREQ = crate::Reg<softlbreq::SOFTLBREQ_SPEC>;
#[doc = "DMA Software Last Burst Request Register"]
pub mod softlbreq;
#[doc = "SOFTLSREQ (rw) register accessor: DMA Software Last Single Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softlsreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softlsreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softlsreq`]
module"]
pub type SOFTLSREQ = crate::Reg<softlsreq::SOFTLSREQ_SPEC>;
#[doc = "DMA Software Last Single Request Register"]
pub mod softlsreq;
#[doc = "CONFIG (rw) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "SYNC (rw) register accessor: DMA Synchronization Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`]
module"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "DMA Synchronization Register"]
pub mod sync;
#[doc = "SRCADDR (rw) register accessor: DMA Channel 0 Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr`]
module"]
pub type SRCADDR = crate::Reg<srcaddr::SRCADDR_SPEC>;
#[doc = "DMA Channel 0 Source Address Register"]
pub mod srcaddr;
#[doc = "DESTADDR (rw) register accessor: DMA Channel 0 Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destaddr`]
module"]
pub type DESTADDR = crate::Reg<destaddr::DESTADDR_SPEC>;
#[doc = "DMA Channel 0 Destination Address Register"]
pub mod destaddr;
#[doc = "LLI (rw) register accessor: DMA Channel 0 Linked List Item Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lli::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lli::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lli`]
module"]
pub type LLI = crate::Reg<lli::LLI_SPEC>;
#[doc = "DMA Channel 0 Linked List Item Register"]
pub mod lli;
#[doc = "CONTROL (rw) register accessor: DMA Channel 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "DMA Channel 0 Control Register"]
pub mod control;
#[doc = "CCONFIG (rw) register accessor: DMA Channel 0 Configuration Register\\[1\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cconfig`]
module"]
pub type CCONFIG = crate::Reg<cconfig::CCONFIG_SPEC>;
#[doc = "DMA Channel 0 Configuration Register\\[1\\]"]
pub mod cconfig;
