#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel."]
    pub dao: DAO,
    #[doc = "0x04 - I2S Digital Audio Input Register. Contains control bits for the I2S receive channel."]
    pub dai: DAI,
    #[doc = "0x08 - I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO."]
    pub txfifo: TXFIFO,
    #[doc = "0x0c - I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO."]
    pub rxfifo: RXFIFO,
    #[doc = "0x10 - I2S Status Feedback Register. Contains status information about the I2S interface."]
    pub state: STATE,
    #[doc = "0x14 - I2S DMA Configuration Register 1. Contains control information for DMA request 1."]
    pub dma1: DMA1,
    #[doc = "0x18 - I2S DMA Configuration Register 2. Contains control information for DMA request 2."]
    pub dma2: DMA2,
    #[doc = "0x1c - I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated."]
    pub irq: IRQ,
    #[doc = "0x20 - I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
    pub txrate: TXRATE,
    #[doc = "0x24 - I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
    pub rxrate: RXRATE,
    #[doc = "0x28 - I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock."]
    pub txbitrate: TXBITRATE,
    #[doc = "0x2c - I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock."]
    pub rxbitrate: RXBITRATE,
    #[doc = "0x30 - I2S Transmit mode control."]
    pub txmode: TXMODE,
    #[doc = "0x34 - I2S Receive mode control."]
    pub rxmode: RXMODE,
}
#[doc = "DAO (rw) register accessor: I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dao::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dao::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dao`]
module"]
pub type DAO = crate::Reg<dao::DAO_SPEC>;
#[doc = "I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel."]
pub mod dao;
#[doc = "DAI (rw) register accessor: I2S Digital Audio Input Register. Contains control bits for the I2S receive channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dai::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dai::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dai`]
module"]
pub type DAI = crate::Reg<dai::DAI_SPEC>;
#[doc = "I2S Digital Audio Input Register. Contains control bits for the I2S receive channel."]
pub mod dai;
#[doc = "TXFIFO (w) register accessor: I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifo`]
module"]
pub type TXFIFO = crate::Reg<txfifo::TXFIFO_SPEC>;
#[doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO."]
pub mod txfifo;
#[doc = "RXFIFO (r) register accessor: I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifo::R`]. WARN: The register is **modified** in some way after a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifo`]
module"]
pub type RXFIFO = crate::Reg<rxfifo::RXFIFO_SPEC>;
#[doc = "I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO."]
pub mod rxfifo;
#[doc = "STATE (r) register accessor: I2S Status Feedback Register. Contains status information about the I2S interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`]
module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "I2S Status Feedback Register. Contains status information about the I2S interface."]
pub mod state;
#[doc = "DMA1 (rw) register accessor: I2S DMA Configuration Register 1. Contains control information for DMA request 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1`]
module"]
pub type DMA1 = crate::Reg<dma1::DMA1_SPEC>;
#[doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1."]
pub mod dma1;
#[doc = "DMA2 (rw) register accessor: I2S DMA Configuration Register 2. Contains control information for DMA request 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2`]
module"]
pub type DMA2 = crate::Reg<dma2::DMA2_SPEC>;
#[doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2."]
pub mod dma2;
#[doc = "IRQ (rw) register accessor: I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq`]
module"]
pub type IRQ = crate::Reg<irq::IRQ_SPEC>;
#[doc = "I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated."]
pub mod irq;
#[doc = "TXRATE (rw) register accessor: I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txrate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txrate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txrate`]
module"]
pub type TXRATE = crate::Reg<txrate::TXRATE_SPEC>;
#[doc = "I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub mod txrate;
#[doc = "RXRATE (rw) register accessor: I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxrate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxrate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxrate`]
module"]
pub type RXRATE = crate::Reg<rxrate::RXRATE_SPEC>;
#[doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub mod rxrate;
#[doc = "TXBITRATE (rw) register accessor: I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbitrate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbitrate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbitrate`]
module"]
pub type TXBITRATE = crate::Reg<txbitrate::TXBITRATE_SPEC>;
#[doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock."]
pub mod txbitrate;
#[doc = "RXBITRATE (rw) register accessor: I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbitrate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxbitrate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbitrate`]
module"]
pub type RXBITRATE = crate::Reg<rxbitrate::RXBITRATE_SPEC>;
#[doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock."]
pub mod rxbitrate;
#[doc = "TXMODE (rw) register accessor: I2S Transmit mode control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmode`]
module"]
pub type TXMODE = crate::Reg<txmode::TXMODE_SPEC>;
#[doc = "I2S Transmit mode control."]
pub mod txmode;
#[doc = "RXMODE (rw) register accessor: I2S Receive mode control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmode`]
module"]
pub type RXMODE = crate::Reg<rxmode::RXMODE_SPEC>;
#[doc = "I2S Receive mode control."]
pub mod rxmode;
