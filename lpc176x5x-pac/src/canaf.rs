#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Acceptance Filter Register"]
    pub afmr: AFMR,
    #[doc = "0x04 - Standard Frame Individual Start Address Register"]
    pub sff_sa: SFF_SA,
    #[doc = "0x08 - Standard Frame Group Start Address Register"]
    pub sff_grp_sa: SFF_GRP_SA,
    #[doc = "0x0c - Extended Frame Start Address Register"]
    pub eff_sa: EFF_SA,
    #[doc = "0x10 - Extended Frame Group Start Address Register"]
    pub eff_grp_sa: EFF_GRP_SA,
    #[doc = "0x14 - End of AF Tables register"]
    pub endoftable: ENDOFTABLE,
    #[doc = "0x18 - LUT Error Address register"]
    pub luterrad: LUTERRAD,
    #[doc = "0x1c - LUT Error Register"]
    pub luterr: LUTERR,
    #[doc = "0x20 - FullCAN interrupt enable register"]
    pub fcanie: FCANIE,
    #[doc = "0x24 - FullCAN interrupt and capture register0"]
    pub fcanic0: FCANIC0,
    #[doc = "0x28 - FullCAN interrupt and capture register1"]
    pub fcanic1: FCANIC1,
}
#[doc = "AFMR (rw) register accessor: Acceptance Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afmr`]
module"]
pub type AFMR = crate::Reg<afmr::AFMR_SPEC>;
#[doc = "Acceptance Filter Register"]
pub mod afmr;
#[doc = "SFF_SA (rw) register accessor: Standard Frame Individual Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sff_sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sff_sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sff_sa`]
module"]
pub type SFF_SA = crate::Reg<sff_sa::SFF_SA_SPEC>;
#[doc = "Standard Frame Individual Start Address Register"]
pub mod sff_sa;
#[doc = "SFF_GRP_SA (rw) register accessor: Standard Frame Group Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sff_grp_sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sff_grp_sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sff_grp_sa`]
module"]
pub type SFF_GRP_SA = crate::Reg<sff_grp_sa::SFF_GRP_SA_SPEC>;
#[doc = "Standard Frame Group Start Address Register"]
pub mod sff_grp_sa;
#[doc = "EFF_SA (rw) register accessor: Extended Frame Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eff_sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eff_sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eff_sa`]
module"]
pub type EFF_SA = crate::Reg<eff_sa::EFF_SA_SPEC>;
#[doc = "Extended Frame Start Address Register"]
pub mod eff_sa;
#[doc = "EFF_GRP_SA (rw) register accessor: Extended Frame Group Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eff_grp_sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eff_grp_sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eff_grp_sa`]
module"]
pub type EFF_GRP_SA = crate::Reg<eff_grp_sa::EFF_GRP_SA_SPEC>;
#[doc = "Extended Frame Group Start Address Register"]
pub mod eff_grp_sa;
#[doc = "ENDOFTABLE (rw) register accessor: End of AF Tables register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endoftable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endoftable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endoftable`]
module"]
pub type ENDOFTABLE = crate::Reg<endoftable::ENDOFTABLE_SPEC>;
#[doc = "End of AF Tables register"]
pub mod endoftable;
#[doc = "LUTERRAD (r) register accessor: LUT Error Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`luterrad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@luterrad`]
module"]
pub type LUTERRAD = crate::Reg<luterrad::LUTERRAD_SPEC>;
#[doc = "LUT Error Address register"]
pub mod luterrad;
#[doc = "LUTERR (r) register accessor: LUT Error Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`luterr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@luterr`]
module"]
pub type LUTERR = crate::Reg<luterr::LUTERR_SPEC>;
#[doc = "LUT Error Register"]
pub mod luterr;
#[doc = "FCANIE (rw) register accessor: FullCAN interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcanie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcanie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcanie`]
module"]
pub type FCANIE = crate::Reg<fcanie::FCANIE_SPEC>;
#[doc = "FullCAN interrupt enable register"]
pub mod fcanie;
#[doc = "FCANIC0 (rw) register accessor: FullCAN interrupt and capture register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcanic0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcanic0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcanic0`]
module"]
pub type FCANIC0 = crate::Reg<fcanic0::FCANIC0_SPEC>;
#[doc = "FullCAN interrupt and capture register0"]
pub mod fcanic0;
#[doc = "FCANIC1 (rw) register accessor: FullCAN interrupt and capture register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcanic1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcanic1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcanic1`]
module"]
pub type FCANIC1 = crate::Reg<fcanic1::FCANIC1_SPEC>;
#[doc = "FullCAN interrupt and capture register1"]
pub mod fcanic1;
