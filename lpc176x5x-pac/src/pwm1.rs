#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending."]
    pub ir: IR,
    #[doc = "0x04 - Timer Control Register. The TCR is used to control the Timer Counter functions."]
    pub tcr: TCR,
    #[doc = "0x08 - Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
    pub tc: TC,
    #[doc = "0x0c - Prescale Register. Determines how often the PWM counter is incremented."]
    pub pr: PR,
    #[doc = "0x10 - Prescale Counter. Prescaler for the main PWM counter."]
    pub pc: PC,
    #[doc = "0x14 - Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs."]
    pub mcr: MCR,
    #[doc = "0x18..0x28 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr: [MR; 4],
    #[doc = "0x28 - Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event."]
    pub ccr: CCR,
    #[doc = "0x2c..0x34 - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
    pub cr: [CR; 2],
    _reserved9: [u8; 0x0c],
    #[doc = "0x40 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr4: MR4,
    #[doc = "0x44 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr5: MR4,
    #[doc = "0x48 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr6: MR4,
    #[doc = "0x4c - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
    pub pcr: PCR,
    #[doc = "0x50 - Load Enable Register. Enables use of updated PWM match values."]
    pub ler: LER,
    _reserved14: [u8; 0x1c],
    #[doc = "0x70 - Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
    pub ctcr: CTCR,
}
#[doc = "IR (rw) register accessor: Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending."]
pub mod ir;
#[doc = "TCR (rw) register accessor: Timer Control Register. The TCR is used to control the Timer Counter functions.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions."]
pub mod tcr;
#[doc = "TC (rw) register accessor: Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc`]
module"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
pub mod tc;
#[doc = "PR (rw) register accessor: Prescale Register. Determines how often the PWM counter is incremented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Prescale Register. Determines how often the PWM counter is incremented."]
pub mod pr;
#[doc = "PC (rw) register accessor: Prescale Counter. Prescaler for the main PWM counter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc`]
module"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Prescale Counter. Prescaler for the main PWM counter."]
pub mod pc;
#[doc = "MCR (rw) register accessor: Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs."]
pub mod mcr;
#[doc = "MR (rw) register accessor: Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub mod mr;
#[doc = "CCR (rw) register accessor: Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event."]
pub mod ccr;
#[doc = "CR (rw) register accessor: PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub mod cr;
pub use mr as mr4;
pub use MR as MR4;
#[doc = "PCR (rw) register accessor: PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`]
module"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub mod pcr;
#[doc = "LER (rw) register accessor: Load Enable Register. Enables use of updated PWM match values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ler::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ler::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ler`]
module"]
pub type LER = crate::Reg<ler::LER_SPEC>;
#[doc = "Load Enable Register. Enables use of updated PWM match values."]
pub mod ler;
#[doc = "CTCR (rw) register accessor: Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctcr`]
module"]
pub type CTCR = crate::Reg<ctcr::CTCR_SPEC>;
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
pub mod ctcr;
