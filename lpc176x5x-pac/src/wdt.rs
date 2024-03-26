#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer."]
    pub mod_: MOD,
    #[doc = "0x04 - Watchdog timer constant register. The value in this register determines the time-out value."]
    pub tc: TC,
    #[doc = "0x08 - Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC."]
    pub feed: FEED,
    #[doc = "0x0c - Watchdog timer value register. This register reads out the current value of the Watchdog timer."]
    pub tv: TV,
    #[doc = "0x10 - Watchdog clock select register."]
    pub clksel: CLKSEL,
}
#[doc = "MOD (rw) register accessor: Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mod_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mod_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mod_`]
module"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer."]
pub mod mod_;
#[doc = "TC (rw) register accessor: Watchdog timer constant register. The value in this register determines the time-out value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc`]
module"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Watchdog timer constant register. The value in this register determines the time-out value."]
pub mod tc;
#[doc = "FEED (w) register accessor: Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feed`]
module"]
pub type FEED = crate::Reg<feed::FEED_SPEC>;
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC."]
pub mod feed;
#[doc = "TV (r) register accessor: Watchdog timer value register. This register reads out the current value of the Watchdog timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv`]
module"]
pub type TV = crate::Reg<tv::TV_SPEC>;
#[doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer."]
pub mod tv;
#[doc = "CLKSEL (rw) register accessor: Watchdog clock select register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel`]
module"]
pub type CLKSEL = crate::Reg<clksel::CLKSEL_SPEC>;
#[doc = "Watchdog clock select register."]
pub mod clksel;
