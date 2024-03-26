#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub con: CON,
    #[doc = "0x04 - Status register"]
    pub stat: STAT,
    #[doc = "0x08 - Configuration register"]
    pub conf: CONF,
    #[doc = "0x0c - Position register"]
    pub pos: POS,
    #[doc = "0x10 - Maximum position register"]
    pub maxpos: MAXPOS,
    #[doc = "0x14 - Position compare register 0"]
    pub cmpos0: CMPOS0,
    #[doc = "0x18 - Position compare register 1"]
    pub cmpos1: CMPOS1,
    #[doc = "0x1c - Position compare register 2"]
    pub cmpos2: CMPOS2,
    #[doc = "0x20 - Index count register 0"]
    pub inxcnt: INXCNT,
    #[doc = "0x24 - Index compare register 0"]
    pub inxcmp0: INXCMP0,
    #[doc = "0x28 - Velocity timer reload register"]
    pub load: LOAD,
    #[doc = "0x2c - Velocity timer register"]
    pub time: TIME,
    #[doc = "0x30 - Velocity counter register"]
    pub vel: VEL,
    #[doc = "0x34 - Velocity capture register"]
    pub cap: CAP,
    #[doc = "0x38 - Velocity compare register"]
    pub velcomp: VELCOMP,
    #[doc = "0x3c - Digital filter register"]
    pub filter: FILTER,
    _reserved16: [u8; 0x0f98],
    #[doc = "0xfd8 - Interrupt enable clear register"]
    pub iec: IEC,
    #[doc = "0xfdc - Interrupt enable set register"]
    pub ies: IES,
    #[doc = "0xfe0 - Interrupt status register"]
    pub intstat: INTSTAT,
    #[doc = "0xfe4 - Interrupt enable register"]
    pub ie: IE,
    #[doc = "0xfe8 - Interrupt status clear register"]
    pub clr: CLR,
    #[doc = "0xfec - Interrupt status set register"]
    pub set: SET,
}
#[doc = "CON (w) register accessor: Control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con`]
module"]
pub type CON = crate::Reg<con::CON_SPEC>;
#[doc = "Control register"]
#[path = "qei/con_.rs"]
pub mod con;
#[doc = "CONF (rw) register accessor: Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`]
module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configuration register"]
pub mod conf;
#[doc = "STAT (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "POS (r) register accessor: Position register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pos::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pos`]
module"]
pub type POS = crate::Reg<pos::POS_SPEC>;
#[doc = "Position register"]
pub mod pos;
#[doc = "MAXPOS (rw) register accessor: Maximum position register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxpos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxpos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxpos`]
module"]
pub type MAXPOS = crate::Reg<maxpos::MAXPOS_SPEC>;
#[doc = "Maximum position register"]
pub mod maxpos;
#[doc = "CMPOS0 (rw) register accessor: Position compare register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpos0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpos0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpos0`]
module"]
pub type CMPOS0 = crate::Reg<cmpos0::CMPOS0_SPEC>;
#[doc = "Position compare register 0"]
pub mod cmpos0;
#[doc = "CMPOS1 (rw) register accessor: Position compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpos1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpos1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpos1`]
module"]
pub type CMPOS1 = crate::Reg<cmpos1::CMPOS1_SPEC>;
#[doc = "Position compare register 1"]
pub mod cmpos1;
#[doc = "CMPOS2 (rw) register accessor: Position compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpos2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpos2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpos2`]
module"]
pub type CMPOS2 = crate::Reg<cmpos2::CMPOS2_SPEC>;
#[doc = "Position compare register 2"]
pub mod cmpos2;
#[doc = "INXCNT (r) register accessor: Index count register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inxcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inxcnt`]
module"]
pub type INXCNT = crate::Reg<inxcnt::INXCNT_SPEC>;
#[doc = "Index count register 0"]
pub mod inxcnt;
#[doc = "INXCMP0 (rw) register accessor: Index compare register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inxcmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inxcmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inxcmp0`]
module"]
pub type INXCMP0 = crate::Reg<inxcmp0::INXCMP0_SPEC>;
#[doc = "Index compare register 0"]
pub mod inxcmp0;
#[doc = "LOAD (rw) register accessor: Velocity timer reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load`]
module"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Velocity timer reload register"]
pub mod load;
#[doc = "TIME (r) register accessor: Velocity timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`]
module"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "Velocity timer register"]
pub mod time;
#[doc = "VEL (r) register accessor: Velocity counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vel::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vel`]
module"]
pub type VEL = crate::Reg<vel::VEL_SPEC>;
#[doc = "Velocity counter register"]
pub mod vel;
#[doc = "CAP (r) register accessor: Velocity capture register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap`]
module"]
pub type CAP = crate::Reg<cap::CAP_SPEC>;
#[doc = "Velocity capture register"]
pub mod cap;
#[doc = "VELCOMP (rw) register accessor: Velocity compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`velcomp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`velcomp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@velcomp`]
module"]
pub type VELCOMP = crate::Reg<velcomp::VELCOMP_SPEC>;
#[doc = "Velocity compare register"]
pub mod velcomp;
#[doc = "FILTER (rw) register accessor: Digital filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter`]
module"]
pub type FILTER = crate::Reg<filter::FILTER_SPEC>;
#[doc = "Digital filter register"]
pub mod filter;
#[doc = "INTSTAT (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Interrupt status register"]
pub mod intstat;
#[doc = "SET (w) register accessor: Interrupt status set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set`]
module"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "Interrupt status set register"]
pub mod set;
#[doc = "CLR (w) register accessor: Interrupt status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Interrupt status clear register"]
pub mod clr;
#[doc = "IE (r) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ie;
#[doc = "IES (w) register accessor: Interrupt enable set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ies::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ies`]
module"]
pub type IES = crate::Reg<ies::IES_SPEC>;
#[doc = "Interrupt enable set register"]
pub mod ies;
#[doc = "IEC (w) register accessor: Interrupt enable clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iec::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iec`]
module"]
pub type IEC = crate::Reg<iec::IEC_SPEC>;
#[doc = "Interrupt enable clear register"]
pub mod iec;
