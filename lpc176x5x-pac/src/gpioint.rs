#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO overall Interrupt Status."]
    pub status: STATUS,
    #[doc = "0x04 - GPIO Interrupt Status for Rising edge for Port 0."]
    pub statr0: STATR0,
    #[doc = "0x08 - GPIO Interrupt Status for Falling edge for Port 0."]
    pub statf0: STATF0,
    #[doc = "0x0c - GPIO Interrupt Clear."]
    pub clr0: CLR0,
    #[doc = "0x10 - GPIO Interrupt Enable for Rising edge for Port 0."]
    pub enr0: ENR0,
    #[doc = "0x14 - GPIO Interrupt Enable for Falling edge for Port 0."]
    pub enf0: ENF0,
    _reserved6: [u8; 0x0c],
    #[doc = "0x24 - GPIO Interrupt Status for Rising edge for Port 0."]
    pub statr2: STATR2,
    #[doc = "0x28 - GPIO Interrupt Status for Falling edge for Port 0."]
    pub statf2: STATF2,
    #[doc = "0x2c - GPIO Interrupt Clear."]
    pub clr2: CLR2,
    #[doc = "0x30 - GPIO Interrupt Enable for Rising edge for Port 0."]
    pub enr2: ENR2,
    #[doc = "0x34 - GPIO Interrupt Enable for Falling edge for Port 0."]
    pub enf2: ENF2,
}
#[doc = "STATUS (r) register accessor: GPIO overall Interrupt Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO overall Interrupt Status."]
pub mod status;
#[doc = "STATR0 (r) register accessor: GPIO Interrupt Status for Rising edge for Port 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statr0`]
module"]
pub type STATR0 = crate::Reg<statr0::STATR0_SPEC>;
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub mod statr0;
#[doc = "STATF0 (r) register accessor: GPIO Interrupt Status for Falling edge for Port 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statf0`]
module"]
pub type STATF0 = crate::Reg<statf0::STATF0_SPEC>;
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub mod statf0;
#[doc = "CLR0 (w) register accessor: GPIO Interrupt Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr0`]
module"]
pub type CLR0 = crate::Reg<clr0::CLR0_SPEC>;
#[doc = "GPIO Interrupt Clear."]
pub mod clr0;
#[doc = "ENR0 (rw) register accessor: GPIO Interrupt Enable for Rising edge for Port 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enr0`]
module"]
pub type ENR0 = crate::Reg<enr0::ENR0_SPEC>;
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub mod enr0;
#[doc = "ENF0 (rw) register accessor: GPIO Interrupt Enable for Falling edge for Port 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enf0`]
module"]
pub type ENF0 = crate::Reg<enf0::ENF0_SPEC>;
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub mod enf0;
#[doc = "STATR2 (r) register accessor: GPIO Interrupt Status for Rising edge for Port 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statr2`]
module"]
pub type STATR2 = crate::Reg<statr2::STATR2_SPEC>;
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub mod statr2;
#[doc = "STATF2 (r) register accessor: GPIO Interrupt Status for Falling edge for Port 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statf2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statf2`]
module"]
pub type STATF2 = crate::Reg<statf2::STATF2_SPEC>;
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub mod statf2;
#[doc = "CLR2 (w) register accessor: GPIO Interrupt Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr2`]
module"]
pub type CLR2 = crate::Reg<clr2::CLR2_SPEC>;
#[doc = "GPIO Interrupt Clear."]
pub mod clr2;
#[doc = "ENR2 (rw) register accessor: GPIO Interrupt Enable for Rising edge for Port 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enr2`]
module"]
pub type ENR2 = crate::Reg<enr2::ENR2_SPEC>;
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub mod enr2;
#[doc = "ENF2 (rw) register accessor: GPIO Interrupt Enable for Falling edge for Port 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enf2`]
module"]
pub type ENF2 = crate::Reg<enf2::ENF2_SPEC>;
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub mod enf2;
