#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x04],
    _reserved_1_dlm: [u8; 0x04],
    _reserved_2_fcr: [u8; 0x04],
    #[doc = "0x0c - Line Control Register. Contains controls for frame formatting and break generation."]
    pub lcr: LCR,
    #[doc = "0x10 - Modem Control Register. Contains controls for flow control handshaking and loopback mode."]
    pub mcr: MCR,
    #[doc = "0x14 - Line Status Register. Contains flags for transmit and receive status, including line errors."]
    pub lsr: LSR,
    #[doc = "0x18 - Modem Status Register. Contains handshake signal status flags."]
    pub msr: MSR,
    #[doc = "0x1c - Scratch Pad Register. 8-bit temporary storage for software."]
    pub scr: SCR,
    #[doc = "0x20 - Auto-baud Control Register. Contains controls for the auto-baud feature."]
    pub acr: ACR,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - Fractional Divider Register. Generates a clock input for the baud rate divider."]
    pub fdr: FDR,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Transmit Enable Register. Turns off UART transmitter for use with software flow control."]
    pub ter: TER,
    _reserved11: [u8; 0x18],
    #[doc = "0x4c - RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
    pub rs485ctrl: RS485CTRL,
    #[doc = "0x50 - RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
    pub rs485adrmatch: RS485ADRMATCH,
    #[doc = "0x54 - RS-485/EIA-485 direction control delay."]
    pub rs485dly: RS485DLY,
}
impl RegisterBlock {
    #[doc = "0x00 - DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
    #[inline(always)]
    pub const fn dll(&self) -> &DLL {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - DLAB =0. Transmit Holding Register. The next character to be transmitted is written here."]
    #[inline(always)]
    pub const fn thr(&self) -> &THR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - DLAB =0 Receiver Buffer Register. Contains the next received character to be read."]
    #[inline(always)]
    pub const fn rbr(&self) -> &RBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x04 - DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts."]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
    #[inline(always)]
    pub const fn dlm(&self) -> &DLM {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x08 - FIFO Control Register. Controls UART1 FIFO usage and modes."]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending."]
    #[inline(always)]
    pub const fn iir(&self) -> &IIR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
}
#[doc = "RBR (r) register accessor: DLAB =0 Receiver Buffer Register. Contains the next received character to be read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`]. WARN: The register is **modified** in some way after a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr`]
module"]
pub type RBR = crate::Reg<rbr::RBR_SPEC>;
#[doc = "DLAB =0 Receiver Buffer Register. Contains the next received character to be read."]
pub mod rbr;
#[doc = "THR (w) register accessor: DLAB =0. Transmit Holding Register. The next character to be transmitted is written here.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "DLAB =0. Transmit Holding Register. The next character to be transmitted is written here."]
pub mod thr;
#[doc = "DLL (rw) register accessor: DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`]
module"]
pub type DLL = crate::Reg<dll::DLL_SPEC>;
#[doc = "DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
pub mod dll;
#[doc = "DLM (rw) register accessor: DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlm`]
module"]
pub type DLM = crate::Reg<dlm::DLM_SPEC>;
#[doc = "DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
pub mod dlm;
#[doc = "IER (rw) register accessor: DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts."]
pub mod ier;
#[doc = "IIR (r) register accessor: Interrupt ID Register. Identifies which interrupt(s) are pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`]
module"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub mod iir;
#[doc = "FCR (w) register accessor: FIFO Control Register. Controls UART1 FIFO usage and modes.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FIFO Control Register. Controls UART1 FIFO usage and modes."]
pub mod fcr;
#[doc = "LCR (rw) register accessor: Line Control Register. Contains controls for frame formatting and break generation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`]
module"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub mod lcr;
#[doc = "MCR (rw) register accessor: Modem Control Register. Contains controls for flow control handshaking and loopback mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Modem Control Register. Contains controls for flow control handshaking and loopback mode."]
pub mod mcr;
#[doc = "LSR (r) register accessor: Line Status Register. Contains flags for transmit and receive status, including line errors.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`]. WARN: The register is **modified** in some way after a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub mod lsr;
#[doc = "MSR (r) register accessor: Modem Status Register. Contains handshake signal status flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`]. WARN: The register is **modified** in some way after a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Modem Status Register. Contains handshake signal status flags."]
pub mod msr;
#[doc = "SCR (rw) register accessor: Scratch Pad Register. 8-bit temporary storage for software.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Scratch Pad Register. 8-bit temporary storage for software."]
pub mod scr;
#[doc = "ACR (rw) register accessor: Auto-baud Control Register. Contains controls for the auto-baud feature.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub mod acr;
#[doc = "FDR (rw) register accessor: Fractional Divider Register. Generates a clock input for the baud rate divider.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr`]
module"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub mod fdr;
#[doc = "TER (rw) register accessor: Transmit Enable Register. Turns off UART transmitter for use with software flow control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ter`]
module"]
pub type TER = crate::Reg<ter::TER_SPEC>;
#[doc = "Transmit Enable Register. Turns off UART transmitter for use with software flow control."]
pub mod ter;
#[doc = "RS485CTRL (rw) register accessor: RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs485ctrl`]
module"]
pub type RS485CTRL = crate::Reg<rs485ctrl::RS485CTRL_SPEC>;
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub mod rs485ctrl;
#[doc = "RS485ADRMATCH (rw) register accessor: RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485adrmatch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485adrmatch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs485adrmatch`]
module"]
pub type RS485ADRMATCH = crate::Reg<rs485adrmatch::RS485ADRMATCH_SPEC>;
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub mod rs485adrmatch;
#[doc = "RS485DLY (rw) register accessor: RS-485/EIA-485 direction control delay.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485dly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485dly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs485dly`]
module"]
pub type RS485DLY = crate::Reg<rs485dly::RS485DLY_SPEC>;
#[doc = "RS-485/EIA-485 direction control delay."]
pub mod rs485dly;
