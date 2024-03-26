#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC configuration register 1."]
    pub mac1: MAC1,
    #[doc = "0x04 - MAC configuration register 2."]
    pub mac2: MAC2,
    #[doc = "0x08 - Back-to-Back Inter-Packet-Gap register."]
    pub ipgt: IPGT,
    #[doc = "0x0c - Non Back-to-Back Inter-Packet-Gap register."]
    pub ipgr: IPGR,
    #[doc = "0x10 - Collision window / Retry register."]
    pub clrt: CLRT,
    #[doc = "0x14 - Maximum Frame register."]
    pub maxf: MAXF,
    #[doc = "0x18 - PHY Support register."]
    pub supp: SUPP,
    #[doc = "0x1c - Test register."]
    pub test: TEST,
    #[doc = "0x20 - MII Mgmt Configuration register."]
    pub mcfg: MCFG,
    #[doc = "0x24 - MII Mgmt Command register."]
    pub mcmd: MCMD,
    #[doc = "0x28 - MII Mgmt Address register."]
    pub madr: MADR,
    #[doc = "0x2c - MII Mgmt Write Data register."]
    pub mwtd: MWTD,
    #[doc = "0x30 - MII Mgmt Read Data register."]
    pub mrdd: MRDD,
    #[doc = "0x34 - MII Mgmt Indicators register."]
    pub mind: MIND,
    _reserved14: [u8; 0x08],
    #[doc = "0x40 - Station Address 0 register."]
    pub sa0: SA0,
    #[doc = "0x44 - Station Address 1 register."]
    pub sa1: SA1,
    #[doc = "0x48 - Station Address 2 register."]
    pub sa2: SA2,
    _reserved17: [u8; 0xb4],
    #[doc = "0x100 - Command register."]
    pub command: COMMAND,
    #[doc = "0x104 - Status register."]
    pub status: STATUS,
    #[doc = "0x108 - Receive descriptor base address register."]
    pub rxdescriptor: RXDESCRIPTOR,
    #[doc = "0x10c - Receive status base address register."]
    pub rxstatus: RXSTATUS,
    #[doc = "0x110 - Receive number of descriptors register."]
    pub rxdescriptornumber: RXDESCRIPTORNUMBER,
    #[doc = "0x114 - Receive produce index register."]
    pub rxproduceindex: RXPRODUCEINDEX,
    #[doc = "0x118 - Receive consume index register."]
    pub rxconsumeindex: RXCONSUMEINDEX,
    #[doc = "0x11c - Transmit descriptor base address register."]
    pub txdescriptor: TXDESCRIPTOR,
    #[doc = "0x120 - Transmit status base address register."]
    pub txstatus: TXSTATUS,
    #[doc = "0x124 - Transmit number of descriptors register."]
    pub txdescriptornumber: TXDESCRIPTORNUMBER,
    #[doc = "0x128 - Transmit produce index register."]
    pub txproduceindex: TXPRODUCEINDEX,
    #[doc = "0x12c - Transmit consume index register."]
    pub txconsumeindex: TXCONSUMEINDEX,
    _reserved29: [u8; 0x28],
    #[doc = "0x158 - Transmit status vector 0 register."]
    pub tsv0: TSV0,
    #[doc = "0x15c - Transmit status vector 1 register."]
    pub tsv1: TSV1,
    #[doc = "0x160 - Receive status vector register."]
    pub rsv: RSV,
    _reserved32: [u8; 0x0c],
    #[doc = "0x170 - Flow control counter register."]
    pub flowcontrolcounter: FLOWCONTROLCOUNTER,
    #[doc = "0x174 - Flow control status register."]
    pub flowcontrolstatus: FLOWCONTROLSTATUS,
    _reserved34: [u8; 0x88],
    #[doc = "0x200 - Receive filter control register."]
    pub rxfilterctrl: RXFILTERCTRL,
    #[doc = "0x204 - Receive filter WoL status register."]
    pub rxfilterwolstatus: RXFILTERWOLSTATUS,
    #[doc = "0x208 - Receive filter WoL clear register."]
    pub rxfilterwolclear: RXFILTERWOLCLEAR,
    _reserved37: [u8; 0x04],
    #[doc = "0x210 - Hash filter table LSBs register."]
    pub hashfilterl: HASHFILTERL,
    #[doc = "0x214 - Hash filter table MSBs register."]
    pub hashfilterh: HASHFILTERH,
    _reserved39: [u8; 0x0dc8],
    #[doc = "0xfe0 - Interrupt status register."]
    pub intstatus: INTSTATUS,
    #[doc = "0xfe4 - Interrupt enable register."]
    pub intenable: INTENABLE,
    #[doc = "0xfe8 - Interrupt clear register."]
    pub intclear: INTCLEAR,
    #[doc = "0xfec - Interrupt set register."]
    pub intset: INTSET,
    _reserved43: [u8; 0x04],
    #[doc = "0xff4 - Power-down register."]
    pub powerdown: POWERDOWN,
}
#[doc = "MAC1 (rw) register accessor: MAC configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac1`]
module"]
pub type MAC1 = crate::Reg<mac1::MAC1_SPEC>;
#[doc = "MAC configuration register 1."]
pub mod mac1;
#[doc = "MAC2 (rw) register accessor: MAC configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac2`]
module"]
pub type MAC2 = crate::Reg<mac2::MAC2_SPEC>;
#[doc = "MAC configuration register 2."]
pub mod mac2;
#[doc = "IPGT (rw) register accessor: Back-to-Back Inter-Packet-Gap register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipgt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipgt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipgt`]
module"]
pub type IPGT = crate::Reg<ipgt::IPGT_SPEC>;
#[doc = "Back-to-Back Inter-Packet-Gap register."]
pub mod ipgt;
#[doc = "IPGR (rw) register accessor: Non Back-to-Back Inter-Packet-Gap register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipgr`]
module"]
pub type IPGR = crate::Reg<ipgr::IPGR_SPEC>;
#[doc = "Non Back-to-Back Inter-Packet-Gap register."]
pub mod ipgr;
#[doc = "CLRT (rw) register accessor: Collision window / Retry register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clrt`]
module"]
pub type CLRT = crate::Reg<clrt::CLRT_SPEC>;
#[doc = "Collision window / Retry register."]
pub mod clrt;
#[doc = "MAXF (rw) register accessor: Maximum Frame register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxf`]
module"]
pub type MAXF = crate::Reg<maxf::MAXF_SPEC>;
#[doc = "Maximum Frame register."]
pub mod maxf;
#[doc = "SUPP (rw) register accessor: PHY Support register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`supp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`supp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@supp`]
module"]
pub type SUPP = crate::Reg<supp::SUPP_SPEC>;
#[doc = "PHY Support register."]
pub mod supp;
#[doc = "TEST (rw) register accessor: Test register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test register."]
pub mod test;
#[doc = "MCFG (rw) register accessor: MII Mgmt Configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg`]
module"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "MII Mgmt Configuration register."]
pub mod mcfg;
#[doc = "MCMD (rw) register accessor: MII Mgmt Command register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmd`]
module"]
pub type MCMD = crate::Reg<mcmd::MCMD_SPEC>;
#[doc = "MII Mgmt Command register."]
pub mod mcmd;
#[doc = "MADR (rw) register accessor: MII Mgmt Address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`madr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`madr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@madr`]
module"]
pub type MADR = crate::Reg<madr::MADR_SPEC>;
#[doc = "MII Mgmt Address register."]
pub mod madr;
#[doc = "MWTD (w) register accessor: MII Mgmt Write Data register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mwtd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwtd`]
module"]
pub type MWTD = crate::Reg<mwtd::MWTD_SPEC>;
#[doc = "MII Mgmt Write Data register."]
pub mod mwtd;
#[doc = "MRDD (r) register accessor: MII Mgmt Read Data register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrdd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrdd`]
module"]
pub type MRDD = crate::Reg<mrdd::MRDD_SPEC>;
#[doc = "MII Mgmt Read Data register."]
pub mod mrdd;
#[doc = "MIND (r) register accessor: MII Mgmt Indicators register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mind::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mind`]
module"]
pub type MIND = crate::Reg<mind::MIND_SPEC>;
#[doc = "MII Mgmt Indicators register."]
pub mod mind;
#[doc = "SA0 (rw) register accessor: Station Address 0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa0`]
module"]
pub type SA0 = crate::Reg<sa0::SA0_SPEC>;
#[doc = "Station Address 0 register."]
pub mod sa0;
#[doc = "SA1 (rw) register accessor: Station Address 1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa1`]
module"]
pub type SA1 = crate::Reg<sa1::SA1_SPEC>;
#[doc = "Station Address 1 register."]
pub mod sa1;
#[doc = "SA2 (rw) register accessor: Station Address 2 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa2`]
module"]
pub type SA2 = crate::Reg<sa2::SA2_SPEC>;
#[doc = "Station Address 2 register."]
pub mod sa2;
#[doc = "COMMAND (rw) register accessor: Command register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`]
module"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Command register."]
pub mod command;
#[doc = "STATUS (r) register accessor: Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register."]
pub mod status;
#[doc = "RXDESCRIPTOR (rw) register accessor: Receive descriptor base address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdescriptor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdescriptor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdescriptor`]
module"]
pub type RXDESCRIPTOR = crate::Reg<rxdescriptor::RXDESCRIPTOR_SPEC>;
#[doc = "Receive descriptor base address register."]
pub mod rxdescriptor;
#[doc = "RXSTATUS (rw) register accessor: Receive status base address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxstatus`]
module"]
pub type RXSTATUS = crate::Reg<rxstatus::RXSTATUS_SPEC>;
#[doc = "Receive status base address register."]
pub mod rxstatus;
#[doc = "RXDESCRIPTORNUMBER (rw) register accessor: Receive number of descriptors register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdescriptornumber::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdescriptornumber::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdescriptornumber`]
module"]
pub type RXDESCRIPTORNUMBER = crate::Reg<rxdescriptornumber::RXDESCRIPTORNUMBER_SPEC>;
#[doc = "Receive number of descriptors register."]
pub mod rxdescriptornumber;
#[doc = "RXPRODUCEINDEX (r) register accessor: Receive produce index register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxproduceindex::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxproduceindex`]
module"]
pub type RXPRODUCEINDEX = crate::Reg<rxproduceindex::RXPRODUCEINDEX_SPEC>;
#[doc = "Receive produce index register."]
pub mod rxproduceindex;
#[doc = "RXCONSUMEINDEX (rw) register accessor: Receive consume index register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxconsumeindex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxconsumeindex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxconsumeindex`]
module"]
pub type RXCONSUMEINDEX = crate::Reg<rxconsumeindex::RXCONSUMEINDEX_SPEC>;
#[doc = "Receive consume index register."]
pub mod rxconsumeindex;
#[doc = "TXDESCRIPTOR (rw) register accessor: Transmit descriptor base address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdescriptor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdescriptor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdescriptor`]
module"]
pub type TXDESCRIPTOR = crate::Reg<txdescriptor::TXDESCRIPTOR_SPEC>;
#[doc = "Transmit descriptor base address register."]
pub mod txdescriptor;
#[doc = "TXSTATUS (rw) register accessor: Transmit status base address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txstatus`]
module"]
pub type TXSTATUS = crate::Reg<txstatus::TXSTATUS_SPEC>;
#[doc = "Transmit status base address register."]
pub mod txstatus;
#[doc = "TXDESCRIPTORNUMBER (rw) register accessor: Transmit number of descriptors register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdescriptornumber::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdescriptornumber::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdescriptornumber`]
module"]
pub type TXDESCRIPTORNUMBER = crate::Reg<txdescriptornumber::TXDESCRIPTORNUMBER_SPEC>;
#[doc = "Transmit number of descriptors register."]
pub mod txdescriptornumber;
#[doc = "TXPRODUCEINDEX (rw) register accessor: Transmit produce index register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txproduceindex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txproduceindex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txproduceindex`]
module"]
pub type TXPRODUCEINDEX = crate::Reg<txproduceindex::TXPRODUCEINDEX_SPEC>;
#[doc = "Transmit produce index register."]
pub mod txproduceindex;
#[doc = "TXCONSUMEINDEX (r) register accessor: Transmit consume index register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txconsumeindex::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txconsumeindex`]
module"]
pub type TXCONSUMEINDEX = crate::Reg<txconsumeindex::TXCONSUMEINDEX_SPEC>;
#[doc = "Transmit consume index register."]
pub mod txconsumeindex;
#[doc = "TSV0 (r) register accessor: Transmit status vector 0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsv0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsv0`]
module"]
pub type TSV0 = crate::Reg<tsv0::TSV0_SPEC>;
#[doc = "Transmit status vector 0 register."]
pub mod tsv0;
#[doc = "TSV1 (r) register accessor: Transmit status vector 1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsv1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsv1`]
module"]
pub type TSV1 = crate::Reg<tsv1::TSV1_SPEC>;
#[doc = "Transmit status vector 1 register."]
pub mod tsv1;
#[doc = "RSV (r) register accessor: Receive status vector register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsv`]
module"]
pub type RSV = crate::Reg<rsv::RSV_SPEC>;
#[doc = "Receive status vector register."]
pub mod rsv;
#[doc = "FLOWCONTROLCOUNTER (rw) register accessor: Flow control counter register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flowcontrolcounter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flowcontrolcounter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flowcontrolcounter`]
module"]
pub type FLOWCONTROLCOUNTER = crate::Reg<flowcontrolcounter::FLOWCONTROLCOUNTER_SPEC>;
#[doc = "Flow control counter register."]
pub mod flowcontrolcounter;
#[doc = "FLOWCONTROLSTATUS (r) register accessor: Flow control status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flowcontrolstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flowcontrolstatus`]
module"]
pub type FLOWCONTROLSTATUS = crate::Reg<flowcontrolstatus::FLOWCONTROLSTATUS_SPEC>;
#[doc = "Flow control status register."]
pub mod flowcontrolstatus;
#[doc = "RXFILTERCTRL (rw) register accessor: Receive filter control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfilterctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfilterctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfilterctrl`]
module"]
pub type RXFILTERCTRL = crate::Reg<rxfilterctrl::RXFILTERCTRL_SPEC>;
#[doc = "Receive filter control register."]
pub mod rxfilterctrl;
#[doc = "RXFILTERWOLSTATUS (r) register accessor: Receive filter WoL status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfilterwolstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfilterwolstatus`]
module"]
pub type RXFILTERWOLSTATUS = crate::Reg<rxfilterwolstatus::RXFILTERWOLSTATUS_SPEC>;
#[doc = "Receive filter WoL status register."]
pub mod rxfilterwolstatus;
#[doc = "RXFILTERWOLCLEAR (w) register accessor: Receive filter WoL clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfilterwolclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfilterwolclear`]
module"]
pub type RXFILTERWOLCLEAR = crate::Reg<rxfilterwolclear::RXFILTERWOLCLEAR_SPEC>;
#[doc = "Receive filter WoL clear register."]
pub mod rxfilterwolclear;
#[doc = "HASHFILTERL (rw) register accessor: Hash filter table LSBs register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashfilterl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashfilterl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashfilterl`]
module"]
pub type HASHFILTERL = crate::Reg<hashfilterl::HASHFILTERL_SPEC>;
#[doc = "Hash filter table LSBs register."]
pub mod hashfilterl;
#[doc = "HASHFILTERH (rw) register accessor: Hash filter table MSBs register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashfilterh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashfilterh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashfilterh`]
module"]
pub type HASHFILTERH = crate::Reg<hashfilterh::HASHFILTERH_SPEC>;
#[doc = "Hash filter table MSBs register."]
pub mod hashfilterh;
#[doc = "INTSTATUS (r) register accessor: Interrupt status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Interrupt status register."]
pub mod intstatus;
#[doc = "INTENABLE (rw) register accessor: Interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenable`]
module"]
pub type INTENABLE = crate::Reg<intenable::INTENABLE_SPEC>;
#[doc = "Interrupt enable register."]
pub mod intenable;
#[doc = "INTCLEAR (w) register accessor: Interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclear`]
module"]
pub type INTCLEAR = crate::Reg<intclear::INTCLEAR_SPEC>;
#[doc = "Interrupt clear register."]
pub mod intclear;
#[doc = "INTSET (w) register accessor: Interrupt set register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intset`]
module"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "Interrupt set register."]
pub mod intset;
#[doc = "POWERDOWN (rw) register accessor: Power-down register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`powerdown::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerdown::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@powerdown`]
module"]
pub type POWERDOWN = crate::Reg<powerdown::POWERDOWN_SPEC>;
#[doc = "Power-down register."]
pub mod powerdown;
