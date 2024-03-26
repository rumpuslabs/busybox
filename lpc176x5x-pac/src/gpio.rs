#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Port Direction control register."]
    pub dir: crate::ArrayProxy<DIR, 5, 0x20>,
    _reserved1: [u8; 0x10],
    #[doc = "0x10 - Mask register for Port."]
    pub mask: crate::ArrayProxy<MASK, 5, 0x20>,
    _reserved2: [u8; 0x04],
    #[doc = "0x14 - Port Pin value register using FIOMASK."]
    pub pin: crate::ArrayProxy<PIN, 5, 0x20>,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - Port Output Set register using FIOMASK."]
    pub set: crate::ArrayProxy<SET, 5, 0x20>,
    _reserved4: [u8; 0x04],
    #[doc = "0x1c - Port Output Clear register using FIOMASK."]
    pub clr: crate::ArrayProxy<CLR, 5, 0x20>,
}
#[doc = "DIR (rw) register accessor: GPIO Port Direction control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "GPIO Port Direction control register."]
pub mod dir;
#[doc = "MASK (rw) register accessor: Mask register for Port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask register for Port."]
pub mod mask;
#[doc = "PIN (rw) register accessor: Port Pin value register using FIOMASK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`]
module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "Port Pin value register using FIOMASK."]
pub mod pin;
#[doc = "SET (rw) register accessor: Port Output Set register using FIOMASK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set`]
module"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "Port Output Set register using FIOMASK."]
pub mod set;
#[doc = "CLR (w) register accessor: Port Output Clear register using FIOMASK.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Port Output Clear register using FIOMASK."]
pub mod clr;
