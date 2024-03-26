#[doc = "Register `TSV0` reader"]
pub type R = crate::R<TSV0_SPEC>;
#[doc = "Field `CRCERR` reader - CRC error. The attached CRC in the packet did not match the internally generated CRC."]
pub type CRCERR_R = crate::BitReader;
#[doc = "Field `LCE` reader - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
pub type LCE_R = crate::BitReader;
#[doc = "Field `LOR` reader - Length out of range. Indicates that frame type/length field was larger than 1500 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
pub type LOR_R = crate::BitReader;
#[doc = "Field `DONE` reader - Transmission of packet was completed."]
pub type DONE_R = crate::BitReader;
#[doc = "Field `MULTICAST` reader - Packet's destination was a multicast address."]
pub type MULTICAST_R = crate::BitReader;
#[doc = "Field `BROADCAST` reader - Packet's destination was a broadcast address."]
pub type BROADCAST_R = crate::BitReader;
#[doc = "Field `PACKETDEFER` reader - Packet was deferred for at least one attempt, but less than an excessive defer."]
pub type PACKETDEFER_R = crate::BitReader;
#[doc = "Field `EXDF` reader - Excessive Defer. Packet was deferred in excess of 6071 nibble times in 100 Mbps or 24287 bit times in 10 Mbps mode."]
pub type EXDF_R = crate::BitReader;
#[doc = "Field `EXCOL` reader - Excessive Collision. Packet was aborted due to exceeding of maximum allowed number of collisions."]
pub type EXCOL_R = crate::BitReader;
#[doc = "Field `LCOL` reader - Late Collision. Collision occurred beyond collision window, 512 bit times."]
pub type LCOL_R = crate::BitReader;
#[doc = "Field `GIANT` reader - Byte count in frame was greater than can be represented in the transmit byte count field in TSV1."]
pub type GIANT_R = crate::BitReader;
#[doc = "Field `UNDERRUN` reader - Host side caused buffer underrun."]
pub type UNDERRUN_R = crate::BitReader;
#[doc = "Field `TOTALBYTES` reader - The total number of bytes transferred including collided attempts."]
pub type TOTALBYTES_R = crate::FieldReader<u16>;
#[doc = "Field `CONTROLFRAME` reader - The frame was a control frame."]
pub type CONTROLFRAME_R = crate::BitReader;
#[doc = "Field `PAUSE` reader - The frame was a control frame with a valid PAUSE opcode."]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `BACKPRESSURE` reader - Carrier-sense method backpressure was previously applied."]
pub type BACKPRESSURE_R = crate::BitReader;
#[doc = "Field `VLAN` reader - Frame's length/type field contained 0x8100 which is the VLAN protocol identifier."]
pub type VLAN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CRC error. The attached CRC in the packet did not match the internally generated CRC."]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
    #[inline(always)]
    pub fn lce(&self) -> LCE_R {
        LCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Length out of range. Indicates that frame type/length field was larger than 1500 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
    #[inline(always)]
    pub fn lor(&self) -> LOR_R {
        LOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission of packet was completed."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Packet's destination was a multicast address."]
    #[inline(always)]
    pub fn multicast(&self) -> MULTICAST_R {
        MULTICAST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Packet's destination was a broadcast address."]
    #[inline(always)]
    pub fn broadcast(&self) -> BROADCAST_R {
        BROADCAST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Packet was deferred for at least one attempt, but less than an excessive defer."]
    #[inline(always)]
    pub fn packetdefer(&self) -> PACKETDEFER_R {
        PACKETDEFER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Excessive Defer. Packet was deferred in excess of 6071 nibble times in 100 Mbps or 24287 bit times in 10 Mbps mode."]
    #[inline(always)]
    pub fn exdf(&self) -> EXDF_R {
        EXDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Excessive Collision. Packet was aborted due to exceeding of maximum allowed number of collisions."]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Late Collision. Collision occurred beyond collision window, 512 bit times."]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Byte count in frame was greater than can be represented in the transmit byte count field in TSV1."]
    #[inline(always)]
    pub fn giant(&self) -> GIANT_R {
        GIANT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Host side caused buffer underrun."]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:27 - The total number of bytes transferred including collided attempts."]
    #[inline(always)]
    pub fn totalbytes(&self) -> TOTALBYTES_R {
        TOTALBYTES_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28 - The frame was a control frame."]
    #[inline(always)]
    pub fn controlframe(&self) -> CONTROLFRAME_R {
        CONTROLFRAME_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The frame was a control frame with a valid PAUSE opcode."]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Carrier-sense method backpressure was previously applied."]
    #[inline(always)]
    pub fn backpressure(&self) -> BACKPRESSURE_R {
        BACKPRESSURE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Frame's length/type field contained 0x8100 which is the VLAN protocol identifier."]
    #[inline(always)]
    pub fn vlan(&self) -> VLAN_R {
        VLAN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSV0")
            .field("crcerr", &format_args!("{}", self.crcerr().bit()))
            .field("lce", &format_args!("{}", self.lce().bit()))
            .field("lor", &format_args!("{}", self.lor().bit()))
            .field("done", &format_args!("{}", self.done().bit()))
            .field("multicast", &format_args!("{}", self.multicast().bit()))
            .field("broadcast", &format_args!("{}", self.broadcast().bit()))
            .field("packetdefer", &format_args!("{}", self.packetdefer().bit()))
            .field("exdf", &format_args!("{}", self.exdf().bit()))
            .field("excol", &format_args!("{}", self.excol().bit()))
            .field("lcol", &format_args!("{}", self.lcol().bit()))
            .field("giant", &format_args!("{}", self.giant().bit()))
            .field("underrun", &format_args!("{}", self.underrun().bit()))
            .field("totalbytes", &format_args!("{}", self.totalbytes().bits()))
            .field(
                "controlframe",
                &format_args!("{}", self.controlframe().bit()),
            )
            .field("pause", &format_args!("{}", self.pause().bit()))
            .field(
                "backpressure",
                &format_args!("{}", self.backpressure().bit()),
            )
            .field("vlan", &format_args!("{}", self.vlan().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TSV0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transmit status vector 0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsv0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSV0_SPEC;
impl crate::RegisterSpec for TSV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsv0::R`](R) reader structure"]
impl crate::Readable for TSV0_SPEC {}
#[doc = "`reset()` method sets TSV0 to value 0"]
impl crate::Resettable for TSV0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
