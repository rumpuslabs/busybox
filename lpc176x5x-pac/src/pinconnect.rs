#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin function select register 0."]
    pub pinsel0: PINSEL0,
    #[doc = "0x04 - Pin function select register 1."]
    pub pinsel1: PINSEL1,
    #[doc = "0x08 - Pin function select register 2."]
    pub pinsel2: PINSEL2,
    #[doc = "0x0c - Pin function select register 3."]
    pub pinsel3: PINSEL3,
    #[doc = "0x10 - Pin function select register 4"]
    pub pinsel4: PINSEL4,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - Pin function select register 7"]
    pub pinsel7: PINSEL7,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - Pin function select register 9"]
    pub pinsel9: PINSEL9,
    #[doc = "0x28 - Pin function select register 10"]
    pub pinsel10: PINSEL10,
    _reserved8: [u8; 0x14],
    #[doc = "0x40 - Pin mode select register 0"]
    pub pinmode0: PINMODE0,
    #[doc = "0x44 - Pin mode select register 1"]
    pub pinmode1: PINMODE1,
    #[doc = "0x48 - Pin mode select register 2"]
    pub pinmode2: PINMODE2,
    #[doc = "0x4c - Pin mode select register 3."]
    pub pinmode3: PINMODE3,
    #[doc = "0x50 - Pin mode select register 4"]
    pub pinmode4: PINMODE4,
    _reserved13: [u8; 0x08],
    #[doc = "0x5c - Pin mode select register 7"]
    pub pinmode7: PINMODE7,
    _reserved14: [u8; 0x04],
    #[doc = "0x64 - Pin mode select register 9"]
    pub pinmode9: PINMODE9,
    #[doc = "0x68 - Open drain mode control register 0"]
    pub pinmode_od0: PINMODE_OD0,
    #[doc = "0x6c - Open drain mode control register 1"]
    pub pinmode_od1: PINMODE_OD1,
    #[doc = "0x70 - Open drain mode control register 2"]
    pub pinmode_od2: PINMODE_OD2,
    #[doc = "0x74 - Open drain mode control register 3"]
    pub pinmode_od3: PINMODE_OD3,
    #[doc = "0x78 - Open drain mode control register 4"]
    pub pinmode_od4: PINMODE_OD4,
    #[doc = "0x7c - I2C Pin Configuration register"]
    pub i2cpadcfg: I2CPADCFG,
}
#[doc = "PINSEL0 (rw) register accessor: Pin function select register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel0`]
module"]
pub type PINSEL0 = crate::Reg<pinsel0::PINSEL0_SPEC>;
#[doc = "Pin function select register 0."]
pub mod pinsel0;
#[doc = "PINSEL1 (rw) register accessor: Pin function select register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel1`]
module"]
pub type PINSEL1 = crate::Reg<pinsel1::PINSEL1_SPEC>;
#[doc = "Pin function select register 1."]
pub mod pinsel1;
#[doc = "PINSEL2 (rw) register accessor: Pin function select register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel2`]
module"]
pub type PINSEL2 = crate::Reg<pinsel2::PINSEL2_SPEC>;
#[doc = "Pin function select register 2."]
pub mod pinsel2;
#[doc = "PINSEL3 (rw) register accessor: Pin function select register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel3`]
module"]
pub type PINSEL3 = crate::Reg<pinsel3::PINSEL3_SPEC>;
#[doc = "Pin function select register 3."]
pub mod pinsel3;
#[doc = "PINSEL4 (rw) register accessor: Pin function select register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel4`]
module"]
pub type PINSEL4 = crate::Reg<pinsel4::PINSEL4_SPEC>;
#[doc = "Pin function select register 4"]
pub mod pinsel4;
#[doc = "PINSEL7 (rw) register accessor: Pin function select register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel7`]
module"]
pub type PINSEL7 = crate::Reg<pinsel7::PINSEL7_SPEC>;
#[doc = "Pin function select register 7"]
pub mod pinsel7;
#[doc = "PINSEL9 (rw) register accessor: Pin function select register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel9`]
module"]
pub type PINSEL9 = crate::Reg<pinsel9::PINSEL9_SPEC>;
#[doc = "Pin function select register 9"]
pub mod pinsel9;
#[doc = "PINSEL10 (rw) register accessor: Pin function select register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel10`]
module"]
pub type PINSEL10 = crate::Reg<pinsel10::PINSEL10_SPEC>;
#[doc = "Pin function select register 10"]
pub mod pinsel10;
#[doc = "PINMODE0 (rw) register accessor: Pin mode select register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode0`]
module"]
pub type PINMODE0 = crate::Reg<pinmode0::PINMODE0_SPEC>;
#[doc = "Pin mode select register 0"]
pub mod pinmode0;
#[doc = "PINMODE1 (rw) register accessor: Pin mode select register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode1`]
module"]
pub type PINMODE1 = crate::Reg<pinmode1::PINMODE1_SPEC>;
#[doc = "Pin mode select register 1"]
pub mod pinmode1;
#[doc = "PINMODE2 (rw) register accessor: Pin mode select register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode2`]
module"]
pub type PINMODE2 = crate::Reg<pinmode2::PINMODE2_SPEC>;
#[doc = "Pin mode select register 2"]
pub mod pinmode2;
#[doc = "PINMODE3 (rw) register accessor: Pin mode select register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode3`]
module"]
pub type PINMODE3 = crate::Reg<pinmode3::PINMODE3_SPEC>;
#[doc = "Pin mode select register 3."]
pub mod pinmode3;
#[doc = "PINMODE4 (rw) register accessor: Pin mode select register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode4`]
module"]
pub type PINMODE4 = crate::Reg<pinmode4::PINMODE4_SPEC>;
#[doc = "Pin mode select register 4"]
pub mod pinmode4;
#[doc = "PINMODE7 (rw) register accessor: Pin mode select register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode7`]
module"]
pub type PINMODE7 = crate::Reg<pinmode7::PINMODE7_SPEC>;
#[doc = "Pin mode select register 7"]
pub mod pinmode7;
#[doc = "PINMODE9 (rw) register accessor: Pin mode select register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode9`]
module"]
pub type PINMODE9 = crate::Reg<pinmode9::PINMODE9_SPEC>;
#[doc = "Pin mode select register 9"]
pub mod pinmode9;
#[doc = "PINMODE_OD0 (rw) register accessor: Open drain mode control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode_od0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode_od0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode_od0`]
module"]
pub type PINMODE_OD0 = crate::Reg<pinmode_od0::PINMODE_OD0_SPEC>;
#[doc = "Open drain mode control register 0"]
pub mod pinmode_od0;
#[doc = "PINMODE_OD1 (rw) register accessor: Open drain mode control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode_od1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode_od1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode_od1`]
module"]
pub type PINMODE_OD1 = crate::Reg<pinmode_od1::PINMODE_OD1_SPEC>;
#[doc = "Open drain mode control register 1"]
pub mod pinmode_od1;
#[doc = "PINMODE_OD2 (rw) register accessor: Open drain mode control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode_od2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode_od2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode_od2`]
module"]
pub type PINMODE_OD2 = crate::Reg<pinmode_od2::PINMODE_OD2_SPEC>;
#[doc = "Open drain mode control register 2"]
pub mod pinmode_od2;
#[doc = "PINMODE_OD3 (rw) register accessor: Open drain mode control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode_od3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode_od3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode_od3`]
module"]
pub type PINMODE_OD3 = crate::Reg<pinmode_od3::PINMODE_OD3_SPEC>;
#[doc = "Open drain mode control register 3"]
pub mod pinmode_od3;
#[doc = "PINMODE_OD4 (rw) register accessor: Open drain mode control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode_od4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode_od4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode_od4`]
module"]
pub type PINMODE_OD4 = crate::Reg<pinmode_od4::PINMODE_OD4_SPEC>;
#[doc = "Open drain mode control register 4"]
pub mod pinmode_od4;
#[doc = "I2CPADCFG (rw) register accessor: I2C Pin Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cpadcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cpadcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cpadcfg`]
module"]
pub type I2CPADCFG = crate::Reg<i2cpadcfg::I2CPADCFG_SPEC>;
#[doc = "I2C Pin Configuration register"]
pub mod i2cpadcfg;
