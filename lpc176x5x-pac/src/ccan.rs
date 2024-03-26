#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Central Transmit Status Register"]
    pub txsr: TXSR,
    #[doc = "0x04 - CAN Central Receive Status Register"]
    pub rxsr: RXSR,
    #[doc = "0x08 - CAN Central Miscellaneous Register"]
    pub msr: MSR,
}
#[doc = "TXSR (r) register accessor: CAN Central Transmit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txsr`]
module"]
pub type TXSR = crate::Reg<txsr::TXSR_SPEC>;
#[doc = "CAN Central Transmit Status Register"]
pub mod txsr;
#[doc = "RXSR (r) register accessor: CAN Central Receive Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxsr`]
module"]
pub type RXSR = crate::Reg<rxsr::RXSR_SPEC>;
#[doc = "CAN Central Receive Status Register"]
pub mod rxsr;
#[doc = "MSR (r) register accessor: CAN Central Miscellaneous Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "CAN Central Miscellaneous Register"]
pub mod msr;
