#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Accelerator Configuration Register. Controls flash access timing."]
    pub flashcfg: FLASHCFG,
    _reserved1: [u8; 0x7c],
    #[doc = "0x80 - PLL0 Control Register"]
    pub pll0con: PLL0CON,
    #[doc = "0x84 - PLL0 Configuration Register"]
    pub pll0cfg: PLL0CFG,
    #[doc = "0x88 - PLL0 Status Register"]
    pub pll0stat: PLL0STAT,
    #[doc = "0x8c - PLL0 Feed Register"]
    pub pll0feed: PLL0FEED,
    _reserved5: [u8; 0x10],
    #[doc = "0xa0 - PLL1 Control Register"]
    pub pll1con: PLL1CON,
    #[doc = "0xa4 - PLL1 Configuration Register"]
    pub pll1cfg: PLL1CFG,
    #[doc = "0xa8 - PLL1 Status Register"]
    pub pll1stat: PLL1STAT,
    #[doc = "0xac - PLL1 Feed Register"]
    pub pll1feed: PLL1FEED,
    _reserved9: [u8; 0x10],
    #[doc = "0xc0 - Power Control Register"]
    pub pcon: PCON,
    #[doc = "0xc4 - Power Control for Peripherals Register"]
    pub pconp: PCONP,
    _reserved11: [u8; 0x3c],
    #[doc = "0x104 - CPU Clock Configuration Register"]
    pub cclkcfg: CCLKCFG,
    #[doc = "0x108 - USB Clock Configuration Register"]
    pub usbclkcfg: USBCLKCFG,
    #[doc = "0x10c - Clock Source Select Register"]
    pub clksrcsel: CLKSRCSEL,
    #[doc = "0x110 - Allows clearing the current CAN channel sleep state as well as reading that state."]
    pub cansleepclr: CANSLEEPCLR,
    #[doc = "0x114 - Allows reading the wake-up state of the CAN channels."]
    pub canwakeflags: CANWAKEFLAGS,
    _reserved16: [u8; 0x28],
    #[doc = "0x140 - External Interrupt Flag Register"]
    pub extint: EXTINT,
    _reserved17: [u8; 0x04],
    #[doc = "0x148 - External Interrupt Mode register"]
    pub extmode: EXTMODE,
    #[doc = "0x14c - External Interrupt Polarity Register"]
    pub extpolar: EXTPOLAR,
    _reserved19: [u8; 0x30],
    #[doc = "0x180 - Reset Source Identification Register"]
    pub rsid: RSID,
    _reserved20: [u8; 0x1c],
    #[doc = "0x1a0 - System control and status"]
    pub scs: SCS,
    _reserved21: [u8; 0x04],
    #[doc = "0x1a8 - Peripheral Clock Selection register 0."]
    pub pclksel0: PCLKSEL0,
    #[doc = "0x1ac - Peripheral Clock Selection register 1."]
    pub pclksel1: PCLKSEL1,
    _reserved23: [u8; 0x10],
    #[doc = "0x1c0 - USB Interrupt Status"]
    pub usbintst: USBINTST,
    #[doc = "0x1c4 - Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
    pub dmacreqsel: DMACREQSEL,
    #[doc = "0x1c8 - Clock Output Configuration Register"]
    pub clkoutcfg: CLKOUTCFG,
}
#[doc = "FLASHCFG (rw) register accessor: Flash Accelerator Configuration Register. Controls flash access timing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcfg`]
module"]
pub type FLASHCFG = crate::Reg<flashcfg::FLASHCFG_SPEC>;
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing."]
pub mod flashcfg;
#[doc = "PLL0CON (rw) register accessor: PLL0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll0con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll0con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0con`]
module"]
pub type PLL0CON = crate::Reg<pll0con::PLL0CON_SPEC>;
#[doc = "PLL0 Control Register"]
pub mod pll0con;
#[doc = "PLL0CFG (rw) register accessor: PLL0 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll0cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll0cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0cfg`]
module"]
pub type PLL0CFG = crate::Reg<pll0cfg::PLL0CFG_SPEC>;
#[doc = "PLL0 Configuration Register"]
pub mod pll0cfg;
#[doc = "PLL0STAT (r) register accessor: PLL0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll0stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0stat`]
module"]
pub type PLL0STAT = crate::Reg<pll0stat::PLL0STAT_SPEC>;
#[doc = "PLL0 Status Register"]
pub mod pll0stat;
#[doc = "PLL0FEED (w) register accessor: PLL0 Feed Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll0feed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0feed`]
module"]
pub type PLL0FEED = crate::Reg<pll0feed::PLL0FEED_SPEC>;
#[doc = "PLL0 Feed Register"]
pub mod pll0feed;
#[doc = "PLL1CON (rw) register accessor: PLL1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1con`]
module"]
pub type PLL1CON = crate::Reg<pll1con::PLL1CON_SPEC>;
#[doc = "PLL1 Control Register"]
pub mod pll1con;
#[doc = "PLL1CFG (rw) register accessor: PLL1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1cfg`]
module"]
pub type PLL1CFG = crate::Reg<pll1cfg::PLL1CFG_SPEC>;
#[doc = "PLL1 Configuration Register"]
pub mod pll1cfg;
#[doc = "PLL1STAT (r) register accessor: PLL1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1stat`]
module"]
pub type PLL1STAT = crate::Reg<pll1stat::PLL1STAT_SPEC>;
#[doc = "PLL1 Status Register"]
pub mod pll1stat;
#[doc = "PLL1FEED (w) register accessor: PLL1 Feed Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1feed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1feed`]
module"]
pub type PLL1FEED = crate::Reg<pll1feed::PLL1FEED_SPEC>;
#[doc = "PLL1 Feed Register"]
pub mod pll1feed;
#[doc = "PCON (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcon`]
module"]
pub type PCON = crate::Reg<pcon::PCON_SPEC>;
#[doc = "Power Control Register"]
pub mod pcon;
#[doc = "PCONP (rw) register accessor: Power Control for Peripherals Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pconp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pconp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconp`]
module"]
pub type PCONP = crate::Reg<pconp::PCONP_SPEC>;
#[doc = "Power Control for Peripherals Register"]
pub mod pconp;
#[doc = "CCLKCFG (rw) register accessor: CPU Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cclkcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cclkcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cclkcfg`]
module"]
pub type CCLKCFG = crate::Reg<cclkcfg::CCLKCFG_SPEC>;
#[doc = "CPU Clock Configuration Register"]
pub mod cclkcfg;
#[doc = "USBCLKCFG (rw) register accessor: USB Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbclkcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbclkcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkcfg`]
module"]
pub type USBCLKCFG = crate::Reg<usbclkcfg::USBCLKCFG_SPEC>;
#[doc = "USB Clock Configuration Register"]
pub mod usbclkcfg;
#[doc = "CLKSRCSEL (rw) register accessor: Clock Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksrcsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksrcsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksrcsel`]
module"]
pub type CLKSRCSEL = crate::Reg<clksrcsel::CLKSRCSEL_SPEC>;
#[doc = "Clock Source Select Register"]
pub mod clksrcsel;
#[doc = "CANSLEEPCLR (rw) register accessor: Allows clearing the current CAN channel sleep state as well as reading that state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cansleepclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cansleepclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cansleepclr`]
module"]
pub type CANSLEEPCLR = crate::Reg<cansleepclr::CANSLEEPCLR_SPEC>;
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state."]
pub mod cansleepclr;
#[doc = "CANWAKEFLAGS (rw) register accessor: Allows reading the wake-up state of the CAN channels.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`canwakeflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`canwakeflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canwakeflags`]
module"]
pub type CANWAKEFLAGS = crate::Reg<canwakeflags::CANWAKEFLAGS_SPEC>;
#[doc = "Allows reading the wake-up state of the CAN channels."]
pub mod canwakeflags;
#[doc = "EXTINT (rw) register accessor: External Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extint`]
module"]
pub type EXTINT = crate::Reg<extint::EXTINT_SPEC>;
#[doc = "External Interrupt Flag Register"]
pub mod extint;
#[doc = "EXTMODE (rw) register accessor: External Interrupt Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extmode`]
module"]
pub type EXTMODE = crate::Reg<extmode::EXTMODE_SPEC>;
#[doc = "External Interrupt Mode register"]
pub mod extmode;
#[doc = "EXTPOLAR (rw) register accessor: External Interrupt Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extpolar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extpolar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extpolar`]
module"]
pub type EXTPOLAR = crate::Reg<extpolar::EXTPOLAR_SPEC>;
#[doc = "External Interrupt Polarity Register"]
pub mod extpolar;
#[doc = "RSID (rw) register accessor: Reset Source Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsid`]
module"]
pub type RSID = crate::Reg<rsid::RSID_SPEC>;
#[doc = "Reset Source Identification Register"]
pub mod rsid;
#[doc = "SCS (rw) register accessor: System control and status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scs`]
module"]
pub type SCS = crate::Reg<scs::SCS_SPEC>;
#[doc = "System control and status"]
pub mod scs;
#[doc = "PCLKSEL0 (rw) register accessor: Peripheral Clock Selection register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclksel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclksel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclksel0`]
module"]
pub type PCLKSEL0 = crate::Reg<pclksel0::PCLKSEL0_SPEC>;
#[doc = "Peripheral Clock Selection register 0."]
pub mod pclksel0;
#[doc = "PCLKSEL1 (rw) register accessor: Peripheral Clock Selection register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclksel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclksel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclksel1`]
module"]
pub type PCLKSEL1 = crate::Reg<pclksel1::PCLKSEL1_SPEC>;
#[doc = "Peripheral Clock Selection register 1."]
pub mod pclksel1;
#[doc = "USBINTST (rw) register accessor: USB Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbintst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbintst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbintst`]
module"]
pub type USBINTST = crate::Reg<usbintst::USBINTST_SPEC>;
#[doc = "USB Interrupt Status"]
pub mod usbintst;
#[doc = "DMACREQSEL (rw) register accessor: Selects between alternative requests on DMA channels 0 through 7 and 10 through 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacreqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacreqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacreqsel`]
module"]
pub type DMACREQSEL = crate::Reg<dmacreqsel::DMACREQSEL_SPEC>;
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
pub mod dmacreqsel;
#[doc = "CLKOUTCFG (rw) register accessor: Clock Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkoutcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkoutcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutcfg`]
module"]
pub type CLKOUTCFG = crate::Reg<clkoutcfg::CLKOUTCFG_SPEC>;
#[doc = "Clock Output Configuration Register"]
pub mod clkoutcfg;
