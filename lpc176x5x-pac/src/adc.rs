#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur."]
    pub cr: CR,
    #[doc = "0x04 - A/D Global Data Register. This register contains the ADC's DONE bit and the result of the most recent A/D conversion."]
    pub gdr: GDR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt."]
    pub inten: INTEN,
    #[doc = "0x10..0x30 - A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0."]
    pub dr: [DR; 8],
    #[doc = "0x30 - A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt/DMA flag."]
    pub stat: STAT,
    #[doc = "0x34 - ADC trim register."]
    pub trm: TRM,
}
#[doc = "CR (rw) register accessor: A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur."]
pub mod cr;
#[doc = "GDR (rw) register accessor: A/D Global Data Register. This register contains the ADC's DONE bit and the result of the most recent A/D conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdr`]
module"]
pub type GDR = crate::Reg<gdr::GDR_SPEC>;
#[doc = "A/D Global Data Register. This register contains the ADC's DONE bit and the result of the most recent A/D conversion."]
pub mod gdr;
#[doc = "INTEN (rw) register accessor: A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt."]
pub mod inten;
#[doc = "DR (r) register accessor: A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0."]
pub mod dr;
#[doc = "STAT (r) register accessor: A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt/DMA flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt/DMA flag."]
pub mod stat;
#[doc = "TRM (rw) register accessor: ADC trim register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trm`]
module"]
pub type TRM = crate::Reg<trm::TRM_SPEC>;
#[doc = "ADC trim register."]
pub mod trm;
