#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Control read address"]
    pub con: CON,
    #[doc = "0x04 - PWM Control set address"]
    pub con_set: CON_SET,
    #[doc = "0x08 - PWM Control clear address"]
    pub con_clr: CON_CLR,
    #[doc = "0x0c - Capture Control read address"]
    pub capcon: CAPCON,
    #[doc = "0x10 - Capture Control set address"]
    pub capcon_set: CAPCON_SET,
    #[doc = "0x14 - Event Control clear address"]
    pub capcon_clr: CAPCON_CLR,
    #[doc = "0x18..0x24 - Timer Counter register"]
    pub tc: [TC; 3],
    #[doc = "0x24..0x30 - Limit register"]
    pub lim: [LIM; 3],
    #[doc = "0x30..0x3c - Match register"]
    pub mat: [MAT; 3],
    #[doc = "0x3c - Dead time register"]
    pub dt: DT,
    #[doc = "0x40 - Communication Pattern register"]
    pub cp: CP,
    #[doc = "0x44..0x50 - Capture register"]
    pub cap: [CAP; 3],
    #[doc = "0x50 - Interrupt Enable read address"]
    pub inten: INTEN,
    #[doc = "0x54 - Interrupt Enable set address"]
    pub inten_set: INTEN_SET,
    #[doc = "0x58 - Interrupt Enable clear address"]
    pub inten_clr: INTEN_CLR,
    #[doc = "0x5c - Count Control read address"]
    pub cntcon: CNTCON,
    #[doc = "0x60 - Count Control set address"]
    pub cntcon_set: CNTCON_SET,
    #[doc = "0x64 - Count Control clear address"]
    pub cntcon_clr: CNTCON_CLR,
    #[doc = "0x68 - Interrupt flags read address"]
    pub intf: INTF,
    #[doc = "0x6c - Interrupt flags set address"]
    pub intf_set: INTF_SET,
    #[doc = "0x70 - Interrupt flags clear address"]
    pub intf_clr: INTF_CLR,
    #[doc = "0x74 - Capture clear address"]
    pub cap_clr: CAP_CLR,
}
#[doc = "CON (r) register accessor: PWM Control read address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`con::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con`]
module"]
pub type CON = crate::Reg<con::CON_SPEC>;
#[doc = "PWM Control read address"]
#[path = "mcpwm/con_.rs"]
pub mod con;
#[doc = "CON_SET (w) register accessor: PWM Control set address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con_set`]
module"]
pub type CON_SET = crate::Reg<con_set::CON_SET_SPEC>;
#[doc = "PWM Control set address"]
pub mod con_set;
#[doc = "CON_CLR (w) register accessor: PWM Control clear address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con_clr`]
module"]
pub type CON_CLR = crate::Reg<con_clr::CON_CLR_SPEC>;
#[doc = "PWM Control clear address"]
pub mod con_clr;
#[doc = "CAPCON (r) register accessor: Capture Control read address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capcon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capcon`]
module"]
pub type CAPCON = crate::Reg<capcon::CAPCON_SPEC>;
#[doc = "Capture Control read address"]
pub mod capcon;
#[doc = "CAPCON_SET (w) register accessor: Capture Control set address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capcon_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capcon_set`]
module"]
pub type CAPCON_SET = crate::Reg<capcon_set::CAPCON_SET_SPEC>;
#[doc = "Capture Control set address"]
pub mod capcon_set;
#[doc = "CAPCON_CLR (w) register accessor: Event Control clear address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capcon_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capcon_clr`]
module"]
pub type CAPCON_CLR = crate::Reg<capcon_clr::CAPCON_CLR_SPEC>;
#[doc = "Event Control clear address"]
pub mod capcon_clr;
#[doc = "TC (rw) register accessor: Timer Counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc`]
module"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Timer Counter register"]
pub mod tc;
#[doc = "LIM (rw) register accessor: Limit register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lim`]
module"]
pub type LIM = crate::Reg<lim::LIM_SPEC>;
#[doc = "Limit register"]
pub mod lim;
#[doc = "MAT (rw) register accessor: Match register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mat`]
module"]
pub type MAT = crate::Reg<mat::MAT_SPEC>;
#[doc = "Match register"]
pub mod mat;
#[doc = "DT (rw) register accessor: Dead time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Dead time register"]
pub mod dt;
#[doc = "CP (rw) register accessor: Communication Pattern register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp`]
module"]
pub type CP = crate::Reg<cp::CP_SPEC>;
#[doc = "Communication Pattern register"]
pub mod cp;
#[doc = "CAP (r) register accessor: Capture register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap`]
module"]
pub type CAP = crate::Reg<cap::CAP_SPEC>;
#[doc = "Capture register"]
pub mod cap;
#[doc = "INTEN (r) register accessor: Interrupt Enable read address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable read address"]
pub mod inten;
#[doc = "INTEN_SET (w) register accessor: Interrupt Enable set address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten_set`]
module"]
pub type INTEN_SET = crate::Reg<inten_set::INTEN_SET_SPEC>;
#[doc = "Interrupt Enable set address"]
pub mod inten_set;
#[doc = "INTEN_CLR (w) register accessor: Interrupt Enable clear address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten_clr`]
module"]
pub type INTEN_CLR = crate::Reg<inten_clr::INTEN_CLR_SPEC>;
#[doc = "Interrupt Enable clear address"]
pub mod inten_clr;
#[doc = "INTF (r) register accessor: Interrupt flags read address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt flags read address"]
pub mod intf;
#[doc = "INTF_SET (w) register accessor: Interrupt flags set address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_set`]
module"]
pub type INTF_SET = crate::Reg<intf_set::INTF_SET_SPEC>;
#[doc = "Interrupt flags set address"]
pub mod intf_set;
#[doc = "INTF_CLR (w) register accessor: Interrupt flags clear address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_clr`]
module"]
pub type INTF_CLR = crate::Reg<intf_clr::INTF_CLR_SPEC>;
#[doc = "Interrupt flags clear address"]
pub mod intf_clr;
#[doc = "CNTCON (r) register accessor: Count Control read address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntcon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntcon`]
module"]
pub type CNTCON = crate::Reg<cntcon::CNTCON_SPEC>;
#[doc = "Count Control read address"]
pub mod cntcon;
#[doc = "CNTCON_SET (w) register accessor: Count Control set address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcon_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntcon_set`]
module"]
pub type CNTCON_SET = crate::Reg<cntcon_set::CNTCON_SET_SPEC>;
#[doc = "Count Control set address"]
pub mod cntcon_set;
#[doc = "CNTCON_CLR (w) register accessor: Count Control clear address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcon_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntcon_clr`]
module"]
pub type CNTCON_CLR = crate::Reg<cntcon_clr::CNTCON_CLR_SPEC>;
#[doc = "Count Control clear address"]
pub mod cntcon_clr;
#[doc = "CAP_CLR (w) register accessor: Capture clear address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_clr`]
module"]
pub type CAP_CLR = crate::Reg<cap_clr::CAP_CLR_SPEC>;
#[doc = "Capture clear address"]
pub mod cap_clr;
