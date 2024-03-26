#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit."]
    pub cr: CR,
    #[doc = "0x04 - DAC Control register. This register controls DMA and timer operation."]
    pub ctrl: CTRL,
    #[doc = "0x08 - DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer."]
    pub cntval: CNTVAL,
}
#[doc = "CR (rw) register accessor: D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit."]
pub mod cr;
#[doc = "CTRL (rw) register accessor: DAC Control register. This register controls DMA and timer operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DAC Control register. This register controls DMA and timer operation."]
pub mod ctrl;
#[doc = "CNTVAL (rw) register accessor: DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntval`]
module"]
pub type CNTVAL = crate::Reg<cntval::CNTVAL_SPEC>;
#[doc = "DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer."]
pub mod cntval;
