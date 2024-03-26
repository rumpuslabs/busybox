#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 0. Selects the serial clock rate, bus type, and data size."]
    pub cr0: CR0,
    #[doc = "0x04 - Control Register 1. Selects master/slave and other modes."]
    pub cr1: CR1,
    #[doc = "0x08 - Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO."]
    pub dr: DR,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
    #[doc = "0x10 - Clock Prescale Register"]
    pub cpsr: CPSR,
    #[doc = "0x14 - Interrupt Mask Set and Clear Register"]
    pub imsc: IMSC,
    #[doc = "0x18 - Raw Interrupt Status Register"]
    pub ris: RIS,
    #[doc = "0x1c - Masked Interrupt Status Register"]
    pub mis: MIS,
    #[doc = "0x20 - SSPICR Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x24 - SSP0 DMA control register"]
    pub dmacr: DMACR,
}
#[doc = "CR0 (rw) register accessor: Control Register 0. Selects the serial clock rate, bus type, and data size.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`]
module"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "Control Register 0. Selects the serial clock rate, bus type, and data size."]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: Control Register 1. Selects master/slave and other modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control Register 1. Selects master/slave and other modes."]
pub mod cr1;
#[doc = "DR (rw) register accessor: Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`]. WARN: The register is **modified** in some way after a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO."]
pub mod dr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "CPSR (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsr`]
module"]
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod cpsr;
#[doc = "IMSC (rw) register accessor: Interrupt Mask Set and Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imsc`]
module"]
pub type IMSC = crate::Reg<imsc::IMSC_SPEC>;
#[doc = "Interrupt Mask Set and Clear Register"]
pub mod imsc;
#[doc = "RIS (r) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw Interrupt Status Register"]
pub mod ris;
#[doc = "MIS (r) register accessor: Masked Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked Interrupt Status Register"]
pub mod mis;
#[doc = "ICR (w) register accessor: SSPICR Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "SSPICR Interrupt Clear Register"]
pub mod icr;
#[doc = "DMACR (rw) register accessor: SSP0 DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`]
module"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "SSP0 DMA control register"]
pub mod dmacr;
