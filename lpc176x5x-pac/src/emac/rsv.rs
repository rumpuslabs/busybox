#[doc = "Register `RSV` reader"]
pub type R = crate::R<RSV_SPEC>;
#[doc = "Field `RBC` reader - Received byte count. Indicates length of received frame."]
pub type RBC_R = crate::FieldReader<u16>;
#[doc = "Field `PPI` reader - Packet previously ignored. Indicates that a packet was dropped."]
pub type PPI_R = crate::BitReader;
#[doc = "Field `RXDVSEEN` reader - RXDV event previously seen. Indicates that the last receive event seen was not long enough to be a valid packet."]
pub type RXDVSEEN_R = crate::BitReader;
#[doc = "Field `CESEEN` reader - Carrier event previously seen. Indicates that at some time since the last receive statistics, a carrier event was detected."]
pub type CESEEN_R = crate::BitReader;
#[doc = "Field `RCV` reader - Receive code violation. Indicates that received PHY data does not represent a valid receive code."]
pub type RCV_R = crate::BitReader;
#[doc = "Field `CRCERR` reader - CRC error. The attached CRC in the packet did not match the internally generated CRC."]
pub type CRCERR_R = crate::BitReader;
#[doc = "Field `LCERR` reader - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
pub type LCERR_R = crate::BitReader;
#[doc = "Field `LOR` reader - Length out of range. Indicates that frame type/length field was larger than 1518 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
pub type LOR_R = crate::BitReader;
#[doc = "Field `ROK` reader - Receive OK. The packet had valid CRC and no symbol errors."]
pub type ROK_R = crate::BitReader;
#[doc = "Field `MULTICAST` reader - The packet destination was a multicast address."]
pub type MULTICAST_R = crate::BitReader;
#[doc = "Field `BROADCAST` reader - The packet destination was a broadcast address."]
pub type BROADCAST_R = crate::BitReader;
#[doc = "Field `DRIBBLENIBBLE` reader - Indicates that after the end of packet another 1-7 bits were received. A single nibble, called dribble nibble, is formed but not sent out."]
pub type DRIBBLENIBBLE_R = crate::BitReader;
#[doc = "Field `CONTROLFRAME` reader - The frame was a control frame."]
pub type CONTROLFRAME_R = crate::BitReader;
#[doc = "Field `PAUSE` reader - The frame was a control frame with a valid PAUSE opcode."]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `UO` reader - Unsupported Opcode. The current frame was recognized as a Control Frame but contains an unknown opcode."]
pub type UO_R = crate::BitReader;
#[doc = "Field `VLAN` reader - Frame's length/type field contained 0x8100 which is the VLAN protocol identifier."]
pub type VLAN_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Received byte count. Indicates length of received frame."]
    #[inline(always)]
    pub fn rbc(&self) -> RBC_R {
        RBC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Packet previously ignored. Indicates that a packet was dropped."]
    #[inline(always)]
    pub fn ppi(&self) -> PPI_R {
        PPI_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RXDV event previously seen. Indicates that the last receive event seen was not long enough to be a valid packet."]
    #[inline(always)]
    pub fn rxdvseen(&self) -> RXDVSEEN_R {
        RXDVSEEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Carrier event previously seen. Indicates that at some time since the last receive statistics, a carrier event was detected."]
    #[inline(always)]
    pub fn ceseen(&self) -> CESEEN_R {
        CESEEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive code violation. Indicates that received PHY data does not represent a valid receive code."]
    #[inline(always)]
    pub fn rcv(&self) -> RCV_R {
        RCV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CRC error. The attached CRC in the packet did not match the internally generated CRC."]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
    #[inline(always)]
    pub fn lcerr(&self) -> LCERR_R {
        LCERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Length out of range. Indicates that frame type/length field was larger than 1518 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
    #[inline(always)]
    pub fn lor(&self) -> LOR_R {
        LOR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Receive OK. The packet had valid CRC and no symbol errors."]
    #[inline(always)]
    pub fn rok(&self) -> ROK_R {
        ROK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The packet destination was a multicast address."]
    #[inline(always)]
    pub fn multicast(&self) -> MULTICAST_R {
        MULTICAST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The packet destination was a broadcast address."]
    #[inline(always)]
    pub fn broadcast(&self) -> BROADCAST_R {
        BROADCAST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Indicates that after the end of packet another 1-7 bits were received. A single nibble, called dribble nibble, is formed but not sent out."]
    #[inline(always)]
    pub fn dribblenibble(&self) -> DRIBBLENIBBLE_R {
        DRIBBLENIBBLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The frame was a control frame."]
    #[inline(always)]
    pub fn controlframe(&self) -> CONTROLFRAME_R {
        CONTROLFRAME_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The frame was a control frame with a valid PAUSE opcode."]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Unsupported Opcode. The current frame was recognized as a Control Frame but contains an unknown opcode."]
    #[inline(always)]
    pub fn uo(&self) -> UO_R {
        UO_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Frame's length/type field contained 0x8100 which is the VLAN protocol identifier."]
    #[inline(always)]
    pub fn vlan(&self) -> VLAN_R {
        VLAN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSV")
            .field("rbc", &format_args!("{}", self.rbc().bits()))
            .field("ppi", &format_args!("{}", self.ppi().bit()))
            .field("rxdvseen", &format_args!("{}", self.rxdvseen().bit()))
            .field("ceseen", &format_args!("{}", self.ceseen().bit()))
            .field("rcv", &format_args!("{}", self.rcv().bit()))
            .field("crcerr", &format_args!("{}", self.crcerr().bit()))
            .field("lcerr", &format_args!("{}", self.lcerr().bit()))
            .field("lor", &format_args!("{}", self.lor().bit()))
            .field("rok", &format_args!("{}", self.rok().bit()))
            .field("multicast", &format_args!("{}", self.multicast().bit()))
            .field("broadcast", &format_args!("{}", self.broadcast().bit()))
            .field(
                "dribblenibble",
                &format_args!("{}", self.dribblenibble().bit()),
            )
            .field(
                "controlframe",
                &format_args!("{}", self.controlframe().bit()),
            )
            .field("pause", &format_args!("{}", self.pause().bit()))
            .field("uo", &format_args!("{}", self.uo().bit()))
            .field("vlan", &format_args!("{}", self.vlan().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RSV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Receive status vector register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSV_SPEC;
impl crate::RegisterSpec for RSV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsv::R`](R) reader structure"]
impl crate::Readable for RSV_SPEC {}
#[doc = "`reset()` method sets RSV to value 0"]
impl crate::Resettable for RSV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
