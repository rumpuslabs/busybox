#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Compare register"]
    pub compval: COMPVAL,
    #[doc = "0x04 - Mask register. This register holds the 32-bit mask value. A 1 written to any bit will force a compare on the corresponding bit of the counter and compare register."]
    pub mask: MASK,
    #[doc = "0x08 - Control register."]
    pub ctrl: CTRL,
    #[doc = "0x0c - 32-bit counter"]
    pub counter: COUNTER,
}
#[doc = "COMPVAL (rw) register accessor: Compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compval`]
module"]
pub type COMPVAL = crate::Reg<compval::COMPVAL_SPEC>;
#[doc = "Compare register"]
pub mod compval;
#[doc = "MASK (rw) register accessor: Mask register. This register holds the 32-bit mask value. A 1 written to any bit will force a compare on the corresponding bit of the counter and compare register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask register. This register holds the 32-bit mask value. A 1 written to any bit will force a compare on the corresponding bit of the counter and compare register."]
pub mod mask;
#[doc = "CTRL (rw) register accessor: Control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register."]
pub mod ctrl;
#[doc = "COUNTER (rw) register accessor: 32-bit counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter`]
module"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "32-bit counter"]
pub mod counter;
