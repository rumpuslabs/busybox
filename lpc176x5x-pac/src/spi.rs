#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control Register. This register controls the operation of the SPI."]
    pub cr: CR,
    #[doc = "0x04 - SPI Status Register. This register shows the status of the SPI."]
    pub sr: SR,
    #[doc = "0x08 - SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register."]
    pub dr: DR,
    #[doc = "0x0c - SPI Clock Counter Register. This register controls the frequency of a master's SCK0."]
    pub ccr: CCR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1c - SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface."]
    pub int: INT,
}
#[doc = "CR (rw) register accessor: SPI Control Register. This register controls the operation of the SPI.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "SPI Control Register. This register controls the operation of the SPI."]
pub mod cr;
#[doc = "SR (r) register accessor: SPI Status Register. This register shows the status of the SPI.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SPI Status Register. This register shows the status of the SPI."]
pub mod sr;
#[doc = "DR (rw) register accessor: SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`]. WARN: The register is **modified** in some way after a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register."]
pub mod dr;
#[doc = "CCR (rw) register accessor: SPI Clock Counter Register. This register controls the frequency of a master's SCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "SPI Clock Counter Register. This register controls the frequency of a master's SCK0."]
pub mod ccr;
#[doc = "INT (rw) register accessor: SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface."]
pub mod int;
