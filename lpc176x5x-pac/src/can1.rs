#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Controls the operating mode of the CAN Controller."]
    pub mod_: MOD,
    #[doc = "0x04 - Command bits that affect the state of the CAN Controller"]
    pub cmr: CMR,
    #[doc = "0x08 - Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1."]
    pub gsr: GSR,
    #[doc = "0x0c - Interrupt status, Arbitration Lost Capture, Error Code Capture"]
    pub icr: ICR,
    #[doc = "0x10 - Interrupt Enable"]
    pub ier: IER,
    #[doc = "0x14 - Bus Timing. Can only be written when RM in CANMOD is 1."]
    pub btr: BTR,
    #[doc = "0x18 - Error Warning Limit. Can only be written when RM in CANMOD is 1."]
    pub ewl: EWL,
    #[doc = "0x1c - Status Register"]
    pub sr: SR,
    #[doc = "0x20 - Receive frame status. Can only be written when RM in CANMOD is 1."]
    pub rfs: RFS,
    #[doc = "0x24 - Received Identifier. Can only be written when RM in CANMOD is 1."]
    pub rid: RID,
    #[doc = "0x28 - Received data bytes 1-4. Can only be written when RM in CANMOD is 1."]
    pub rda: RDA,
    #[doc = "0x2c - Received data bytes 5-8. Can only be written when RM in CANMOD is 1."]
    pub rdb: RDB,
    #[doc = "0x30 - Transmit frame info (Tx Buffer )"]
    pub tfi: crate::ArrayProxy<TFI, 3, 0x10>,
    _reserved13: [u8; 0x04],
    #[doc = "0x34 - Transmit Identifier (Tx Buffer)"]
    pub tid: crate::ArrayProxy<TID, 3, 0x10>,
    _reserved14: [u8; 0x04],
    #[doc = "0x38 - Transmit data bytes 1-4 (Tx Buffer)"]
    pub tda: crate::ArrayProxy<TDA, 3, 0x10>,
    _reserved15: [u8; 0x04],
    #[doc = "0x3c - Transmit data bytes 5-8 (Tx Buffer )"]
    pub tdb: crate::ArrayProxy<TDB, 3, 0x10>,
}
impl RegisterBlock {
    #[doc = "0x30 - Transmit frame info (Tx Buffer )"]
    #[inline(always)]
    pub fn tfi1(&self) -> &TFI {
        &self.tfi[0]
    }
    #[doc = "0x34 - Transmit frame info (Tx Buffer )"]
    #[inline(always)]
    pub fn tfi2(&self) -> &TFI {
        &self.tfi[1]
    }
    #[doc = "0x38 - Transmit frame info (Tx Buffer )"]
    #[inline(always)]
    pub fn tfi3(&self) -> &TFI {
        &self.tfi[2]
    }
    #[doc = "0x34 - Transmit Identifier (Tx Buffer)"]
    #[inline(always)]
    pub fn tid1(&self) -> &TID {
        &self.tid[0]
    }
    #[doc = "0x38 - Transmit Identifier (Tx Buffer)"]
    #[inline(always)]
    pub fn tid2(&self) -> &TID {
        &self.tid[1]
    }
    #[doc = "0x3c - Transmit Identifier (Tx Buffer)"]
    #[inline(always)]
    pub fn tid3(&self) -> &TID {
        &self.tid[2]
    }
    #[doc = "0x38 - Transmit data bytes 1-4 (Tx Buffer)"]
    #[inline(always)]
    pub fn tda1(&self) -> &TDA {
        &self.tda[0]
    }
    #[doc = "0x3c - Transmit data bytes 1-4 (Tx Buffer)"]
    #[inline(always)]
    pub fn tda2(&self) -> &TDA {
        &self.tda[1]
    }
    #[doc = "0x40 - Transmit data bytes 1-4 (Tx Buffer)"]
    #[inline(always)]
    pub fn tda3(&self) -> &TDA {
        &self.tda[2]
    }
    #[doc = "0x3c - Transmit data bytes 5-8 (Tx Buffer )"]
    #[inline(always)]
    pub fn tdb1(&self) -> &TDB {
        &self.tdb[0]
    }
    #[doc = "0x40 - Transmit data bytes 5-8 (Tx Buffer )"]
    #[inline(always)]
    pub fn tdb2(&self) -> &TDB {
        &self.tdb[1]
    }
    #[doc = "0x44 - Transmit data bytes 5-8 (Tx Buffer )"]
    #[inline(always)]
    pub fn tdb3(&self) -> &TDB {
        &self.tdb[2]
    }
}
#[doc = "MOD (rw) register accessor: Controls the operating mode of the CAN Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mod_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mod_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mod_`]
module"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Controls the operating mode of the CAN Controller."]
pub mod mod_;
#[doc = "CMR (w) register accessor: Command bits that affect the state of the CAN Controller\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr`]
module"]
pub type CMR = crate::Reg<cmr::CMR_SPEC>;
#[doc = "Command bits that affect the state of the CAN Controller"]
pub mod cmr;
#[doc = "GSR (r) register accessor: Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsr`]
module"]
pub type GSR = crate::Reg<gsr::GSR_SPEC>;
#[doc = "Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1."]
pub mod gsr;
#[doc = "ICR (r) register accessor: Interrupt status, Arbitration Lost Capture, Error Code Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt status, Arbitration Lost Capture, Error Code Capture"]
pub mod icr;
#[doc = "IER (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "BTR (rw) register accessor: Bus Timing. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr`]
module"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "Bus Timing. Can only be written when RM in CANMOD is 1."]
pub mod btr;
#[doc = "EWL (rw) register accessor: Error Warning Limit. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ewl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ewl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ewl`]
module"]
pub type EWL = crate::Reg<ewl::EWL_SPEC>;
#[doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1."]
pub mod ewl;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "RFS (rw) register accessor: Receive frame status. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfs`]
module"]
pub type RFS = crate::Reg<rfs::RFS_SPEC>;
#[doc = "Receive frame status. Can only be written when RM in CANMOD is 1."]
pub mod rfs;
#[doc = "RID (rw) register accessor: Received Identifier. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rid`]
module"]
pub type RID = crate::Reg<rid::RID_SPEC>;
#[doc = "Received Identifier. Can only be written when RM in CANMOD is 1."]
pub mod rid;
#[doc = "RDA (rw) register accessor: Received data bytes 1-4. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rda`]
module"]
pub type RDA = crate::Reg<rda::RDA_SPEC>;
#[doc = "Received data bytes 1-4. Can only be written when RM in CANMOD is 1."]
pub mod rda;
#[doc = "RDB (rw) register accessor: Received data bytes 5-8. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdb`]
module"]
pub type RDB = crate::Reg<rdb::RDB_SPEC>;
#[doc = "Received data bytes 5-8. Can only be written when RM in CANMOD is 1."]
pub mod rdb;
#[doc = "TFI (rw) register accessor: Transmit frame info (Tx Buffer )\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfi`]
module"]
pub type TFI = crate::Reg<tfi::TFI_SPEC>;
#[doc = "Transmit frame info (Tx Buffer )"]
pub mod tfi;
#[doc = "TID (rw) register accessor: Transmit Identifier (Tx Buffer)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tid`]
module"]
pub type TID = crate::Reg<tid::TID_SPEC>;
#[doc = "Transmit Identifier (Tx Buffer)"]
pub mod tid;
#[doc = "TDA (rw) register accessor: Transmit data bytes 1-4 (Tx Buffer)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tda`]
module"]
pub type TDA = crate::Reg<tda::TDA_SPEC>;
#[doc = "Transmit data bytes 1-4 (Tx Buffer)"]
pub mod tda;
#[doc = "TDB (rw) register accessor: Transmit data bytes 5-8 (Tx Buffer )\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdb`]
module"]
pub type TDB = crate::Reg<tdb::TDB_SPEC>;
#[doc = "Transmit data bytes 5-8 (Tx Buffer )"]
pub mod tdb;
